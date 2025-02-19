use aes_gcm::aead::{rand_core, Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM with 256-bit key and 96-bit nonce
use rand_core::RngCore;

/// **Encrypt Data using AES-256-GCM**
pub fn encrypt_data(data: &str, key: &[u8]) -> Result<Vec<u8>, &'static str> {
    // Ensure the key is 32 bytes (256 bits)
    if key.len() != 32 {
        return Err("Invalid key length. Expected 32 bytes.");
    }

    // Explicitly annotate the key type
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    // Generate a secure 12-byte nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt data with nonce
    match cipher.encrypt(nonce, data.as_bytes()) {
        Ok(mut ciphertext) => {
            // Prepend nonce to ciphertext for decryption
            let mut result = nonce_bytes.to_vec();
            result.append(&mut ciphertext);
            Ok(result)
        }
        Err(_) => Err("Encryption failed"),
    }
}

/// **Decrypt AES-256-GCM Encrypted Data**
pub fn decrypt_data(encrypted_data: &[u8], key: &[u8]) -> Result<String, &'static str> {
    // Ensure the key is 32 bytes (256 bits)
    if key.len() != 32 {
        return Err("Invalid key length. Expected 32 bytes.");
    }

    if encrypted_data.len() < 12 {
        return Err("Invalid encrypted data length. Missing nonce.");
    }

    // Explicitly annotate the key type
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    // Extract nonce and ciphertext
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Decrypt the ciphertext
    match cipher.decrypt(nonce, ciphertext) {
        Ok(plaintext_bytes) => match String::from_utf8(plaintext_bytes) {
            Ok(plaintext) => Ok(plaintext),
            Err(_) => Err("Decryption succeeded but UTF-8 decoding failed."),
        },
        Err(_) => Err("Decryption failed. Incorrect key or corrupted data."),
    }
}
