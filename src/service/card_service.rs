use crate::domain::card::Card;
use crate::repo::repo_file::RepoFile;

pub struct CardService<'a> {
    cards: &'a mut RepoFile<Card>,
}

impl<'a> CardService<'a> {
    pub fn new(repo: &'a mut RepoFile<Card>) -> Self {
        CardService { cards: repo }
    }

    pub fn get_all(&self) -> &Vec<Card> {
        self.cards.get_all()
    }

    pub fn get(&self, index: usize) -> Result<&Card, String> {
        self.cards.get_elem(index)
    }

    pub fn add(&mut self, new_card: Card) -> Result<(), String> {
        for card in self.cards.get_all() {
            if new_card.cnp() == card.cnp() {
                return Err("A card with this CNP already exists.".into());
            } else if new_card.id() == card.id() {
                return Err("A card with this ID already exists".into());
            }
        }

        self.cards.add_elem(new_card)
    }

    pub fn update(&mut self, index: usize, new_card: Card) -> Result<(), String> {
        let mut found = false;
        for (i, card) in self.cards.get_all().iter().enumerate() {
            if i != index {
                if new_card.cnp() == card.cnp() {
                    return Err("A different card with that CNP already exists.".into());
                } else if new_card.id() == card.id() {
                    return Err("A different card with that ID already exists".into());
                }

                found = true;
            }
        }

        if found == true {
            self.cards.update_elem(index, new_card)
        } else {
            Err("There is no card with that ID.".into())
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<(), String> {
        self.cards.remove_elem(index)
    }
}
