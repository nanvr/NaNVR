use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub static NANVR_VERSION: &str = env!("BUILD_ID");

// Consistent across architectures, might not be consistent across different compiler versions.
pub fn hash_string(string: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish()
}

pub fn protocol_id() -> String {
    NANVR_VERSION.to_string()
}
