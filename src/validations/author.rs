use crate::routes::NewAuthorData;

pub struct NewAuthor {
    pub name: ValidatedAuthorName,
    pub nationality: ValidatedAuthorNationality,
}

impl TryFrom<NewAuthorData> for NewAuthor {
    type Error = String;

    fn try_from(value: NewAuthorData) -> Result<Self, Self::Error> {
        let name = ValidatedAuthorName::new(value.name)?;
        let nationality = ValidatedAuthorNationality::new(value.nationality)?;
        Ok(Self { name, nationality })
    }
}

pub struct ValidatedAuthorName(String);

impl ValidatedAuthorName {
    pub fn new(value: String) -> Result<Self, String> {
        let is_empty_or_whitespace = value.trim().is_empty();
        let size_too_big = value.chars().count() > 256;

        if is_empty_or_whitespace || size_too_big {
            Err(format!("'{}' is not a valid author name.", value))
        } else {
            Ok(Self(value))
        }
    }
}

impl AsRef<str> for ValidatedAuthorName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct ValidatedAuthorNationality(String);

impl ValidatedAuthorNationality {
    fn new(value: String) -> Result<Self, String> {
        let is_empty_or_whitespace = value.trim().is_empty();
        let size_too_big = value.chars().count() > 80;

        if is_empty_or_whitespace || size_too_big {
            Err(format!("'{}' is not a valid author nationality.", value))
        } else {
            Ok(Self(value))
        }
    }
}

impl AsRef<str> for ValidatedAuthorNationality {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_name() {
        let name = String::from("John Doe");
        assert!(ValidatedAuthorName::new(name).is_ok());
    }

    #[test]
    fn empty_name() {
        let name = String::from("");
        assert!(ValidatedAuthorName::new(name).is_err());
    }

    #[test]
    fn whitespace_only_name() {
        let name = String::from(" ");
        assert!(ValidatedAuthorName::new(name).is_err());
    }

    #[test]
    fn max_length_name() {
        let name = "a".repeat(256);
        assert!(ValidatedAuthorName::new(name).is_ok());
    }

    #[test]
    fn too_long_name() {
        let name = "a".repeat(257);
        assert!(ValidatedAuthorName::new(name).is_err());
    }

    #[test]
    fn valid_nationality() {
        let nationality = String::from("Brazilian");
        assert!(ValidatedAuthorNationality::new(nationality).is_ok());
    }

    #[test]
    fn empty_nationality() {
        let nationality = String::from("");
        assert!(ValidatedAuthorNationality::new(nationality).is_err());
    }

    #[test]
    fn whitespace_only_nationality() {
        let nationality = String::from(" ");
        assert!(ValidatedAuthorNationality::new(nationality).is_err());
    }

    #[test]
    fn max_length_nationality() {
        let nationality = "a".repeat(80);
        assert!(ValidatedAuthorNationality::new(nationality).is_ok());
    }

    #[test]
    fn too_long_nationality() {
        let nationality = "a".repeat(81);
        assert!(ValidatedAuthorNationality::new(nationality).is_err());
    }

    #[test]
    fn new_author_success() {
        let data = NewAuthorData {
            name: String::from("Jane Doe"),
            nationality: String::from("American"),
        };
        assert!(NewAuthor::try_from(data).is_ok());
    }

    #[test]
    fn new_author_failure() {
        let data = NewAuthorData {
            name: String::from(""), // Invalid name
            nationality: String::from("American"),
        };
        assert!(NewAuthor::try_from(data).is_err());
    }
}
