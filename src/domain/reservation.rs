use crate::domain::date::Date;
use crate::repo::traits::Serializable;
use crate::validators::common_validators::id_validator;
use crate::validators::reservation_validator::{reservation_validator, time_validator};

pub struct Reservation {
    id: u32,
    movie_id: u32,
    card_id: Option<u32>,
    date: Date,
    hour: String,
}

impl Serializable for Reservation {
    fn to_csv(&self) -> String {
        let card_id = if self.card_id.is_none() {
            "None".to_string()
        } else {
            self.card_id.unwrap().to_string()
        };

        format!(
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
            self.id,
            self.movie_id,
            card_id,
            self.date.to_string(),
            self.hour
        )
    }

    fn from_csv_to_obj(s: &str) -> Reservation {
        let mut parts = s.trim().trim_matches('"').split("\",\"");

        let id: u32 = parts.next().unwrap_or("0").parse().unwrap_or(0);

        let movie_id: u32 = parts.next().unwrap_or("0").parse().unwrap_or(0);

        let card_id = parts.next().unwrap_or("none");

        let date: String = parts.next().unwrap_or("01.01.2000").to_string();
        let date = Date::from_string(&date).unwrap_or_default();

        let hour: String = parts.next().unwrap_or("00:00").to_string();

        Reservation {
            id,
            movie_id,
            date,
            hour,
            card_id: match card_id.parse::<u32>() {
                Ok(x) => Option::Some(x),
                Err(_) => Option::None,
            },
        }
    }
}

impl Reservation {
    pub fn new(
        id: u32,
        movie_id: u32,
        card_id: Option<u32>,
        date: Date,
        hour: String,
    ) -> Result<Reservation, String> {
        let reservation = Reservation {
            id,
            movie_id,
            card_id,
            date,
            hour,
        };

        match reservation_validator(&reservation) {
            Result::Ok(_) => Result::Ok(reservation),
            Result::Err(err) => Result::Err(err),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn movie_id(&self) -> u32 {
        self.movie_id
    }

    pub fn card_id(&self) -> Option<u32> {
        self.card_id
    }

    pub fn date(&self) -> Date {
        self.date
    }

    pub fn hour(&self) -> &str {
        &self.hour
    }
}
