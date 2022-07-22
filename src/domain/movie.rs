use crate::repo::traits::Serializable;
use crate::validators::common_validators::id_validator;
use crate::validators::movie_validator::{
    movie_validator, price_validator, release_year_validator, title_validator,
};

#[derive(Debug)]
pub struct Movie {
    id: u32,
    title: String,
    release_year: u32,
    price: u32,
    in_program: bool,
}

impl Serializable for Movie {
    fn to_csv(&self) -> String {
        format!(
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
            self.id, self.title, self.release_year, self.price, self.in_program,
        )
    }

    fn from_csv_to_obj(s: &str) -> Movie {
        let mut parts = s.trim().trim_matches('"').split("\",\"");

        let id: u32 = parts.next().unwrap_or("0").parse().unwrap_or(0);

        let title: String = parts.next().unwrap_or("placeholder").to_string();

        let release_year: u32 = parts.next().unwrap_or("1926").parse().unwrap_or(1926);

        let price: u32 = parts.next().unwrap_or("0").parse().unwrap_or(0);

        let in_program: bool = parts.next().unwrap_or("false").parse().unwrap_or(false);

        Movie::new(id, &title, release_year, price, in_program).unwrap()
    }
}

impl Movie {
    pub fn new(
        id: u32,
        title: &str,
        release_year: u32,
        price: u32,
        in_program: bool,
    ) -> Result<Movie, String> {
        let movie = Movie {
            id,
            release_year,
            price,
            in_program,
            title: title.to_string(),
        };

        match movie_validator(&movie) {
            Result::Ok(_) => Result::Ok(movie),
            Result::Err(err) => Result::Err(err),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn release_year(&self) -> u32 {
        self.release_year
    }

    pub fn price(&self) -> u32 {
        self.price
    }

    pub fn in_program(&self) -> bool {
        self.in_program
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

    pub fn set_title(&mut self, title: &str) -> Result<(), String> {
        match title_validator(title) {
            Result::Ok(_) => {
                self.title = title.to_string();
                Result::Ok(())
            }
            Result::Err(err) => Result::Err(err),
        }
    }

    pub fn set_release_year(&mut self, release_year: u32) -> Result<(), String> {
        match release_year_validator(release_year) {
            Result::Ok(_) => {
                self.release_year = release_year;
                Result::Ok(())
            }
            Result::Err(err) => Result::Err(err),
        }
    }

    pub fn set_price(&mut self, price: u32) -> Result<(), String> {
        match price_validator(price) {
            Result::Ok(_) => {
                self.price = price;
                Result::Ok(())
            }
            Result::Err(err) => Result::Err(err),
        }
    }

    pub fn set_in_program(&mut self, in_program: bool) -> Result<(), String> {
        self.in_program = in_program;
        Result::Ok(())
    }
}
