use std::io;

fn main() {
    println!("Input degrees in Fahrenheit");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let farhrenheit: u32 = input.trim().parse().expect("Failed to parse int");

    println!("You input: {} degrees", farhrenheit);

    let celsius = (farhrenheit - 32) * 5 / 9;

    println!(
        "{} degrees Fahrenheit = {} degrees Celsius",
        farhrenheit, celsius
    );
}
