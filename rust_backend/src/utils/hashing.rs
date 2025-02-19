use argon2::password_hash::{
    rand_core::OsRng, Error as PasswordHashError, PasswordHash, SaltString,
};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

/// **Securely hash a password using Argon2id**
pub fn hash_password(password: &str) -> Result<String, String> {
    // Generate a secure salt
    let salt = SaltString::generate(&mut OsRng);

    // Use Argon2id as the default algorithm
    let argon2 = Argon2::default();

    // Hash the password with the salt
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Password hashing failed: {}", e))
}

/// **Verify a password against an Argon2id hash**
pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let argon2 = Argon2::default();

    // Parse the hash
    let parsed_hash = PasswordHash::new(hash).map_err(|e| format!("Invalid hash format: {}", e))?;

    // Verify the password
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(PasswordHashError::Password) => Ok(false), // Password mismatch
        Err(e) => Err(format!("Password verification failed: {}", e)), // Other errors
    }
}
