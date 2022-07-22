use crate::domain::movie::Movie;
use crate::repo::repo_file::RepoFile;

pub struct MovieService<'a> {
    movies: &'a mut RepoFile<Movie>,
}

impl<'a> MovieService<'a> {
    pub fn new(repo: &'a mut RepoFile<Movie>) -> Self {
        MovieService { movies: repo }
    }

    pub fn get_all(&self) -> &Vec<Movie> {
        self.movies.get_all()
    }

    pub fn get(&self, index: usize) -> Result<&Movie, String> {
        self.movies.get_elem(index)
    }

    pub fn add(&mut self, new_movie: Movie) -> Result<(), String> {
        for movie in self.movies.get_all() {
            if movie.title() == new_movie.title() {
                return Err("A movie with this title already exists".into());
            } else if movie.id() == new_movie.id() {
                return Err("A movie with this ID already exists.".into());
            }
        }

        self.movies.add_elem(new_movie)
    }

    pub fn update(&mut self, index: usize, new_movie: Movie) -> Result<(), String> {
        let mut found = false;

        for (i, movie) in self.movies.get_all().iter().enumerate() {
            if i != index {
                if new_movie.title() == movie.title() {
                    return Err("A different movie with that title already exists".into());
                } else if new_movie.id() == movie.id() {
                    return Err("A different movie with that ID already exists".into());
                }

                found = true;
            }
        }

        if found == true {
            self.movies.update_elem(index, new_movie)
        } else {
            Err("There is no movie with that ID".into())
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<(), String> {
        self.movies.remove_elem(index)
    }
}
