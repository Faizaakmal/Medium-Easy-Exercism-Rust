#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
  Abundant,
  Perfect,
  Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    else if sums(num) == num {
        Some(Classification::Perfect)
    }
    else if sums(num) > num {
        Some(Classification::Abundant)
    }
    else {
        Some(Classification::Deficient)
    }
}
pub fn sums(num: u64) -> u64 {
    let mut sum = 0;
    for x in 1..num {
        if num % x == 0 {
            sum += x
        }
    }
    return sum
}