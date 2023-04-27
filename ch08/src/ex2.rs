pub fn main() {
    println!("pig latin: {}", to_pig_latin(&"Hello World!".to_string()));
    println!("pig latin: {}", to_pig_latin(&"".to_string()));
}

fn to_pig_latin(string: &String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut pig_latin = string
        .split_whitespace()
        .fold(String::new(), |mut acc, word| {
            let mut chars = word.chars();
            let first = chars.next().unwrap();
            let new_word = if vowels.contains(&first.to_ascii_lowercase()) {
                format!("{word}-hay ")
            } else {
                format!("{}-{first}ay ", chars.as_str())
            };
            acc.push_str(&new_word);
            acc
        });

    pig_latin.pop();
    pig_latin
}
