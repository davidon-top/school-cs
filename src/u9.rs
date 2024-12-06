pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    //i, í, y, ý, I, Í, Y, Ý
    let input = input
        .trim()
        .replace("i", "_")
        .replace("í", "_")
        .replace("y", "_")
        .replace("ý", "_")
        .replace("I", "_")
        .replace("Í", "_")
        .replace("Y", "_")
        .replace("Ý", "_");
    println!("{}", input);
}
