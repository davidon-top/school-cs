use std::fs::OpenOptions;
use std::io::Write;

pub fn main() {
    let mut input = String::new();
    println!("Zadaj diktat");
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();
    let povodny_pocet = input.clone().chars().filter(|c| *c == '_').count();
    let diktat = input.replace(['i', 'í', 'y', 'ý'], "_");
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("./doplnovacka.txt")
        .unwrap();
    file.write_all(diktat.clone().as_bytes()).unwrap();
    let max = diktat.chars().filter(|c| *c == '_').count() - povodny_pocet;
    println!("Max pocet chyb: {}", max);
}
