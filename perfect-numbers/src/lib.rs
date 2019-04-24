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
// https://exercism.io/my/solutions/e0240a2a29c94a43a4ab1d9b9e2685c7
