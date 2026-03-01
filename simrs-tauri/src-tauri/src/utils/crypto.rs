/// Utility untuk enkripsi/dekripsi
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use sha2::{Digest, Sha256};

/// Encrypt data menggunakan AES-256-GCM
pub fn encrypt(data: &str, key: &str) -> Result<String, String> {
    let key_bytes = derive_key(key);
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)
        .map_err(|e| e.to_string())?;

    let nonce = Nonce::from_slice(b"unique nonce"); // Should be random in production
    let ciphertext = cipher
        .encrypt(nonce, data.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(BASE64.encode(&ciphertext))
}

/// Decrypt data menggunakan AES-256-GCM
pub fn decrypt(encrypted: &str, key: &str) -> Result<String, String> {
    let key_bytes = derive_key(key);
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)
        .map_err(|e| e.to_string())?;

    let nonce = Nonce::from_slice(b"unique nonce");
    let ciphertext = BASE64
        .decode(encrypted)
        .map_err(|e| e.to_string())?;

    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| e.to_string())?;

    String::from_utf8(plaintext).map_err(|e| e.to_string())
}

/// Derive key dari password menggunakan SHA-256
fn derive_key(password: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

/// Generate nomor rawat baru
pub fn generate_no_rawat(tanggal: &str, urut: i32) -> String {
    format!("{}/{:06}", tanggal.replace("-", "/"), urut)
}

/// Generate nomor rekam medis baru
pub fn generate_no_rkm_medis(urut: i32) -> String {
    format!("{:06}", urut)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let data = "test_password";
        let key = "secret_key";
        let encrypted = encrypt(data, key).unwrap();
        let decrypted = decrypt(&encrypted, key).unwrap();
        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_generate_no_rawat() {
        let no_rawat = generate_no_rawat("2024-02-14", 1);
        assert_eq!(no_rawat, "2024/02/14/000001");
    }
}