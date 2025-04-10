pub fn main() {
    println!("Zaklad");
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).unwrap();
    let inp: f64 = inp.trim().parse().unwrap();

    println!("mocnitel: ");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: f64 = n.trim().parse().unwrap();

    println!("vysledok: {}", inp.powf(n))
}
