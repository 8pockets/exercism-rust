pub fn lsp(s: &str, n: usize) -> Result<u32, &str> {
    let digit: Vec<u32> = s.chars()
        .map(|x| x.to_digit(10))
        .collect::<Option<_>>()
        .ok_or("not all digits")?;
    if n == 0 {
        return Ok(1);
    }
    digit
        .windows(n)
        .map(|slice| slice.into_iter().product::<u32>())
        .max()
        .ok_or("no")
}
