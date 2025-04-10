pub fn main() {
	rec(7)
}

fn rec(num: i32) {
	if num >= 1000 {return;}
	println!("num: {num}");
	rec(num + 7)
}
