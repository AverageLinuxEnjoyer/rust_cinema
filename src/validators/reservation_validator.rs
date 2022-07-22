use super::common_validators::id_validator;
use super::date_validators::date_validator;
use crate::domain::reservation::Reservation;

pub fn time_validator(time: &str) -> Result<(), String> {
    if time.len() != 5 {
        Result::Err("The time must have 5 characters. (hh:mm)".into())
    } else if match time[0..2].parse::<i32>() {
        Ok(x) => (0..=23).contains(&x),
        Err(err) => return Result::Err(err.to_string()),
    } {
        Result::Err("The hour must be from 0 to 23.".into())
    } else if match time[3..5].parse::<i32>() {
        Ok(x) => (0..=59).contains(&x),
        Err(err) => return Result::Err(err.to_string()),
    } {
        Result::Err("The minutes must be from 0 to 59.".into())
    } else if ['.', ':', '-'].contains(&time.chars().nth(2).unwrap_or('x')) {
        Result::Err("The time separators must be of the following kind: '.', ':', '-'".into())
    } else {
        Result::Ok(())
    }
}

pub fn card_id_validator(id: Option<u32>) -> Result<(), String> {
    match id {
        None => Result::Ok(()),
        Some(x) => id_validator(x),
    }
}

pub fn reservation_validator(reservation: &Reservation) -> Result<(), String> {
    let errors = vec![
        id_validator(reservation.id()),
        id_validator(reservation.movie_id()),
        card_id_validator(reservation.card_id()),
        date_validator(&reservation.date()),
        time_validator(reservation.hour()),
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
