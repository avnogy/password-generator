use clap::Parser;
use rand::Rng;
use std::iter;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// length of password 
    #[arg(short, long, default_value_t = 8)]
    length: usize,

    #[arg(short, long , action)]
    uppercase: bool,
}

fn main() {
    let length = Args::parse().length; 
    let uppercase = Args::parse().uppercase; 


    println!("{}",generate(length.clone(),uppercase));
}

fn generate(length: usize,uppercase: bool) -> String {
    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz".as_bytes();
    let uppercase_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    let digits = "0123456789".as_bytes();
    let special_chars = "!@#$%^&*()_+{}[];':\"\\|,.<>?/".as_bytes();

    let mut char_set = Vec::new();
    char_set.extend_from_slice(lowercase_letters);
    if uppercase {char_set.extend_from_slice(uppercase_letters)};
    char_set.extend_from_slice(digits);
    char_set.extend_from_slice(special_chars);

    let mut rng = rand::thread_rng();
    let one_char = || char_set[rng.gen_range(0..char_set.len())] as char;   
    iter::repeat_with(one_char).take(length).collect()

}
