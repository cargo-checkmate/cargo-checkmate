use humantime::{format_duration, FormattedDuration};
use std::time::Instant;

pub(crate) struct DurationTracker(Instant);

impl DurationTracker {
    pub(crate) fn start() -> Self {
        DurationTracker(Instant::now())
    }

    pub(crate) fn finish(self) -> Duration {
        let raw_duration = Instant::now().duration_since(self.0);
        let ms = u64::try_from(raw_duration.as_millis()).unwrap();
        Duration(std::time::Duration::from_millis(ms))
    }
}

pub(crate) struct Duration(std::time::Duration);

impl Duration {
    pub(crate) fn format_human(self) -> FormattedDuration {
        format_duration(self.0)
    }

    pub(crate) fn format_seconds(self) -> String {
        let s = self.into_ms_f64();
        format!("{s:>7.3} seconds")
    }

    fn into_ms_f64(self) -> f64 {
        // 2^32 ms is ~49 days:
        u32::try_from(self.0.as_millis())
            .map(|ms| f64::from(ms) / 1000f64)
            .unwrap_or(f64::INFINITY)
    }
}
