pub fn check(s: &str) -> bool {
    s.to_lowercase()
        .chars()
        .enumerate()
        .filter(|&(_, c)| (c != '-') && (!c.is_whitespace()))
        .all(|(i, c)| s[i + 1..].find(c) == None)
}
