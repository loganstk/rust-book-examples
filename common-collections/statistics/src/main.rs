use std::collections::HashMap;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9, 10];
    // let numbers = vec![1];
    // let numbers = Vec::new();

    println!("Mode is {}", mode(&numbers).expect("No mode of empty list"));
    println!("Median is {}", median(&numbers).expect("No median of empty list"));
    println!("Mathematical expectation is {}", math_expectation(&numbers).expect("No expectation of empty list"));
}

fn mode(numbers: &Vec<i32>) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }
    
    let mut freqs = HashMap::new();
    for n in numbers {
        let count = freqs.entry(n).or_insert(0);
        *count += 1;
    }
    
    let mut mode: (i32, i32) = (0, -1);
    for (number, count) in freqs.iter() {
        if *count > mode.1 {
            mode = (**number, *count);
        }
    }

    Some(mode.0)
}

fn median(numbers: &Vec<i32>) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }

    let median_idx = numbers.len() / 2;
    let mut numbers_copy = numbers.clone();
    numbers_copy.sort();

    Some(numbers_copy[median_idx])
}

fn math_expectation(numbers: &Vec<i32>) -> Option<f32> {
    if numbers.is_empty() {
        return None;
    }

    let mut sum = 0;
    for n in numbers {
        sum += *n;
    }

    Some((sum as f32) / (numbers.len() as f32))
}
