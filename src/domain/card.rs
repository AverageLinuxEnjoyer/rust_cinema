use crate::validators::card_validator::card_validator;

pub struct Card {
    id: u32,
    first_name: String,
    last_name: String,
    cnp: String,
    birthday: String,
    registration_date: String,
    points: u32,
}

impl Card {
    pub fn new(
        id: u32,
        first_name: &str,
        last_name: &str,
        cnp: &str,
        birthday: &str,
        registration_date: &str,
        points: u32,
    ) -> Result<Card, String> {
        let card = Card {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            cnp: cnp.to_string(),
            birthday: birthday.to_string(),
            registration_date: registration_date.to_string(),
            points,
        };

        match card_validator(&card) {
            Result::Ok(_) => Result::Ok(card),
            Result::Err(err) => Result::Err(err),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn cnp(&self) -> &str {
        &self.cnp
    }

    pub fn birthday(&self) -> &str {
        &self.birthday
    }

    pub fn registration_date(&self) -> &str {
        &self.registration_date
    }

    pub fn points(&self) -> u32 {
        self.points
    }
}
