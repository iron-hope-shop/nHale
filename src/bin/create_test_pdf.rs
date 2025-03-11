use lopdf::{dictionary, Document, Object};
use std::path::Path;

fn main() {
    let output_path = Path::new("tests/fixtures/test.pdf");

    // Create output directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    // Create a simple PDF
    let mut doc = Document::new();

    // Create a simple PDF structure
    let pages_id = doc.new_object_id();
    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => "This is a test PDF for nHale steganography testing."
    });

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![Object::Reference(page_id)],
        "Count" => 1,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);

    // Save the PDF
    doc.save(output_path).expect("Failed to save test PDF");

    println!("Test PDF created at: {:?}", output_path);
}
