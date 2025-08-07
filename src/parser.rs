use std::fs::File;
use zip::read::ZipArchive;
use quick_xml::Reader;
use quick_xml::events::Event;
use anyhow::{Result, Context};

pub(crate) fn extract_text_from_docx(path: &str) -> Result<String> {
    let file = File::open(path).context("Cannot open .docx file")?;
    let mut archive = ZipArchive::new(file).context("Cannot unpack .docx (zip)")?;

    let mut document_xml = String::new();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.name() == "word/document.xml" {
            use std::io::Read;
            file.read_to_string(&mut document_xml)?;
            break;
        }
    }

    if document_xml.is_empty() {
        anyhow::bail!("word/document.xml has not been found in .docx archive");
    }

    let mut reader = Reader::from_str(&document_xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut text = String::new();

    while let Ok(event) = reader.read_event_into(&mut buf) {
        match event {
            //Event::Start(bytes_start) => todo!(),
            Event::End(ref e) if e.name().as_ref() == b"w:p" => {
                text.push('\n');
            },
            //Event::Empty(bytes_start) => todo!(),
            Event::Text(e) => {
                text.push_str(&e.unescape()?.to_string());
            },
            //Event::CData(bytes_cdata) => todo!(),
            // Event::Comment(bytes_text) => todo!(),
            // Event::Decl(bytes_decl) => todo!(),
            // Event::PI(bytes_text) => todo!(),
            // Event::DocType(bytes_text) => todo!(),
            Event::Eof => break,
            _ => {},
        }
        buf.clear();
    }

    todo!()
}