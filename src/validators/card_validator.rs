use std::result;

use crate::domain::card::Card;

pub fn id_validator(id: u32) -> Result<(), String> {
    // no validation, exists cause it may be needed in the future
    Result::Ok(())
}

fn name_validator(name: &str, name_type: &str) -> Result<(), String> {
    if name.len() < 2 || name.len() > 16 {
        Result::Err(format!(
            "The {} needs to be between 1 and 17 characters.",
            name_type,
        ))
    } else if name.contains(' ') {
        Result::Err(format!("The {} can't contain spaces.", name_type))
    } else {
        Result::Ok(())
    }
}

pub fn first_name_validator(first_name: &str) -> Result<(), String> {
    name_validator(first_name, "first name")
}

pub fn last_name_validator(last_name: &str) -> Result<(), String> {
    name_validator(last_name, "last name")
}

pub fn cnp_validator(cnp: &str) -> Result<(), String> {
    if cnp.len() != 13 {
        Result::Err("The CNP must have 13 digits.".into())
    } else if cnp.to_string().chars().into_iter().any(|x| !x.is_numeric()) {
        Result::Err("The CNP must be made out of digits.".into())
    } else {
        Result::Ok(())
    }
}

// dd.mm.yyyy
pub fn date_validator(birthday: &str) -> Result<(), String> {
    if birthday.len() != 10 {
        Result::Err("The birthday must have 10 characters. (dd.mm.yyyy)".into())
    }
    // validate date with format dd.mm.yyyy
    else if match birthday[0..2].parse::<i32>() {
        Ok(x) => !(x > 0 && x < 31),
        Err(err) => return Result::Err(err.to_string()),
    } {
        Result::Err("The date's days must be between 0 and 31.".into())
    } else if match birthday[3..5].parse::<i32>() {
        Ok(x) => !(x > 0 && x < 13),
        Err(err) => return Result::Err(err.to_string()),
    } {
        Result::Err("The date's months must be between 0 and 13.".into())
    } else if match birthday[6..10].parse::<i32>() {
        Ok(x) => !(x > 1925 && x < 2025),
        Err(err) => return Result::Err(err.to_string()),
    } {
        Result::Err("The date's year must be between 1925 and 2025.".into())
    } else if (birthday.chars().nth(2) == birthday.chars().nth(5))
        && (['.', '/', '-'].contains(&birthday.chars().nth(2).unwrap_or('x')))
    {
        Result::Err("The date separators must be of the following kind: '.', '/', '-'".into())
    } else {
        Result::Ok(())
    }
}

pub fn card_validator(card: &Card) -> Result<(), String> {
    let errors = vec![
        id_validator(card.id()),
        first_name_validator(card.first_name()),
        last_name_validator(card.last_name()),
        cnp_validator(card.cnp()),
        date_validator(card.birthday()),
        date_validator(card.registration_date()),
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
