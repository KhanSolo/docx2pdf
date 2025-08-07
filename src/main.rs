mod parser;
mod pdf_gen;

use clap::Parser as ClapParser;
use anyhow::Result;

#[derive(ClapParser)]
#[command(author, version, about)]
struct Args {
    input:String, //input file

    #[arg(short, long, default_value = "output.pdf")]
    output:String, //output file
}

fn main() {
    println!("Hello, world!");
}
