// Authentication service
// Placeholder for auth-related business logic

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};

// 32 bytes key for AES-256 (should be from config)
const AES_KEY: &[u8; 32] = b"SIMRSKHANZA2024KEY!@#$%^&*()++AB";

/// Decrypt AES encrypted string (compatible with Java version)
pub fn decrypt(encrypted: &str) -> Result<String, String> {
    let cipher = Aes256Gcm::new_from_slice(AES_KEY)
        .map_err(|e| e.to_string())?;

    let bytes = BASE64.decode(encrypted)
        .map_err(|e| e.to_string())?;

    if bytes.len() < 12 {
        return Err("Invalid encrypted data".to_string());
    }

    let (nonce_bytes, ciphertext) = bytes.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| e.to_string())?;

    String::from_utf8(plaintext).map_err(|e| e.to_string())
}

/// Encrypt string with AES (compatible with Java version)
pub fn encrypt(plaintext: &str) -> Result<String, String> {
    use rand::Rng;

    let cipher = Aes256Gcm::new_from_slice(AES_KEY)
        .map_err(|e| e.to_string())?;

    let mut rng = rand::thread_rng();
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| e.to_string())?;

    let mut result = nonce_bytes.to_vec();
    result.extend(ciphertext);

    Ok(BASE64.encode(&result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let original = "test_password";
        let encrypted = encrypt(original).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }
}