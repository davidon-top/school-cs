static SAM: &str = "aeiyouáéíóúý";

pub fn main() {
	let mut input = String::new();
	println!("Zadaj vetu");
	std::io::stdin().read_line(&mut input).unwrap();
	let mut input = input.trim().to_string();
	
	for c in SAM.chars() {
		input = input.replace(c, "_");
	}
	
	println!("{input}");
}
