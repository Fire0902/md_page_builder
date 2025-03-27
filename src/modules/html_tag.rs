use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static HASHMAP: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("#", "<h1>");
    map.insert("##", "<h2>");
    map.insert("###", "<h3>");
    map
});
