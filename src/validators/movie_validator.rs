use crate::domain::movie::Movie;
use crate::validators::common_validators::id_validator;

pub fn title_validator(title: &str) -> Result<(), String> {
    if title.len() > 1 && title.len() < 21 {
        Result::Ok(())
    } else {
        Result::Err("The title needs to be between 1 and 21 characters.".into())
    }
}

pub fn release_year_validator(release_year: u32) -> Result<(), String> {
    if release_year > 1925 && release_year < 2025 {
        Result::Ok(())
    } else {
        Result::Err("The release year needs to be between 1925 and 2025".into())
    }
}

pub fn price_validator(price: u32) -> Result<(), String> {
    if price < 10_000 {
        Result::Ok(())
    } else {
        Result::Err("The price must be lower than 10.000".into())
    }
}

pub fn movie_validator(movie: &Movie) -> Result<(), String> {
    let errors = vec![
        id_validator(movie.id()),
        title_validator(movie.title()),
        release_year_validator(movie.release_year()),
        price_validator(movie.price()),
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
