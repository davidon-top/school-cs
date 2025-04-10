pub fn main() {
    println!("Prve cislo: ");
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).unwrap();
    let num: f64 = num.trim().parse().unwrap();

    println!("operator: ");
    let mut operand = String::new();
    std::io::stdin().read_line(&mut operand).unwrap();
    let operand = operand.trim().to_string();

    println!("Druhe cislo: ");
    let mut num1 = String::new();
    std::io::stdin().read_line(&mut num1).unwrap();
    let num1: f64 = num1.trim().parse().unwrap();

    let operator = match operand.as_str() {
        "+" => |c1: f64, c2: f64| c1 + c2,
        "-" => |c1: f64, c2: f64| c1 - c2,
        "*" => |c1: f64, c2: f64| c1 * c2,
        "/" => |c1: f64, c2: f64| c1 / c2,
        _ => {
            panic!("Zly operator")
        }
    };

    let result = operator(num, num1);
    println!("Vysledok {result}");
}
