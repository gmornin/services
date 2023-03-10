use sha2::{Digest, Sha384};
use std::env;

/// Hash a string using the default salt string at `HASH_SALT`, plus any additional salts.
pub fn hash(plain_text: &str, salt1: Vec<&str>) -> String {
    let mut hasher = Sha384::new();
    hasher.update(format!(
        "{plain_text}{}{}",
        env::var("HASH_SALT").expect("cannot find env `HASH_SALT`"),
        salt1.join("")
    ));
    hex::encode(hasher.finalize())
}
