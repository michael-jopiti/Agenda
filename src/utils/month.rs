use chrono::{NaiveDate, Datelike};
use crate::utils::day::Day;

#[derive(Debug, Clone, PartialEq)]
pub struct Month {
    name: MonthName,
    days: Vec<Day>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MonthName {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    /// Creates a `Month` with the correct days for a given year
    pub fn new(name: MonthName, year: i32) -> Self {
        let days_count = Month::days_in_month(name, year);
        
        let days = (1..=days_count)
            .map(|day| Day::new(NaiveDate::from_ymd_opt(year, name.as_u32(), day.into()).unwrap()))
            .collect();

        Self { name, days }
    }

    /// Returns the number of days for a given month and year
    pub fn days_in_month(name: MonthName, year: i32) -> u8 {
        match name {
            MonthName::January | MonthName::March | MonthName::May | MonthName::July 
            | MonthName::August | MonthName::October | MonthName::December => 31,
            MonthName::April | MonthName::June | MonthName::September | MonthName::November => 30,
            MonthName::February => if Month::is_leap_year(year) { 29 } else { 28 },
        }
    }

    /// Checks if a year is a leap year
    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    pub fn name(&self) -> &MonthName{
        &self.name
    }

    pub fn days(&mut self) -> &mut Vec<Day> {
        &mut self.days
    }
}

impl MonthName{
        /// Converts `MonthName` to its corresponding number (1-12)
        pub fn as_u32(&self) -> u32 {
            match self {
                MonthName::January => 1,
                MonthName::February => 2,
                MonthName::March => 3,
                MonthName::April => 4,
                MonthName::May => 5,
                MonthName::June => 6,
                MonthName::July => 7,
                MonthName::August => 8,
                MonthName::September => 9,
                MonthName::October => 10,
                MonthName::November => 11,
                MonthName::December => 12,
            }
        }

        pub fn name(&self) -> &str{
            match self{
                MonthName::January => &"January",
                MonthName::February => &"February",
                MonthName::March => &"March",
                MonthName::April => &"April",
                MonthName::May => &"May",
                MonthName::June => &"June",
                MonthName::July => &"July",
                MonthName::August => &"August",
                MonthName::September => &"September",
                MonthName::October => &"October",
                MonthName::November => &"November",
                MonthName::December => &"December",
            }
        }
}
