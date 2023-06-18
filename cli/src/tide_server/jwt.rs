use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

const SECRET: &[u8; 12] = b"shhhh-secret";

// TODO: Convert to jsonwebtoken

pub fn sign(name: String) -> Result<String, jwt::Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET)?;
    let mut claims = BTreeMap::new();
    claims.insert("name", name);
    return claims.sign_with_key(&key);
}

pub fn verify(token: String) -> Result<bool, jwt::Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET)?;
    let claims: BTreeMap<String, String> = token.verify_with_key(&key)?;
    // What do we get if invalid? Error?
    return Ok(true);
}
