use clap::Parser;
use rand::Rng;
use std::iter;

#[derive(Parser, Debug)]
#[command(author="avnogy", version, about = "A simple password generator written in Rust")]
struct Args {
    /// Length of password
    #[arg(short, long, default_value_t = 8)]
    length: usize,

    /// Include uppercase letters
    #[arg(short, long)]
    uppercase: bool,

    /// Include digits
    #[arg(short, long)]
    digits: bool,

    /// Include special characters
    #[arg(short, long)]
    special: bool,
}

fn main() {
    let args = Args::parse(); 
    println!("{}",generate(args.length,args.uppercase,args.digits,args.special));
}

fn generate(length: usize,uppercase: bool,digits: bool,special: bool) -> String {
    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz".as_bytes();
    let uppercase_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    let digits_letters = "0123456789".as_bytes();
    let special_chars = "!@#$%^&*()_+{}[];':\"\\|,.<>?/".as_bytes();

    // creating the char set
    let mut char_set = Vec::new();
    char_set.extend_from_slice(lowercase_letters);
    if uppercase {char_set.extend_from_slice(uppercase_letters)};
    if digits {char_set.extend_from_slice(digits_letters)};
    if special {char_set.extend_from_slice(special_chars)};

    // generating string
    let mut rng = rand::thread_rng();
    let one_char = || char_set[rng.gen_range(0..char_set.len())] as char;   
    iter::repeat_with(one_char).take(length).collect()

}
