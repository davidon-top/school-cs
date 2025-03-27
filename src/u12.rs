pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    println!("str len {}", input.len());
    let tituly = format!("PaedDr. {} PhDr.", input);
    println!("s titulmi: {}", tituly);
    match tituly.find("PhDr.") {
        Some(pos) => println!("phdr pozicia {}", pos),
        None => println!("nenasiel titul"),
    };
}
