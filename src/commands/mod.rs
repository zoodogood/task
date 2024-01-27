use std::collections::HashMap;

pub mod help;
pub mod version;

pub fn as_vec() -> Vec<fn()> {
    vec![help::execute, version::execute]
}

pub fn as_hash_map() -> &'static HashMap<&'static str, fn()> {
    static MAP: &HashMap<&str, fn()> = &HashMap::new();

    MAP
}
