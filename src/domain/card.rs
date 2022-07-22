use crate::domain::date::Date;
use crate::repo::traits::Serializable;
use crate::validators::card_validator::{
    card_validator, cnp_validator, first_name_validator, last_name_validator,
};
use crate::validators::{common_validators::id_validator, date_validators::date_validator};

#[derive(Clone)]
pub struct Card {
    id: u32,
    first_name: String,
    last_name: String,
    cnp: String,
    birthday: Date,
    registration_date: Date,
    points: u32,
}

impl Serializable for Card {
    fn to_csv(&self) -> String {
        format!(
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
            self.id,
            self.first_name,
            self.last_name,
            self.cnp,
            self.birthday.to_string(),
            self.registration_date.to_string(),
            self.points
        )
    }

    fn from_csv_to_obj(s: &str) -> Card {
        let mut parts = s.trim().trim_matches('"').split("\",\"");

        let id: u32 = parts.next().unwrap_or("0").parse().unwrap_or(0);

        let first_name: String = parts.next().unwrap_or("firstname").to_string();

        let last_name: String = parts.next().unwrap_or("lastname").to_string();

        let cnp: String = parts.next().unwrap_or("1234567890123").to_string();

        let birthday: String = parts.next().unwrap_or("01.01.2000").to_string();
        let birthday = Date::from_string(&birthday).unwrap_or_default();

        let registration_date: String = parts.next().unwrap_or("01.01.2000").to_string();
        let registration_date = Date::from_string(&registration_date).unwrap_or_default();

        let points: u32 = parts.next().unwrap_or("0").parse().unwrap_or(0);

        Card::new(
            id,
            &first_name,
            &last_name,
            &cnp,
            birthday,
            registration_date,
            points,
        )
        .unwrap()
    }
}

impl Card {
    pub fn new(
        id: u32,
        first_name: &str,
        last_name: &str,
        cnp: &str,
        birthday: Date,
        registration_date: Date,
        points: u32,
    ) -> Result<Card, String> {
        let card = Card {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            cnp: cnp.to_string(),
            birthday: birthday,
            registration_date: registration_date,
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

    pub fn birthday(&self) -> Date {
        self.birthday
    }

    pub fn registration_date(&self) -> Date {
        self.registration_date
    }

    pub fn points(&self) -> u32 {
        self.points
    }

    pub fn set_id(&mut self, id: u32) -> Result<(), String> {
        match id_validator(id) {
            Result::Ok(_) => {
                self.id = id;
                Result::Ok(())
            }
            Result::Err(err) => Result::Err(err),
        }
    }

    pub fn set_first_name(&mut self, first_name: &str) -> Result<(), String> {
        match first_name_validator(first_name) {
            Result::Ok(_) => {
                self.first_name = first_name.to_string();
                Result::Ok(())
            }
            Result::Err(err) => Result::Err(err),
        }
    }

    pub fn set_last_name(&mut self, last_name: &str) -> Result<(), String> {
        match last_name_validator(last_name) {
            Result::Ok(_) => {
                self.last_name = last_name.to_string();
                Result::Ok(())
            }
            Result::Err(err) => Result::Err(err),
        }
    }

    pub fn set_cnp(&mut self, cnp: &str) -> Result<(), String> {
        match cnp_validator(cnp) {
            Result::Ok(_) => {
                self.cnp = cnp.to_string();
                Result::Ok(())
            }
            Result::Err(err) => Result::Err(err),
        }
    }

    pub fn set_birthday(&mut self, birthday: Date) -> Result<(), String> {
        match date_validator(&birthday) {
            Result::Ok(_) => {
                self.birthday = birthday;
                Result::Ok(())
            }
            Result::Err(err) => Result::Err(err),
        }
    }

    pub fn set_registration_date(&mut self, registration_date: Date) -> Result<(), String> {
        match date_validator(&registration_date) {
            Result::Ok(_) => {
                self.registration_date = registration_date;
                Result::Ok(())
            }
            Result::Err(err) => Result::Err(err),
        }
    }

    pub fn set_points(&mut self, points: u32) -> Result<(), String> {
        self.points = points;

        Result::Ok(())
    }
}
