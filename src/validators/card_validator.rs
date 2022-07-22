use crate::domain::card::Card;
use crate::validators::common_validators::id_validator;
use crate::validators::date_validators::date_validator;

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

pub fn card_validator(card: &Card) -> Result<(), String> {
    let errors = vec![
        id_validator(card.id()),
        first_name_validator(card.first_name()),
        last_name_validator(card.last_name()),
        cnp_validator(card.cnp()),
        date_validator(&card.birthday()),
        date_validator(&card.registration_date()),
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
