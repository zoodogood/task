// use std::collections::HashMap;

pub mod help;
pub mod version;

pub fn as_vec() -> Vec<fn()> {
    vec![help::execute, version::execute]
}

// pub fn as_hash_map() -> &HashMap<&str, fn()> {
//    &HashMap<&str, fn()> = &HashMap::new()
//}
