pub fn find() -> Option<u32> {
    for a in 1..1000 {
        for b in 1..1000 - a {
            let c: u32 = 1000 - a - b;
            if c > 0 && a * a + b * b == c * c {
                return Some(a * b * c);
            }
        }
    }
    None
}
