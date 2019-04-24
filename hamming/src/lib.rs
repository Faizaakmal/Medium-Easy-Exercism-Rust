/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.chars().count() != s2.chars().count() { 
        None
    }
    else if s1 == s2 {
        Some(0)
    }
    else {
        Some(s1.chars().zip(s2.chars())
                .filter(|&(left, right)| left!= right).count())
    }
}
// https://exercism.io/my/solutions/61eb386a167c4307b4a48bdec325c165
