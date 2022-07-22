mod domain;
mod repo;
mod service;
mod validators;

use domain::card::Card;
use domain::date::Date;
use domain::movie::Movie;
use repo::repo_file::RepoFile;
use repo::traits::Serializable;
use service::card_service::CardService;

fn main() {
    let mut repo = RepoFile::<Card>::new("cards.csv");
    repo.load_from_file();

    let mut service = CardService::new(&mut repo);

    service
        .add(
            Card::new(
                1,
                "Adrian",
                "Placinta",
                "1234567890123",
                Date::new(26, 4, 2000).unwrap_or_default(),
                Date::new(30, 5, 2003).unwrap_or_default(),
                0,
            )
            .unwrap(),
        )
        .unwrap();

    repo.save_to_file();
}
