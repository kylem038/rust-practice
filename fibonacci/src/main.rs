use std::io;
use std::vec;

fn main() {
    println!("Please enter number for n in fibonnaci");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: usize = input.trim().parse().expect("Failed to parse int");

    let answer: Vec<u32> = fibonnaci(n);

    println!("{:?}", answer);
}

fn fibonnaci(n: usize) -> Vec<u32> {
    // Start at one because we already have one item in the array
    let mut index: usize = 1;
    let mut result: Vec<u32> = vec![1, 1];
    while index < (n - 1) {
        // add fib_number to itself
        let number_to_add_to_array: u32 = result[index - 1] + result[index];
        // concat
        result.push(number_to_add_to_array);
        index = index + 1;
    }
    return result;
}

// example result: Your n was 5: [1,1,2,3,5]
