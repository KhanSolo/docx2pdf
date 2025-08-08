use std::fs::File;
use std::io::{Read};
use zip::ZipArchive;
use quick_xml::Reader;
use quick_xml::events::Event;
use anyhow::Result;

pub fn extract_text_from_docx(path: &str) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    let mut document_xml = String::new();
    archive.by_name("word/document.xml")?.read_to_string(&mut document_xml)?;

    let mut reader = Reader::from_str(&document_xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut paragraphs = Vec::new();
    let mut current_paragraph = String::new();

    while let Ok(event) = reader.read_event_into(&mut buf) {
        match event {
            Event::Start(ref e) if e.name().as_ref() == b"w:p" => {
                current_paragraph = String::new();
            }
            Event::End(ref e) if e.name().as_ref() == b"w:p" => {
                if !current_paragraph.trim().is_empty() {
                    paragraphs.push(current_paragraph.clone());
                }
            }
            Event::Text(e) => {
                current_paragraph.push_str(&e.unescape()?);
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(paragraphs)
}