/*
Vytvorte podprogram, ktorý uloží do nového reťazca znaky v opačnom poradí,
ako sú v zadanom reťazci. Použite ho v programe, ktorý vyzve užívateľa na
zadanie ľubovoľného reťazca a vypíše ho na obrazovku obrátene.
 */

pub fn main() {
    println!("Zadaj retazec");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    println!("opacny retazec: {}", opacne_poradie(&input));
    input.clear();

}

fn opacne_poradie(input: &String) -> String {
    input.chars().into_iter().rev().collect()
}
