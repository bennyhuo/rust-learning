fn main() {
    println!("pig latin: {}", to_pig_latin(&"Hello World!".to_string()));
    println!("pig latin: {}", to_pig_latin(&"".to_string()));
}

fn to_pig_latin(string: &String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut pig_latin = String::new();

    for word in string.split_whitespace() {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();

        if vowels.contains(&first_char) {
            pig_latin.push_str(word);
            pig_latin.push_str("-hay");
        } else {
            pig_latin.push_str(chars.as_str());
            pig_latin.push('-');
            pig_latin.push(first_char);
            pig_latin.push_str("ay");
        }
        pig_latin.push(' ')
    }

    pig_latin.pop();
    pig_latin
}
