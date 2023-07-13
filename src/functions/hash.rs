use sha2::{Digest, Sha384};

use crate::HASH_SALT;

/// Hash a string using the default salt string at `HASH_SALT`, plus any additional salts.
pub fn hash(plain_text: &str, salt1: &str) -> String {
    let mut hasher = Sha384::new();
    hasher.update(format!("{plain_text}{}{}", HASH_SALT.get().unwrap(), salt1));
    hex::encode(hasher.finalize())
}
