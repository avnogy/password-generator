use clap::Parser;
use rand::seq::IteratorRandom;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// length of password 
    #[arg(short, long, default_value_t = 8)]
    length: usize,
}

fn main() {
    let length = Args::parse().length; 


    println!("{}",generate(length.clone()));
}

fn generate(length: usize) -> String {
    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz";
    let uppercase_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = "0123456789";
    let special_chars = "!@#$%^&*()_+{}[];':\"\\|,.<>?/";

    let mut char_set = String::new();
    char_set.push_str(lowercase_letters);
    char_set.push_str(uppercase_letters);
    char_set.push_str(digits);
    char_set.push_str(special_chars);

    let mut rng = rand::thread_rng();
    let password: String = char_set
        .chars()
        .choose_multiple(&mut rng, length)
        .iter()
        .collect();
    password
}
