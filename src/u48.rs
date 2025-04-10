pub fn main() {
    let ziaci = [
        "Horák Marek",
        "Gajdáč Tibor",
        "Velická Barbora",
        "Malík Peter",
        "Malíková Diana",
    ];

    let hodiny = [15, 0, 0, 75, 34];

    println!("Vsetky vymeskane hodiniy: {}", hodiny.iter().sum::<i32>());
    println!(
        "Priemerny pocet vymeskanych hodin: {}",
        hodiny.iter().sum::<i32>() as f64 / hodiny.len() as f64
    );
    let najhorsi = ziaci.iter().zip(hodiny).max_by(|z, e| z.1.cmp(&e.1));
    if let Some(najhorsi) = najhorsi {
        println!(
            "Najhorsiu dochadzku ma: {} s dochadzkou {}",
            najhorsi.0, najhorsi.1
        )
    } else {
        println!("Nepodarilo sa najst najhorsieho")
    }
    let s0 = ziaci
        .iter()
        .zip(hodiny)
        .filter(|e| e.1 == 0)
        .collect::<Vec<_>>();
    println!("pocet ziakou ktori mozu mat pochvalu: {}", s0.len());
    println!("Ziaci na pochvalu");
    s0.iter().for_each(|e| println!("  {}", e.0));
}
