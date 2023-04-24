const RED: i32 = 0xFF0000;
type T = (i32, f64);

/// # Test
/// This is a test.
/// ```
/// let a = test();
/// ```
///
fn test() -> &'static str {
    let a = 100;
    let b = "Hello";
    let c = {
        1
    };
    b
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn main() {
    for i in 0..10 {
        println!("fib({i}): {}", fibonacci(i));
    }
}
