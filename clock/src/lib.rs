use chrono::{Duration, NaiveTime};

#[derive(Debug)]
pub struct Clock {
    time: NaiveTime,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let base_time = NaiveTime::from_hms(0, 0, 0);

        let time_with_hours = base_time
            .overflowing_add_signed(Duration::hours(hours.into()))
            .0;

        let time_with_hours_and_minutes = time_with_hours
            .overflowing_add_signed(Duration::minutes(minutes.into()))
            .0;

        Self {
            time: time_with_hours_and_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let time_updated = self
            .time
            .overflowing_add_signed(Duration::minutes(minutes.into()))
            .0;

        Self { time: time_updated }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.time.format("%H:%M").fmt(f)
    }
}

impl std::cmp::PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.time == other.time
    }
}
