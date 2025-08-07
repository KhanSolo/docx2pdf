mod parser;
mod pdf_gen;

use clap::Parser as ClapParser;
use anyhow::{Ok, Result};

#[derive(ClapParser)]
#[command(author, version, about)]
struct Args {
    input:String, //input file

    #[arg(short, long, default_value = "output.pdf")]
    output:String, //output file
}

fn main() -> Result<()>{
    let args = Args::parse();

    println!("Reading .docx {}", args.input);
    let text : String = parser::extract_text_from_docx(&args.input)?;
    println!("Extracted {} chars", text.len());

    println!("Generating PDF: {}", args.output);
    pdf_gen::generate_pdf(&text, args.output)?;

    println!("Successfully completed!");
    Ok(())
}
