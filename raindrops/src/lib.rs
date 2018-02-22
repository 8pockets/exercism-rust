pub fn raindrops(n: usize) -> String {
    if !n.divisible_by(3) && !n.divisible_by(5) && !n.divisible_by(7) {
        return n.to_string();
    }

    let mut res: String = "".to_string();

    if n.divisible_by(3) {
        res.push_str("Pling");
    }
    if n.divisible_by(5) {
        res.push_str("Plang");
    }
    if n.divisible_by(7) {
        res.push_str("Plong");
    }

    return res;
}

trait Divisible {
    fn divisible_by(&self, divisor: usize) -> bool;
}

impl Divisible for usize {
    fn divisible_by(&self, divisor: usize) -> bool {
        self % divisor == 0
    }
}
