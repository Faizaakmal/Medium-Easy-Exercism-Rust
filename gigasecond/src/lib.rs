extern crate chrono;
use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    return start + Duration::seconds(1_000_000_000);
}
// https://exercism.io/my/solutions/8a4cdac0d1ee44918b2832e2da7f4dae
