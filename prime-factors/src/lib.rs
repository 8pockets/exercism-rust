pub fn factors(n: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut current = n;
    while current > 1 {
        for i in 2..current + 1 {
            if current % i == 0 {
                result.push(i);
                current /= i;
                break;
            }
        }
    }
    result
}
