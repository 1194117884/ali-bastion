use base64::{engine::general_purpose, Engine as _};

// Simple XOR encryption for demonstration purposes
// DO NOT use this in production - it's not secure
pub fn encrypt_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = get_or_create_key();
    let mut result = Vec::new();
    
    for (i, byte) in password.bytes().enumerate() {
        result.push(byte ^ key[i % key.len()]);
    }
    
    Ok(general_purpose::STANDARD.encode(&result))
}

pub fn decrypt_password(encrypted_password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = general_purpose::STANDARD.decode(encrypted_password)?;
    let key = get_or_create_key();
    let mut result = Vec::new();
    
    for (i, byte) in data.iter().enumerate() {
        result.push(byte ^ key[i % key.len()]);
    }
    
    Ok(String::from_utf8(result)?)
}

fn get_or_create_key() -> [u8; 16] {
    // In a real implementation, you would securely store and retrieve the key
    // For this example, we'll use a fixed key
    // DO NOT use this in production - implement proper key management
    [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let original_password = "my_secret_password";
        
        let encrypted = encrypt_password(original_password).expect("Encryption should succeed");
        let decrypted = decrypt_password(&encrypted).expect("Decryption should succeed");
        
        assert_eq!(original_password, decrypted);
    }

    #[test]
    fn test_encrypt_different_output() {
        let password = "test_password";
        
        let encrypted1 = encrypt_password(password).expect("Encryption should succeed");
        let encrypted2 = encrypt_password(password).expect("Encryption should succeed");
        
        // Same input should produce same output with the same key
        assert_eq!(encrypted1, encrypted2);
    }

    #[test]
    fn test_encrypt_empty_password() {
        let password = "";
        
        let encrypted = encrypt_password(password).expect("Encryption should succeed");
        let decrypted = decrypt_password(&encrypted).expect("Decryption should succeed");
        
        assert_eq!(password, decrypted);
    }

    #[test]
    fn test_decrypt_invalid_base64() {
        let result = decrypt_password("invalid_base64!");
        assert!(result.is_err());
    }
}