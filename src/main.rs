use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let mut sum: i64 = 0;

    for token in input.split_whitespace() {
        sum += token.parse::<i64>().expect("Not a number");
    }

    println!("{sum}")
}
