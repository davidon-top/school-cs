pub fn main() {
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).unwrap();
    let input1 = input1.trim().parse().unwrap();
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).unwrap();
    let input2 = input2.trim().parse().unwrap();

    let res = max(input1, input2);

    if let Some(res) = res {
        println!("{res}")
    } else {
        println!("rovnake")
    }
}

pub fn max(i1: i32, i2: i32) -> Option<i32> {
    let res = (i1.to_string().len(), i2.to_string().len());
    if res.0 == res.1 {
        None
    } else if res.0 > res.1 {
        Some(i1)
    } else {
        Some(i2)
    }
}
