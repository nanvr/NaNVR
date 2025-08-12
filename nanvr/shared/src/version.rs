use semver::Version;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::LazyLock,
};

pub static NANVR_VERSION: LazyLock<Version> =
    LazyLock::new(|| Version::parse(env!("CARGO_PKG_VERSION")).unwrap());

// Consistent across architectures, might not be consistent across different compiler versions.
pub fn hash_string(string: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish()
}

pub fn is_nightly() -> bool {
    NANVR_VERSION.build.contains("nightly")
}

pub fn is_stable() -> bool {
    NANVR_VERSION.pre.is_empty() && !is_nightly()
}

// Semver compatible versions will produce the same protocol ID. Protocol IDs are not ordered
// As a convention, encode/decode the protocol ID bytes as little endian.
// Only makor and
pub fn protocol_id() -> String {
    if NANVR_VERSION.pre.is_empty() {
        NANVR_VERSION.major.to_string()
    } else {
        format!("{}-{}", NANVR_VERSION.major, NANVR_VERSION.pre)
    }
}

pub fn protocol_id_u64() -> u64 {
    hash_string(&protocol_id())
}

// deprecated
pub fn is_version_compatible(other_version: &Version) -> bool {
    let protocol_string = if other_version.pre.is_empty() {
        other_version.major.to_string()
    } else {
        format!("{}-{}", other_version.major, other_version.pre)
    };

    protocol_id_u64() == hash_string(&protocol_string)
}
