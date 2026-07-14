
use argon2::{
    password_hash::{
        SaltString, rand_core::OsRng,
    },
    Argon2
};



pub fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

pub fn derive_master_key(password: &[u8], salt: &[u8]) -> Result<[u8; 32], argon2::password_hash::Error> {
    let argon2 = Argon2::default();

    let mut output = [0u8; 32];

    argon2.hash_password_into(password, salt, &mut output)?;
    Ok(output)
}


pub fn encrypt(key: &[u8; 32], nonce)

#[test]
fn derive_master_key_is_deterministic() {
    let password = b"correct horse battery staple";
    let salt = b"some-16-byte-salt"; // must be 8-64 bytes for Argon2

    let key1 = derive_master_key(password, salt).unwrap();
    let key2 = derive_master_key(password, salt).unwrap();

    assert_eq!(key1, key2, "same password + salt should always produce the same key");
}

#[test]
fn derive_master_key_differs_with_different_password() {
    let salt = b"some-16-byte-salt";

    let key1 = derive_master_key(b"password one", salt).unwrap();
    let key2 = derive_master_key(b"password two", salt).unwrap();

    assert_ne!(key1, key2, "different passwords should produce different keys");
}

#[test]
fn derive_master_key_differs_with_different_salt() {
    let password = b"correct horse battery staple";

    let key1 = derive_master_key(password, b"salt-number-one!").unwrap();
    let key2 = derive_master_key(password, b"salt-number-two!").unwrap();

    assert_ne!(key1, key2, "same password with different salt should produce different keys");
}

#[test]
fn generate_salt_is_random() {
    let salt1 = generate_salt();
    let salt2 = generate_salt();

    assert_ne!(
        salt1.as_str(),
        salt2.as_str(),
        "two generated salts should not collide"
    );
}

#[test]
fn generate_salt_works_with_derive_master_key() {
    let password = b"correct horse battery staple";
    let salt = generate_salt();

    // derive_master_key still takes raw bytes, so bridge SaltString -> &[u8]
    let key1 = derive_master_key(password, salt.as_str().as_bytes()).unwrap();
    let key2 = derive_master_key(password, salt.as_str().as_bytes()).unwrap();

    assert_eq!(
        key1, key2,
        "re-deriving with the same generated salt should give the same key"
    );
}