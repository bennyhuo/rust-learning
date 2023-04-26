#![feature(slice_group_by)]

use std::collections::HashMap;
use std::ops::AddAssign;

pub fn main() {
    let numbers = vec![10, 2, 100, 2, 39, 65, 21, 15, 100, 100, 10, 2, 2];
    let median = find_median(&numbers);
    match median {
        Some(value) => println!("median: {value}"),
        _ => (),
    }

    let mode = find_mode3(&numbers);
    match mode {
        Some(value) => println!("mode: {value}"),
        _ => (),
    }
}

fn find_median(numbers: &Vec<i32>) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }
    let mut new_vec = numbers.to_vec();
    new_vec.sort();

    let pos = new_vec.len() / 2;
    Option::from(*new_vec.get(pos).unwrap())
}

fn find_mode(numbers: &Vec<i32>) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }
    let mut number_count = HashMap::new();
    for number in numbers {
        let count = number_count.entry(number).or_insert(0);
        *count += 1;
    }

    let mut result = 0;
    let mut max_count = 0;
    for (n, count) in number_count {
        if count > max_count {
            result = *n;
            max_count = count;
        }
    }

    Option::from(result)
}

fn find_mode2(numbers: &Vec<i32>) -> Option<i32> {
    numbers
        .iter()
        .fold(HashMap::new(), |mut acc, i| {
            acc.entry(i).or_insert(0).add_assign(1);
            acc
        })
        .iter()
        .max_by(|a, b| (*a).1.cmp((*b).1))
        .map(|e| **e.0)
}

fn find_mode3(numbers: &Vec<i32>) -> Option<i32> {
    numbers
        .group_by(|i, j| i == j)
        .map(|e| (e.first().unwrap(), e.len()))
        .max_by(|a, b| (*a).1.cmp(&(*b).1))
        .map(|e| *e.0)
}
