use prtgn_encodeing::CustomEncoding;
use std::fs::File;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example with a wider range of characters, including emojis.
    let original_text = "This is a secret message! With Unicode: 世界 😊";
    println!("Original text: '{}'", original_text);

    let encoded_bytes = CustomEncoding::encode(original_text)?;
    println!("Encoded ({} bytes): {:?}", encoded_bytes.len(), encoded_bytes);

    // Write to file
    let mut file = File::create("custom_encoded.bin")?;
    file.write_all(&encoded_bytes)?;

    // Read from file
    let mut file = File::open("custom_encoded.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let decoded_text = CustomEncoding::decode(&buffer)?;
    println!("Decoded text: '{}'", decoded_text);

    assert_eq!(original_text, decoded_text);
    println!("\nSuccessfully encoded and decoded the text.");

    Ok(())
}
