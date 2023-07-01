use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let output = input
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("... ");

    println!("{}", output);
}
