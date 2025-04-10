pub fn main() {
    let mut sutaziaci = vec![("aoeu", 21), ("htns", 25), ("pyfg", 16)];
    sutaziaci.sort_by(|s, l| s.1.cmp(&l.1));
    sutaziaci.iter().for_each(|n| {
        println!("{} {}", n.0, n.1);
    })
}
