use std::collections::HashMap;

fn main() {
    let numbers_odd = vec![5, 7, 3, 2, 1];
    let numbers_even = vec![2, 8, 4, 6];
    println!("median of {:?} is {}", numbers_odd, median(&numbers_odd));
    println!("median of {:?} is {}", numbers_even, median(&numbers_even));

    let more_numbers = vec![1, 2, 3, 1, 2, 1];
    println!("mode of {:?} is {}", more_numbers, mode(&more_numbers));
}

fn median(numbers: &Vec<i32>) -> f32 {
    let mut xs = numbers.clone();
    xs.sort();
    let n = xs.len();
    if n % 2 == 1 {
        xs[n / 2] as f32
    } else {
        (xs[n / 2 - 1] + xs[n / 2]) as f32 / 2.0
    }
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut occurences = HashMap::new();
    for x in numbers {
        match occurences.get(x) {
            Some(v) => occurences.insert(x, v + 1),
            None => occurences.insert(x, 1),
        };
    }
    let mut best: i32 = 0;
    let mut n: i32 = 0;
    for (&key, &value) in &occurences {
        if value > n {
            n = value;
            best = *key;
        }
    }
    best
}
