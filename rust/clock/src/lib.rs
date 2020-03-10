use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock(i32, i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock(
            (hours + minutes.div_euclid(60)).rem_euclid(24),
            minutes.rem_euclid(60)
        )
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.0, self.1 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{:02}:{:02}", self.0, self.1)
    }
}
