use clap::Parser;
use rand::Rng;
use std::iter;

#[derive(Parser, Debug)]
#[command(author="avnogy", version, about = "A simple password generator written in Rust")]
struct Args {
    /// Length of password
    #[arg(short, long, default_value_t = 8)]
    length: usize,

    /// Include all characters (overrides other flags)
    #[arg(short, long)]
    all: bool,

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
    let mut args = Args::parse(); 
    if args.all {[args.uppercase,args.digits,args.special] = [true; 3]};
    println!("{}",generate(args.length,args.uppercase,args.digits,args.special));
}

fn generate(length: usize,uppercase: bool,digits: bool,special: bool) -> String {
    // creating the char set
    let mut char_set = Vec::new();
    char_set.extend_from_slice("abcdefghijklmnopqrstuvwxyz".as_bytes());
    if uppercase {char_set.extend_from_slice("ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes())};
    if digits {char_set.extend_from_slice("0123456789".as_bytes())};
    if special {char_set.extend_from_slice("!@#$%^&*()_+{}[];':\"\\|,.<>?/".as_bytes())};

    // generating string
    let mut rng = rand::thread_rng();
    let one_char = || char_set[rng.gen_range(0..char_set.len())] as char;   
    iter::repeat_with(one_char).take(length).collect()

}
