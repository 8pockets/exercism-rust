fn is_prime(n: u64) -> bool {
    let limit: u64 = (n as f64).sqrt() as u64 + 1;

    for x in 2..limit {
        if n % x == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u64) -> Result<u64, &'static str> {
    if n < 1 {
        return Err("Invalid value");
    } else {
        let mut i = 1;
        let mut x = 2;
        loop {
            if is_prime(x) {
                if i == n {
                    return Ok(x);
                }
                i += 1;
            }
            x += 1;
        }
    }
}
