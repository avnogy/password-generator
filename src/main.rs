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

    #[arg(short, long , action)]
    digits: bool,

    #[arg(short, long , action)]
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

    let mut char_set = Vec::new();
    char_set.extend_from_slice(lowercase_letters);
    if uppercase {char_set.extend_from_slice(uppercase_letters)};
    if digits {char_set.extend_from_slice(digits_letters)};
    if special {char_set.extend_from_slice(special_chars)};

    let mut rng = rand::thread_rng();
    let one_char = || char_set[rng.gen_range(0..char_set.len())] as char;   
    iter::repeat_with(one_char).take(length).collect()

}
