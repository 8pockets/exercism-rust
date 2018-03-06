use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    for i in s.to_lowercase().split(|c| !char::is_alphanumeric(c)) {
        if i.len() > 0 {
            let count = map.entry(i.to_owned()).or_insert(0);
            *count += 1;
        }
    }
    map
}
