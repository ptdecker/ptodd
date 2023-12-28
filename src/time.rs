//! Utilities for handing std::time::SystemTime
//!
//! This is implemented as a facade around std::time::SystemTime mainly so that
//! we can implement a Display trait for SystemTime
use std::{
    fmt,
    time::{SystemTime, UNIX_EPOCH},
};

pub(crate) struct SimpleSystemTime {
    system_time: SystemTime,
}

impl fmt::Display for SimpleSystemTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let duration_since_epoch = self.system_time.duration_since(UNIX_EPOCH);
        let duration_since_epoch = match &duration_since_epoch {
            Ok(duration) => duration,
            Err(_) => {
                return write!(f, "Time before UNIX epoch");
            }
        };
        let nanoseconds = duration_since_epoch.as_micros();
        write!(
            f,
            "{}.{:0>6}",
            nanoseconds / 1_000_000,
            nanoseconds % 1_000_000
        )
    }
}

impl SimpleSystemTime {
    pub fn now() -> SimpleSystemTime {
        SimpleSystemTime {
            system_time: SystemTime::now(),
        }
    }
}
