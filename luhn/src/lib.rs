pub fn is_valid(s: &str) -> bool {
    let mut count = 0;
    let mut total_sum = 0;

    for (i, c) in s.chars().rev().filter(|x| !x.is_whitespace()).enumerate() {
        match c.to_digit(10) {
            None => {
                return false;
            }
            Some(mut n) => {
                if i % 2 == 1 {
                    n *= 2;
                    if n > 9 {
                        n -= 9;
                    }
                }
                total_sum += n;
                count += 1;
            }
        }
    }
    count > 1 && total_sum % 10 == 0
}
