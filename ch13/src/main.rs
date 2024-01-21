use std::any::Any;
use std::collections::HashMap;

fn need_i(i: i32) {
    println!("{i}");
}

fn main() {
    let a = 1;
    need_i(a);

    println!("{a}");

    let mut v1 = vec![1i32, 2, 3];

    let v1_mut_iter = v1.iter_mut();
    for mut_val in v1_mut_iter {
        *mut_val = *mut_val * 10;
    }

    let v1_iter = v1.iter();

    // for val in v1 {}

    for val in v1_iter {
        println!("Got: {val}");
    }

    let sum: i32 = v1.iter().sum();

    // y = f(x)
    // y1 = g(x)
    // y2 = g(f(x))
    // g . f
    // y = x
    let sum = v1
        .iter()
        .map(|x| x * 2)
        .flat_map(|x| vec![x, x, x])
        .fold(100, |acc, e| acc + e);

    println!("{sum}");

    let v2 = v1.iter().map(|x| x * 2);

    let s;

    s = "hello world";

    let multiply2 = |i| i * 2;
    multiply2(10);

    // let x = "Hello";
    // let y = match x {
    //     "A" => 1,
    //     "B" => 2.0,
    //     "C" => "Hello",
    //     _ => vec![],
    // };

    let cache = Cache {
        values: HashMap::new(),
    };
    let x: Option<&i32> = cache.get_value("hello");
}

struct Cache {
    values: HashMap<String, Box<dyn Any>>,
}

impl Cache {
    fn get_value<T: 'static>(&self, key: &str) -> Option<&T> {
        self.values
            .get(key)
            .map(|value| value.downcast_ref())
            .flatten()
    }
}

fn test<T>() -> T {
    panic!()
}
