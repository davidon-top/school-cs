pub fn main() {
    println!("pocet ziakov: ");
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).unwrap();
    let num: usize = num.trim().parse().unwrap();

    let mut ziaci = Vec::with_capacity(num);
    for i in 0..num {
        println!("Zadaj ziaka {i}, vo formate: meno priezvisko, trieda, znamka znamka znamka znamka znamka");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input
            .trim()
            .split(",")
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();
        ziaci.push((
            input[0].clone(),
            input[1].clone(),
            input[2]
                .clone()
                .split(" ")
                .map(|z| z.parse::<i32>().unwrap())
                .collect::<Vec<_>>(),
        ));
    }
    let ziaci = ziaci
        .iter()
        .map(|z| (z, z.2.iter().sum::<i32>() as f64 / 5.0))
        .collect::<Vec<_>>()
        .sort_by(|z, e| z.1.partial_cmp(&e.1).unwrap());
}
