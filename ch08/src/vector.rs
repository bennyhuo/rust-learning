pub fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    let v3 = vec!["Hello", "World"];

    v.push(5);
    v.append(&mut v2);

    v.pop();

    for e in &mut v {
        *e = *e * 2;
        println!("{}", e);
    }

    for e in v.iter() {
        println!("{e}");
    }

    for e in v2.iter() {
        println!("{e}");
    }

    println!("v: {}, v2: {}", v.len(), v2.len());

    let i = v[0];
    v.push(100);
    println!("{}", i);

    let i2 = &v[0];
    println!("{}", i2);

    let j = v.get(5);
    let jj = j.map_or(-1, |x| x * 2);
    println!("{}", jj);

    match j {
        Some(value) => println!("{value}"),
        None => println!("j is None"),
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.13),
        SpreadsheetCell::Text(String::from("Blue")),
    ];

    for e in &row {
        match e {
            SpreadsheetCell::Int(i) => println!("{i}"),
            SpreadsheetCell::Float(f) => println!("{f}"),
            SpreadsheetCell::Text(t) => println!("{t}"),
        }
    }
}
