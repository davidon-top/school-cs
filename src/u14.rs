use rand::Rng;

pub fn main() {
    let mut teploty = Vec::with_capacity(20);
    let mut rand = rand::thread_rng();
    for _ in 0..20 {
        teploty.push(rand.gen::<f64>() * 40.0 - 20.0);
    }
    println!("teploty: {:?}", teploty);
    println!(
        "najnizsia {}",
        teploty.iter().map(|e| e.clone()).reduce(f64::min).unwrap()
    );
    println!(
        "najvizsia {}",
        teploty.iter().map(|e| e.clone()).reduce(f64::max).unwrap()
    );
    println!("priemer {}", teploty.iter().sum::<f64>() / 20.0);
    println!(
        "pocet nulovych teplot: {}",
        teploty.iter().find(|e| **e == 0.0).iter().count()
    )
}
