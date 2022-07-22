use super::traits::Serializable;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub struct RepoFile<T>
where
    T: Serializable,
{
    elems: Vec<T>,
    filename: String,
}

impl<T> RepoFile<T>
where
    T: Serializable,
{
    pub fn new(filename: &str) -> Self {
        Self {
            elems: vec![],
            filename: filename.to_string(),
        }
    }

    pub fn load_from_file(&mut self) {
        let f = File::open(&self.filename);

        match f {
            Result::Ok(_) => (),
            Result::Err(_) => {
                return;
            }
        }

        let f = BufReader::new(f.unwrap());

        for line in f.lines() {
            let line = line.expect("Unable to read line.");
            self.elems.push(T::from_csv_to_obj(&line));
        }
    }

    pub fn save_to_file(&self) {
        let mut f = File::create(&self.filename).expect("Unable to create file.");

        for i in &self.elems {
            write!(f, "{}\n", i.to_csv());
        }
    }

    pub fn add_elem(&mut self, elem: T) -> Result<(), String> {
        self.elems.push(elem);
        Ok(())
    }

    pub fn remove_elem(&mut self, index: usize) -> Result<(), String> {
        if index > 0 && index < self.elems.len() {
            self.elems.remove(index);
            Ok(())
        } else {
            Err("Index out of bounds".into())
        }
    }

    pub fn update_elem(&mut self, index: usize, elem: T) -> Result<(), String> {
        let existing_elem = self.elems.get_mut(index);

        match existing_elem {
            None => Err("Index out of bounds".into()),
            Some(x) => {
                *x = elem;
                Ok(())
            }
        }
    }

    pub fn get_all(&self) -> &Vec<T> {
        &self.elems
    }

    pub fn get_elem(&self, index: usize) -> Result<&T, String> {
        let existing_elem = self.elems.get(index);

        match existing_elem {
            None => Err("Index out of bounds".into()),
            Some(x) => Ok(x),
        }
    }
}
