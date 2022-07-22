use crate::domain::date::Date;

pub fn day_validator(day: u8) -> Result<(), String> {
    if day < 1 && day > 30 {
        Result::Err("The date's day should be between 0 and 31.".into())
    } else {
        Result::Ok(())
    }
}

pub fn month_validator(month: u8) -> Result<(), String> {
    if month < 1 && month > 12 {
        Result::Err("The date's month should be between 0 and 13.".into())
    } else {
        Result::Ok(())
    }
}

pub fn year_validator(year: u32) -> Result<(), String> {
    if year < 1925 && year > 2025 {
        Result::Err("The date's month should be between 1924 and 2026.".into())
    } else {
        Result::Ok(())
    }
}

pub fn date_validator(date: &Date) -> Result<(), String> {
    let errors = vec![
        day_validator(date.day()),
        month_validator(date.month()),
        year_validator(date.year()),
    ];

    let mut msg = String::new();

    for error in errors {
        if let Result::Err(err) = error {
            msg.push_str(&err);
            msg.push(' ');
        }
    }

    if msg.is_empty() {
        Result::Ok(())
    } else {
        Result::Err(msg)
    }
}
