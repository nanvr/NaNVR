use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub static NANVR_VERSION: &str = env!("BUILD_GIT_HASH");

// Consistent across architectures, might not be consistent across different compiler versions.
pub fn hash_string(string: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish()
}

// Semver compatible versions will produce the same protocol ID. Protocol IDs are not ordered
// As a convention, encode/decode the protocol ID bytes as little endian.
// Only makor and
pub fn protocol_id() -> String {
    NANVR_VERSION.to_string()
}
