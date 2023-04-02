use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// length of password 
    #[arg(short, long, default_value_t = 8)]
    length: u16,
}

fn main() {
    let args = Args::parse(); 
    println!("{:?}",&args);
}
