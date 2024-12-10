use std::io::Write;
use std::ops::{Add, Sub};

pub fn main() {
    let mut input = String::new();
    println!("Zadaj cislo a: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let a: i128 = input.trim().parse().unwrap();
    input.clear();
    println!("Zadaj cislo b: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let b: i128 = input.trim().parse().unwrap();
    input.clear();
    println!("Zadaj operaciu (+, -, priemer): ");
    std::io::stdin().read_line(&mut input).unwrap();
    let op = input.trim().clone();

    let fun = match op {
        "+" | "sucet" | "add" | "sum" => i128::add,
        "-" | "rozdiel" | "difference" | "subtract" => i128::sub,
        "priemer" | "avg" | "average" => |a, b| (a + b) / 2,
        _ => panic!("Nepodporovana operacia"),
    };

    let out = fun(a, b);
    println!("{}", out);

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("output.txt")
        .unwrap();
    file.write_all(out.to_string().as_bytes()).unwrap();
}
