use std::fs::OpenOptions;
use std::io::Write;

pub fn main() {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("./cisla.txt")
        .unwrap();
    let mut cisla = Vec::with_capacity(21);
    for i in 0..20 {
        let mut input = String::new();
        println!("Zadaj cislo {} z 20", i + 1);
        std::io::stdin().read_line(&mut input).unwrap();
        let inp: f64 = input.trim().parse().unwrap();
        cisla.push(inp);
    }
    let avg = cisla.iter().sum::<f64>() / 20f64;
    println!("priemer: {avg}");
    cisla.push(avg);
    file.write_all(
        cisla
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )
    .unwrap()
}
