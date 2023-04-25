pub fn main() {
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();
    let s2 = "initial contents".to_string();

    let s3 = String::from("Hello World");

    let mut s4 = s + "; " + s2.as_str();
    println!("{s4}, {s2}");

    s4.push('x');
    s4.push('中');
    let s5 = "Hello";
    s4.push_str(s5);
    s4.push_str("国，你好！");

    println!("{s4}, {s5}");

    let s6 = format!("{s2}: {s5}");
    println!("{}", s6.len());

    let mut s7 = s4 + "; " + &s2;
    let c = s7.chars();
    let h = &s7[0..4];
}
