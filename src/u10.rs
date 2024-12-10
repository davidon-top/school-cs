pub fn main() {
    let mut v = Vec::with_capacity(20);
    for _ in 0..20 {
        v.push(rand::random::<f32>() * 40.0 - 20.0);
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    println!("{:?}", v);
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: f32 = input.trim().parse().unwrap();
    println!("{}", v.iter().position(|x| *x == input).unwrap_or(usize::MAX))
}
