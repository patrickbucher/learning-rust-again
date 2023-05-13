use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io;

fn main() {
    // exercise 1: median, mode
    let numbers_odd = vec![5, 7, 3, 2, 1];
    let numbers_even = vec![2, 8, 4, 6];
    let more_numbers = vec![1, 2, 3, 1, 2, 1];
    println!("median of {:?} is {}", numbers_odd, median(&numbers_odd));
    println!("median of {:?} is {}", numbers_even, median(&numbers_even));
    println!("mode of {:?} is {}", more_numbers, mode(&more_numbers));

    // exercise 2: pig latin
    let words = vec!["first", "apple", "tree", "barn", "entrance"];
    println!("pig latin for {:?} is {:?}", words, pig_latin(&words));

    // exercise 3: human resources
    let mut employees_by_department: HashMap<String, Vec<String>> = HashMap::new();
    println!("Command: 'Add [name] to [department]', 'quit' to exit");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let words: Vec<&str> = input.split_whitespace().collect();
        if words.len() == 0 {
            continue;
        }
        if words[0].eq("quit") {
            break;
        }
        if words.len() < 4 {
            continue;
        }
        if words[0].eq("Add") && words[2].eq("to") {
            let new_employee = words[1].to_string();
            match employees_by_department.entry(words[3].to_string()) {
                Entry::Vacant(e) => { e.insert(vec![new_employee]); },
                Entry::Occupied(mut e) => { e.get_mut().push(new_employee); },
            };
        }
    }
    let mut departments: Vec<String> = employees_by_department.keys().cloned().collect();
    departments.sort();
    for d in departments {
        println!("Department '{}'", d);
        if let Some(employees) = employees_by_department.get(&d) {
            let mut employees = employees.clone();
            employees.sort();
            for e in employees {
                println!("- Employee '{}'", e);
            }
        }
    }
}

fn pig_latin(words: &Vec<&str>) -> Vec<String> {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'y'];
    let move_first_consonant = |w: &str| -> String {
        let chars = w.chars();
        for (i, c) in chars.enumerate() {
            if !vowels.contains(&c) {
                let left = &w[0..i];
                let right = &w[i+1..];
                return format!("{}{}-{}", left, right, c);
            }
        }
        w.to_string()
    };
    let starts_with_vowel = |w: &str| -> bool {
        match w.chars().next() {
            Some(c) => vowels.contains(&c),
            None => false,
        }
    };
    let latinize = |w: &str| -> String {
        if starts_with_vowel(w) {
            format!("{}-hay", w)
        } else {
            format!("{}ay", move_first_consonant(w))
        }
    };
    let mut latinized: Vec<String> = Vec::new();
    for word in words {
        latinized.push(latinize(word));
    }
    latinized
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
