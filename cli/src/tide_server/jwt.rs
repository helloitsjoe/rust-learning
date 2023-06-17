use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn sign(name: String) -> Result<String, jwt::Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"shhhh-secret")?;
    let mut claims = BTreeMap::new();
    claims.insert("name", name);
    return claims.sign_with_key(&key);
}

// pub fn verify(name: String, password: String) -> bool {
//     let key: Hmac<Sha256> = Hmac::new_from_slice(b"shhhh-secret")?;
// }
