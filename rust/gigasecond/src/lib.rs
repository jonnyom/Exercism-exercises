use chrono::{DateTime, NaiveDateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(start.timestamp() + 10i64.pow(9), 0),
        Utc,
    )
}
