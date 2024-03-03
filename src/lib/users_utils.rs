use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};

pub fn verify_password(word: &str, word_hash: &str) -> bool {
    let word_hash = word_hash.to_owned();
    let word_hash = word_hash.as_str();
    let parsed_hash = PasswordHash::new(word_hash).unwrap();
    let argon2 = Argon2::default();
    let is_word_ok = argon2
        .verify_password(word.as_bytes(), &parsed_hash)
        .is_ok();
    is_word_ok
}

pub fn encrypt_password(word: &String) -> String {
    let word_as_bytes = word.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(word_as_bytes, &salt)
        .unwrap()
        .to_string()
}
