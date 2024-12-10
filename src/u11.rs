pub fn main() {
    let mut input = String::new();
    println!("zadajte meno");
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if !input.chars().all(|c| c.is_alphabetic()) {
        println!("Nespravny vstup, zadajte len pismena");
        panic!("Invalid input");
    }
    let mut input = input.to_lowercase();
    input = input.chars().enumerate().map(|(i, c)| if i == 0 { c.to_uppercase().next().unwrap() } else { c }).collect();
    println!("{}", input);
}
