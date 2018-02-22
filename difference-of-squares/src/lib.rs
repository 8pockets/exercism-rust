pub fn square_of_sum(n: usize) -> usize {
    let sum: usize = (1..).take(n).fold(0, |a, n| a + n);
    return sum * sum;
}

pub fn sum_of_squares(n: usize) -> usize {
    let sum: usize = (1..).take(n).map(|a| a * a).sum();
    return sum;
}

pub fn difference(n: usize) -> usize {
    return square_of_sum(n) - sum_of_squares(n);
}
