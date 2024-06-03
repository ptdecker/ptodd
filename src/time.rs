//! Utilities for handing std::time::SystemTime
//!
//! This is implemented as a facade around std::time::SystemTime mainly so that
//! we can implement a Display trait for SystemTime
use std::{
    fmt,
    time::{SystemTime, UNIX_EPOCH},
};

/// Months
#[derive(Debug, Clone, Copy)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl Month {
    pub fn next_month(&self) -> Month {
        match self {
            Month::January => Month::February,
            Month::February => Month::March,
            Month::March => Month::April,
            Month::April => Month::May,
            Month::May => Month::June,
            Month::June => Month::July,
            Month::July => Month::August,
            Month::August => Month::September,
            Month::September => Month::October,
            Month::October => Month::November,
            Month::November => Month::December,
            Month::December => Month::January,
        }
    }
}

// Define an error type for TryFrom
#[derive(Debug)]
pub struct InvalidMonthError;

impl fmt::Display for InvalidMonthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid month value")
    }
}

impl std::error::Error for InvalidMonthError {}

// Implement TryFrom<u8> for Month
impl TryFrom<u8> for Month {
    type Error = InvalidMonthError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(InvalidMonthError),
        }
    }
}

// Implement TryInto<u8> for Month
impl From<Month> for u8 {
    fn from(month: Month) -> Self {
        month as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SimpleSystemTime {
    epoch_seconds: u64,
    epoch_sub_nanoseconds: u32,
    epoch_days: u64,
    year: u16,
    day_of_year: u16,
    month: Month,
    day: u8,
}

impl fmt::Display for SimpleSystemTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let month_num: u8 = self.month.into();
        write!(
            f,
            "{:04}/{:02}/{:02}, Day {}, ({} sec {} ns, day {}, from epoch)",
            self.year,
            month_num,
            self.day,
            self.day_of_year,
            self.epoch_seconds,
            self.epoch_sub_nanoseconds,
            self.epoch_days
        )
    }
}

/// Determine if a year is a leap year
///
/// Any year prior to 1582 when the Gregorian calendar was adopted is returned as 'false' since leap years did
/// not exist prior to its adoption. For years beyond 1582, the following rules are followed:
///
///   div 4  | div 100 | div 400 |  leap?  | example
/// ---------+---------+---------+---------+---------
///     F    |    F    |    F    |    F    |  2019
///     T    |    F    |    F    |    T    |  2020
///     -    |    T    |    F    |    F    |  1900
///     -    |    -    |    T    |    T    |  2000
pub fn is_leap_year(year: u16) -> bool {
    if year < 1582 || year % 4 != 0 {
        return false;
    }
    if year % 100 != 0 {
        return true;
    }
    year % 400 == 0
}

/// Determine the number of days in a month of a given year
pub fn days_in_month(year: u16, month: Month) -> u8 {
    match month {
        Month::January => 31,
        Month::February => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        Month::March => 31,
        Month::April => 30,
        Month::May => 31,
        Month::June => 30,
        Month::July => 31,
        Month::August => 31,
        Month::September => 30,
        Month::October => 31,
        Month::November => 30,
        Month::December => 31,
    }
}

// Determine the year given the number of days since the Unix epoch
//
// This calculation is done via brute force by iterating through the years
fn year(epoch_days: u64) -> (u16, u16) {
    let mut year = 1970u16;
    let mut remaining_days = epoch_days;
    loop {
        match is_leap_year(year) {
            false if remaining_days <= 365 => break,
            false => {
                remaining_days -= 365;
            }
            true if remaining_days <= 366 => break,
            true => {
                remaining_days -= 366;
            }
        };
        year += 1;
    }
    (year, remaining_days as u16)
}

// Determine the month given the day of the year
//
// This calculation is done via brute force by iterating through the years
fn month(year: u16, day_of_year: u16) -> (Month, u8) {
    let mut month = Month::January;
    let mut remaining_days = day_of_year;
    loop {
        let days_in_month = days_in_month(year, month) as u16;
        if remaining_days <= days_in_month {
            break;
        };
        remaining_days -= days_in_month;
        month = month.next_month();
    }
    (month, remaining_days as u8)
}

impl SimpleSystemTime {
    /// Retrieves the current time
    pub fn now() -> SimpleSystemTime {
        let now = SystemTime::now();
        let duration = unsafe { now.duration_since(UNIX_EPOCH).unwrap_unchecked() };
        let epoch_seconds = duration.as_secs();
        let epoch_days = epoch_seconds / 86_400;
        let (year, day_of_year) = year(epoch_days);
        let (month, day) = month(year, day_of_year);

        SimpleSystemTime {
            epoch_seconds,
            epoch_sub_nanoseconds: duration.subsec_nanos(),
            epoch_days,
            year,
            day_of_year,
            month,
            day,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::time::is_leap_year;

    // Make sure the leap year function tests
    #[test]
    fn leap_year() {
        // not leap year - div 100 true, div 400 false
        assert!(!is_leap_year(1900));
        // leap year - div 400 true
        assert!(is_leap_year(2000));
        // not leap year - div 4 false
        assert!(!is_leap_year(2019));
        // leap year - div 4 true, div 100 false
        assert!(is_leap_year(2020));
    }
}
