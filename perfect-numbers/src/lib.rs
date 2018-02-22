use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Result<Classification, &'static str> {
    if num < 1 {
        return Err("Number must be positive");
    }
    // let sum = (1..num).filter(|i| num % i == 0).sum::<u64>();
    let mut sum = 0 as u64;
    (1..num / 2 + 1).for_each(|i| {
        if num % i == 0 {
            sum += i
        }
    });
    match sum.cmp(&num) {
        Equal => Ok(Classification::Perfect),
        Greater => Ok(Classification::Abundant),
        Less => Ok(Classification::Deficient),
    }
}
