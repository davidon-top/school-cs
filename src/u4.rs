/*
Vytvorte podprogram palindrom, ktorý zistí, či je slovo zadané na vstupe palindróm. Slovo zadávame malými písmenami, nepoužívame medzery a diakritiku.

Poznámka: Palindróm je slovo, veta, číslo (všeobecne akákoľvek postupnosť symbolov), ktoré má tú vlastnosť, že ju možno čítať v ľubovoľnom smere (sprava doľava alebo zľava doprava) a má vždy rovnaký význam.
*/

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    println!("{}", palindrome(input.trim().to_string()));
}

fn palindrome(s: String) -> bool {
    s.chars().zip(s.chars().rev()).all(|(a, b)| a == b)
}
