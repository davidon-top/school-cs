static SAM: &str = "aeiyou";

pub fn main() {
	let mut input = String::new();
	println!("Zadaj slovo");
	std::io::stdin().read_line(&mut input).unwrap();
	let mut input = input.trim().to_string();
	if SAM.contains(input.chars().next().expect("Slovo musi mat aspon 1 znak")) {
		println!("{input}way")
	} else {
		let c = input.chars().next().unwrap();
		input.replace_range(0..1, "");
		println!("{input}{c}ay")
	}
}
