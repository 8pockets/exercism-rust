pub fn series(_digits: &str, _len: usize) -> Vec<String> {
    (0..(_digits.len() + 1 - _len))
        .map(|i| _digits.chars().skip(i).take(_len).collect())
        .collect()
}
