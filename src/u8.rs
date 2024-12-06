pub fn main() {
    let mut a1 = 0;
    let mut a2 = 0;
    let mut a3 = 0;
    let mut a4 = 0;
    let mut a5 = 0;

    for _ in 0..30 {
        let randnum = rand::random::<f32>();
        let randnum = randnum * 5.0;
        let randnum = randnum as i32;

        match randnum {
            0 => a1 += 1,
            1 => a2 += 1,
            2 => a3 += 1,
            3 => a4 += 1,
            4 => a5 += 1,
            _ => panic!("Invalid number"),
        }
    }

    println!("a1: {}, a2: {}, a3: {}, a4: {}, a5: {}", a1, a2, a3, a4, a5);
}
