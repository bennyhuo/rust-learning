fn main() {
    let mut a = [1, 2, 3, 4];

    let r_a = &a[..2];

    let r_b = &mut a;
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word2(s: &String) -> usize {
    for (i, item) in s.chars().enumerate() {
        if item == ' ' {
            return i;
        }
    }
    s.len()
}
