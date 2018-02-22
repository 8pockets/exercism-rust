use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    h: i32,
    m: i32,
}

impl Clock {
    pub fn new(h: i32, m: i32) -> Clock {
        Clock {
            h: modulo((h + ((m as f64 / 60.0).floor() as i32)), 24),
            m: modulo(m, 60),
        }
    }

    pub fn add_minutes(self, addm: i32) -> Clock {
        Clock::new(self.h, self.m + addm)
    }
}

fn modulo(x: i32, n: i32) -> i32 {
    (x % n + n) % n
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.h, self.m)
    }
}
