pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..)
        .take(limit as usize)
        .filter(|i| factors.iter().any(|k| i % k == 0))
        .sum()
}
