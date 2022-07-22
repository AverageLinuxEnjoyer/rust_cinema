use chrono::Datelike;

use crate::validators::date_validators::{
    date_validator, day_validator, month_validator, year_validator,
};

#[derive(Clone, Copy)]
pub struct Date {
    day: u8,
    month: u8,
    year: u32,
}

impl Date {
    pub fn new(day: u8, month: u8, year: u32) -> Result<Date, String> {
        let date = Date { day, month, year };

        match date_validator(&date) {
            Result::Ok(()) => Result::Ok(date),
            Result::Err(err) => Result::Err(err),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.day, self.month, self.year)
    }

    // dd.mm.yyyy
    pub fn from_string(s: &str) -> Result<Date, String> {
        let naive_date = chrono::NaiveDate::parse_from_str(s, "%d.%m.%Y");

        match naive_date {
            Ok(date) => Ok(Date {
                day: date.day() as u8,
                month: date.month() as u8,
                year: date.year() as u32,
            }),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn year(&self) -> u32 {
        self.year
    }

    pub fn set_day(&mut self, day: u8) -> Result<(), String> {
        match day_validator(day) {
            Ok(_) => {
                self.day = day;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn set_month(&mut self, month: u8) -> Result<(), String> {
        match month_validator(month) {
            Ok(_) => {
                self.month = month;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn set_year(&mut self, year: u32) -> Result<(), String> {
        match year_validator(year) {
            Ok(_) => {
                self.year = year;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

impl Default for Date {
    fn default() -> Self {
        Date {
            day: 1,
            month: 1,
            year: 2000,
        }
    }
}
