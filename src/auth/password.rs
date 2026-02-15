use argon2::{
    Argon2, Algorithm, Params, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand_core::OsRng;

use crate::error::AppError;

/// Hash a password using Argon2id with default parameters and a random salt.
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default());

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {e}")))?;

    Ok(hash.to_string())
}

/// Verify a password against a stored Argon2id hash.
/// Returns `Ok(true)` if the password matches, `Ok(false)` if it does not.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("Invalid password hash format: {e}")))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default());

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(AppError::Internal(format!("Password verification failed: {e}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_succeeds() {
        let password = "correct-horse-battery-staple";
        let hash = hash_password(password).expect("hashing should succeed");
        let result = verify_password(password, &hash).expect("verification should succeed");
        assert!(result, "correct password should verify as true");
    }

    #[test]
    fn test_wrong_password_fails() {
        let password = "correct-horse-battery-staple";
        let hash = hash_password(password).expect("hashing should succeed");
        let result = verify_password("wrong-password", &hash).expect("verification should succeed");
        assert!(!result, "wrong password should verify as false");
    }

    #[test]
    fn test_different_hashes_for_same_password() {
        let password = "same-password-twice";
        let hash1 = hash_password(password).expect("first hash should succeed");
        let hash2 = hash_password(password).expect("second hash should succeed");
        assert_ne!(hash1, hash2, "different salts should produce different hashes");

        // Both hashes should still verify correctly
        assert!(verify_password(password, &hash1).unwrap());
        assert!(verify_password(password, &hash2).unwrap());
    }

    #[test]
    fn test_hash_is_argon2id_format() {
        let hash = hash_password("test").expect("hashing should succeed");
        assert!(
            hash.starts_with("$argon2id$"),
            "hash should be in argon2id PHC format, got: {hash}"
        );
    }
}
