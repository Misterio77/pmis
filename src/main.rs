use clap::Parser;
use pmis::cli::Cli;

fn main() {
    let cli = Cli::parse();
    println!("Hello, world!");
}
