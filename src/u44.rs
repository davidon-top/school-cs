pub fn main() {
    println!("hmotnost: (v kg)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let hmotnost: f32 = input.trim().parse().unwrap();
    input.clear();
    println!("viska (v m)");
    std::io::stdin().read_line(&mut input).unwrap();
    let viska: f32 = input.trim().parse().unwrap();
    let bmi = hmotnost / viska.powi(2);
    println!("bmi: {bmi}")
}
