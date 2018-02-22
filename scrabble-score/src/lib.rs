pub fn score(s: &str) -> u32 {
    s.to_lowercase().chars().fold(0, |sum, c| match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => sum + 1,
        'd' | 'g' => sum + 2,
        'b' | 'c' | 'm' | 'p' => sum + 3,
        'f' | 'h' | 'v' | 'w' | 'y' => sum + 4,
        'k' => sum + 5,
        'j' | 'x' => sum + 8,
        'q' | 'z' => sum + 10,
        _ => sum + 0,
    })
}
