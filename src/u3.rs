pub fn main() {
    let mut input = String::new();
    println!("Zadajte prirodzene cislo a zaciatok intervalu: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let a: u128 = input
        .trim()
        .parse()
        .expect("Ocakavali sme prirodzene cislo");
    input.clear();
    println!("Zadajte prirodzene cislo b koniec intervalu: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let b: u128 = input
        .trim()
        .parse()
        .expect("ocakavali sme prirodzene cislo");
    input.clear();
    
    println!("Cisla delitelne tromi v intervale od {} do {}", a, b);

    devidable(a, b);
}

fn devidable(a: u128, b: u128) {
    for i in a..b + 1 {
        let digit_sum: u128 = i
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u128)
            .sum();
        if digit_sum % 3 == 0 {
            println!("{}", i);
        }
    }
}
