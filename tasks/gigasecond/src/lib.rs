use time::{OffsetDateTime, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const GIGASECOND: i64 = 1_000_000_000;
    let after_gigasecond =
        OffsetDateTime::from_unix_timestamp(start.assume_utc().unix_timestamp() + GIGASECOND)
            .unwrap();

    DateTime::new(after_gigasecond.date(), after_gigasecond.time())
}
