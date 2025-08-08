use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use anyhow::Result;

pub fn generate_pdf(paragraphs: &[String], output_path: &str) -> Result<()> {
    let (doc, page1, layer1) = PdfDocument::new("Docx to PDF", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;

    let mut y = Mm(280.0);
    for para in paragraphs {
        if y.0 < 10.0 {
            break;
        }

        current_layer.use_text(para, 12.0, Mm(20.0), y, &font);
        y.0 -= 10.0;
    }

    doc.save(&mut BufWriter::new(File::create(output_path)?))?;
    Ok(())
}