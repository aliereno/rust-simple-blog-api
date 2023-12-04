use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};

pub fn generate_hash(_password: &str) -> BcryptResult<String> {
    hash(_password, DEFAULT_COST)
}

pub fn verify_hash(_password: &str, _hash: &str) -> BcryptResult<bool> {
    verify(_password, _hash)
}
