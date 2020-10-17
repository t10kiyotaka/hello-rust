use ferris_says::say;
use std::io::{stdout, BufWriter};

fn hello_world() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn fizzbuzz(upto: u64) {
    for i in 1..upto {
        match (i%3, i%5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i)
        }
    }
}

fn main() {
    hello_world();
    fizzbuzz(30);
}
