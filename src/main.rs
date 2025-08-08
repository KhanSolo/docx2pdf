mod parser;
mod pdf_gen;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("📂 Reading file: {}", args.input);
    let paragraphs = parser::extract_text_from_docx(&args.input)?;
    println!("📝 Found paragraphs: {}", paragraphs.len());

    println!("📄 Generating PDF: {}", args.output);
    pdf_gen::generate_pdf(&paragraphs, &args.output)?;

    println!("✅ Successfully completed!");
    Ok(())
}