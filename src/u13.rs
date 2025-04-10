pub fn main() {
    println!("1. mtokm");
    println!("2. kmtom");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1." | "1" | "mtokm" => read_and_convert(|u| u * 1.609344),
        "2." | "2" | "kmtom" => read_and_convert(|u| u / 1.609344),
        _ => {}
    }
}

fn read_and_convert<F>(tom: F)
where
    F: Fn(f64) -> f64,
{
    println!("zadaj udaj");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let unit: f64 = input.trim().parse().unwrap();
    println!("{}", tom(unit));
}
