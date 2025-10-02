use argon2::{Argon2, PasswordHasher, PasswordVerifier, 
    password_hash::{SaltString, PasswordHash, rand_core::OsRng}};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2.hash_password(password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string()
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed_password)
        .expect("Invalid hash format");
    let argon2 = Argon2::default();
    
    argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
}