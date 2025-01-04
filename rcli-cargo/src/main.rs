use clap::Parser;
use std::io;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    println!("@@@@@@@@@@@@@@ START UP @@@@@@@@@@@@@@\n");
    println!("Hello, User!\n");
    println!("There are some functions u can use with me:");

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    println!("You said: {}", buffer);

}