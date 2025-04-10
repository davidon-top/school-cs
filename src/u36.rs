use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub fn main() {
    let mut input1 = String::new();
    println!("Zadaj 1 retazec DNA");
    std::io::stdin().read_line(&mut input1).unwrap();
    let input1 = input1.trim().to_uppercase().to_string();
    let mut input2 = String::new();
    println!("Zadaj 2 complementarny retazec DNA");
    std::io::stdin().read_line(&mut input2).unwrap();
    let input2 = input2.trim().to_uppercase().to_string();

    let n_chyb = pocet_chyb(input1.clone(), input2.clone()).unwrap();
    println!("Pocet chyb: {n_chyb}");
    println!("Percento chyb: {}", percento_chyb(input1.clone(), input2));
    println!("Complementarny retazec: {}", generuj_retazec(input1));
}

#[derive(Default, Debug)]
struct LengthError(String);

impl Display for LengthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl Error for LengthError {}

fn pocet_chyb(i1: String, i2: String) -> Result<usize, LengthError> {
    if i1.len() != i2.len() {
        return Err(LengthError(
            "Dna a complementarne dna niesu rovnako dlhe".to_string(),
        ));
    }
    let mut n: usize = 0;
    i1.chars().zip(i2.chars()).for_each(|cp| match cp {
        ('A', 'T') | ('T', 'A') | ('C', 'G') | ('G', 'C') => (),
        _ => n += 1,
    });
    Ok(n)
}

fn percento_chyb(i1: String, i2: String) -> f64 {
    let n_chyb = pocet_chyb(i1.clone(), i2).unwrap();
    (n_chyb as f64 / i1.len() as f64) * 100f64
}

fn generuj_retazec(i1: String) -> String {
    i1.chars()
        .map(|c| match c {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => ' ',
        })
        .collect()
}
