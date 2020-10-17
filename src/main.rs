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
    for i in 0..upto {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 5 == 0 {
            println!("Fizz");
        } else if i % 3 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn main() {
    hello_world();
    fizzbuzz(30);
}
