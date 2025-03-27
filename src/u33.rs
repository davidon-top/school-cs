pub fn main() {
	println!("Zadaj cislo a znacku (F/C)");
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	let mut input = input.trim().to_string();
	let unit = input.remove(input.len() - 1);
	let num: f64 = input.parse().expect("Neda sa retazec premenit na cislo");
	match unit { 
		'C' | 'c' => {
			println!("{}F", num * 1.8 + 32.0)
		}
		'F' | 'f' => {
			println!("{}C", (num - 32.0)/1.8)
		}
		_ => {
			println!("nepodporovana znacka")
		}
	}
}
