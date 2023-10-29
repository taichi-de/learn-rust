mod input;
use input::*;
use std::collections::HashMap;

pub fn analyze_numbers(){
    let numbers_str = input(Some("Give some numbers like '4 5 2 9 8': "));
    let numbers: Vec<i32> = numbers_str
        .trim()
        .split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();

    println!("Mean: {}", mean(&numbers));
    println!("Median: {}", median(&numbers));
    println!("Mode: {}", mode(&numbers));
}

fn mean(numbers: &[i32]) -> f32{
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn median(numbers: &[i32]) -> f32{
    let mut numbers = numbers.to_vec();
    numbers.sort();

    if numbers.len() % 2 == 0 {
        let mid1 = numbers[numbers.len()/2-1];
        let mid2 = numbers[numbers.len()/2];
        (mid1 + mid2) as f32 / 2.0
    }else{
        numbers[numbers.len()/2] as f32
    }
}

fn mode(numbers: &[i32]) -> i32{
    let mut occurrences = HashMap::new();

    for &number in numbers{
        let count = occurrences.entry(number).or_insert(0);
        *count += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(number, _)| number)
        .expect("Cannot find a mode")
}