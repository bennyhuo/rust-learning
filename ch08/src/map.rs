use std::collections::HashMap;

pub fn main() {
    let mut scores = HashMap::new();
    let key = String::from("Yellow");
    scores.insert(key, 50);
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let value = scores.get(&"Red".to_string());
    let value2 = scores[&"Red".to_string()];

    println!("{value2}");

    for (k, v) in scores.iter() {
        println!("{k}, {v}");
    }

    scores.insert("Red".to_string(), 100);
    scores
        .entry("Red".to_string())
        .and_modify(|e| *e = *e * 1000);

    let value_of_red = scores.get_mut("Red");
    match value_of_red {
        Some(v) => *v = *v + 10000,
        None => println!("None"),
    }
    for (k, v) in scores.iter() {
        println!("{k}, {v}");
    }
}
