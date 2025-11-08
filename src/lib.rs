// A proprietary, non-standard encoding using u16 manipulation.
// This is for demonstration and is not a secure encryption method.
pub struct CustomEncoding;

const XOR_KEY: u16 = 0x0666;

impl CustomEncoding {
    // Swaps the high and low bytes of a u16.
    fn swap_bytes(word: u16) -> u16 {
        word.rotate_right(8)
    }

    pub fn encode(text: &str) -> Result<Vec<u8>, String> {
        let mut encoded_bytes = Vec::new();
        // Process the string as a sequence of UTF-16 words (u16).
        for val in text.encode_utf16() {
            // Apply the transformation to each u16.
            let transformed = Self::swap_bytes(val) ^ XOR_KEY;
            // Convert the transformed u16 into two bytes (big-endian).
            encoded_bytes.extend_from_slice(&transformed.to_be_bytes());
        }
        Ok(encoded_bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<String, String> {
        // The byte slice must be a multiple of 2.
        if bytes.len() % 2 != 0 {
            return Err("Input byte slice has a length that is not a multiple of 2.".to_string());
        }

        let mut utf16_units = Vec::new();
        // Process the byte slice in 2-byte chunks.
        for chunk in bytes.chunks_exact(2) {
            // Reconstruct the u16 from the two bytes.
            let val = u16::from_be_bytes(chunk.try_into().expect("chunk is 2 bytes"));
            // Reverse the transformation.
            let original_val = Self::swap_bytes(val ^ XOR_KEY);
            utf16_units.push(original_val);
        }

        // Reconstruct the string from the sequence of UTF-16 words.
        String::from_utf16(&utf16_units)
            .map_err(|e| format!("Failed to decode from UTF-16: {}", e))
    }
}
