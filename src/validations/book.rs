use crate::{routes::NewBookData, validations::author::ValidatedAuthorName};

pub struct NewBook {
    pub title: ValidatedBookTitle,
    pub author: ValidatedAuthorName,
    pub genre: ValidatedBookGenre,
}

impl TryFrom<NewBookData> for NewBook {
    type Error = String;

    fn try_from(value: NewBookData) -> Result<Self, Self::Error> {
        let title = ValidatedBookTitle::new(value.title)?;
        let author = ValidatedAuthorName::new(value.author)?;
        let genre = ValidatedBookGenre::new(value.genre)?;

        Ok(Self {
            title,
            author,
            genre,
        })
    }
}

pub struct ValidatedBookTitle(String);

impl ValidatedBookTitle {
    pub fn new(title: String) -> Result<Self, String> {
        let is_empty_or_whitespace = title.trim().is_empty();
        let size_too_big = title.chars().count() > 256;

        if is_empty_or_whitespace || size_too_big {
            Err(format!("'{}' is not a valid author title.", title))
        } else {
            Ok(Self(title))
        }
    }
}

impl AsRef<str> for ValidatedBookTitle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct ValidatedBookGenre(String);

impl ValidatedBookGenre {
    fn new(genre: String) -> Result<Self, String> {
        let is_empty_or_whitespace = genre.trim().is_empty();
        let size_too_big = genre.chars().count() > 80;

        if is_empty_or_whitespace || size_too_big {
            Err(format!("'{}' is not a valid author genre.", genre))
        } else {
            Ok(Self(genre))
        }
    }
}

impl AsRef<str> for ValidatedBookGenre {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_title() {
        let title = String::from("Lord of the Rings");
        assert!(ValidatedBookTitle::new(title).is_ok());
    }

    #[test]
    fn empty_title() {
        let title = String::from("");
        assert!(ValidatedBookTitle::new(title).is_err());
    }

    #[test]
    fn whitespace_only_title() {
        let title = String::from(" ");
        assert!(ValidatedBookTitle::new(title).is_err());
    }

    #[test]
    fn max_length_title() {
        let title = "a".repeat(256);
        assert!(ValidatedBookTitle::new(title).is_ok());
    }

    #[test]
    fn too_long_tile() {
        let title = "a".repeat(257);
        assert!(ValidatedBookTitle::new(title).is_err());
    }

    #[test]
    fn valid_genre() {
        let genre = String::from("Fiction");
        assert!(ValidatedBookGenre::new(genre).is_ok());
    }

    #[test]
    fn empty_genre() {
        let genre = String::from("");
        assert!(ValidatedBookGenre::new(genre).is_err());
    }

    #[test]
    fn whitespace_only_genre() {
        let genre = String::from(" ");
        assert!(ValidatedBookGenre::new(genre).is_err());
    }

    #[test]
    fn max_length_genre() {
        let genre = "a".repeat(80);
        assert!(ValidatedBookGenre::new(genre).is_ok());
    }

    #[test]
    fn too_long_genre() {
        let genre = "a".repeat(81);
        assert!(ValidatedBookGenre::new(genre).is_err());
    }

    #[test]
    fn new_book_success() {
        let data = NewBookData {
            title: String::from("Pride and Prejudice"),
            author: String::from("Jane Austen"),
            genre: String::from("British"),
        };
        assert!(NewBook::try_from(data).is_ok());
    }

    #[test]
    fn new_book_failure() {
        let data = NewBookData {
            title: String::from("Pride \\(and) Prejudice"),
            author: String::from(""),
            genre: String::from("Britisn"),
        };
        assert!(NewBook::try_from(data).is_err());
    }
}
