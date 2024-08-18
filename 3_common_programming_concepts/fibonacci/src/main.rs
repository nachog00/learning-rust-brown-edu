use std::io;
use std::collections::HashMap;

fn main() {
    println!("Please input the n-th Fibonacci number you want to calculate.");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u128 = n.trim().parse().expect("Please type a number!");

    let mut cache: HashMap<u128, u128> = HashMap::new();

    println!("The {}th Fibonacci number is: {}", n, fibonacci(n, &mut cache));
}

fn fibonacci(n: u128, cache: &mut HashMap<u128, u128>) -> u128 {
    if let Some(&value) = cache.get(&n) {
        return value;
    }

    let result = if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1, cache) + fibonacci(n - 2, cache)
    };

    cache.insert(n, result);
    result
}
