pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    println!("Zadaj cislo a zaciatok intervalu: ");
    let a: i128 = input.trim().parse().unwrap();
    input.clear();
    println!("Zadaj cislo b koniec intervalu: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let b: i128 = input.trim().parse().unwrap();
    input.clear();
    println!("Zadaj operaciu (p, parne, n, neparne): ");
    std::io::stdin().read_line(&mut input).unwrap();
    let op = input.trim();

    match op {
        "p" | "parne" => {
            let a = if a % 2 == 0 { a } else { a + 1 };
            for i in (a..=b).step_by(2) {
                print!("{} ", i);
            }
        }
        "n" | "neparne" => {
            let a = if a % 2 == 1 { a } else { a + 1 };
            for i in (a..=b).step_by(2) {
                print!("{} ", i);
            }
        }
        _ => panic!("Invalid operation"),
    }
}
