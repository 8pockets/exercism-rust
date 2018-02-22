pub fn collatz(n: u64) -> Result<u64, &'static str> {
    let mut x: u64 = n;
    let mut i: u64 = 0;
    loop {
        x = match x {
            0 => return Err("invalid"),
            1 => return Ok(i),
            _ if x % 2 == 0 => x / 2,
            _ => 3 * x + 1,
        };
        i += 1;
    }
}
