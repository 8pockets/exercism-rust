pub fn is_leap_year(year: i32) -> bool {
    year.divisible_by(400) || (year.divisible_by(4) && !year.divisible_by(100))
}

trait Divisible {
    fn divisible_by(&self, divisor: i32) -> bool;
}

impl Divisible for i32 {
    fn divisible_by(&self, divisor: i32) -> bool {
        self % divisor == 0
    }
}
