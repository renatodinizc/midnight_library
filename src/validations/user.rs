use crate::routes::NewUserData;
use regex::Regex;

pub struct NewUser {
    pub name: ValidatedUserName,
    pub email: ValidatedUserEmail,
}

impl TryFrom<NewUserData> for NewUser {
    type Error = String;

    fn try_from(value: NewUserData) -> Result<Self, Self::Error> {
        let name = ValidatedUserName::new(value.name)?;
        let email = ValidatedUserEmail::new(value.email)?;
        Ok(Self { name, email })
    }
}

pub struct ValidatedUserName(String);

impl ValidatedUserName {
    pub fn new(value: String) -> Result<Self, String> {
        let is_empty_or_whitespace = value.trim().is_empty();
        let size_too_big = value.chars().count() > 256;

        if is_empty_or_whitespace || size_too_big {
            Err(format!("'{}' is not a valid user name.", value))
        } else {
            Ok(Self(value))
        }
    }
}

impl AsRef<str> for ValidatedUserName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct ValidatedUserEmail(String);

impl ValidatedUserEmail {
    fn new(value: String) -> Result<Self, String> {
        let is_empty_or_whitespace = value.trim().is_empty();
        let size_too_big = value.chars().count() > 90;

        // https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address
        let email_regex = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])+)+$").unwrap();

        if is_empty_or_whitespace || size_too_big || !email_regex.is_match(&value) {
            Err(format!("'{}' is not a valid user email.", value))
        } else {
            Ok(Self(value))
        }
    }
}

impl AsRef<str> for ValidatedUserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_user_name() {
        let name = "John Doe".to_string();
        let user_name = ValidatedUserName::new(name);
        assert!(user_name.is_ok());
    }

    #[test]
    fn invalid_user_name_empty() {
        let name = "".to_string();
        let user_name = ValidatedUserName::new(name);
        assert!(user_name.is_err());
    }

    #[test]
    fn invalid_user_name_whitespace() {
        let name = "   ".to_string();
        let user_name = ValidatedUserName::new(name);
        assert!(user_name.is_err());
    }

    #[test]
    fn invalid_user_name_too_long() {
        let name = "a".repeat(257);
        let user_name = ValidatedUserName::new(name);
        assert!(user_name.is_err());
    }

    #[test]
    fn email_valid_standard() {
        let email = "test@example.com".to_string();
        assert!(ValidatedUserEmail::new(email).is_ok());
    }

    #[test]
    fn email_valid_subdomain() {
        let email = "test@sub.example.com".to_string();
        assert!(ValidatedUserEmail::new(email).is_ok());
    }

    #[test]
    fn email_valid_plus_tag() {
        let email = "test+tag@example.com".to_string();
        assert!(ValidatedUserEmail::new(email).is_ok());
    }

    #[test]
    fn email_valid_special_characters_local_part() {
        let email = "user.name+tag!#$%&'*+/=?^_`{|}~-@example.com".to_string();
        assert!(ValidatedUserEmail::new(email).is_ok());
    }

    #[test]
    fn email_valid_numeric_domain() {
        let email = "test@123.com".to_string();
        assert!(ValidatedUserEmail::new(email).is_ok());
    }

    #[test]
    fn email_valid_hyphen_in_domain() {
        let email = "test@example-domain.com".to_string();
        assert!(ValidatedUserEmail::new(email).is_ok());
    }

    #[test]
    fn email_invalid_missing_at_symbol() {
        let email = "testexample.com".to_string();
        assert!(ValidatedUserEmail::new(email).is_err());
    }

    #[test]
    fn email_invalid_empty() {
        let email = "".to_string();
        assert!(ValidatedUserEmail::new(email).is_err());
    }

    #[test]
    fn email_invalid_length() {
        let email = "a".repeat(90);
        assert!(ValidatedUserEmail::new(format!("{email}@example-domain.com")).is_err());
    }

    #[test]
    fn email_invalid_multiple_at_symbols() {
        let email = "test@example@com".to_string();
        assert!(ValidatedUserEmail::new(email).is_err());
    }

    #[test]
    fn email_invalid_missing_domain() {
        let email = "test@.com".to_string();
        assert!(ValidatedUserEmail::new(email).is_err());
    }

    #[test]
    fn email_invalid_missing_toplevel_domain() {
        let email = "test@example".to_string();
        assert!(ValidatedUserEmail::new(email).is_err());
    }

    #[test]
    fn email_invalid_spaces() {
        let email = "test @example.com".to_string();
        assert!(ValidatedUserEmail::new(email).is_err());
    }

    #[test]
    fn email_invalid_special_chars_in_domain() {
        let email = "test@example*domain.com".to_string();
        assert!(ValidatedUserEmail::new(email).is_err());
    }

    #[test]
    fn email_invalid_unicode_chars() {
        let email = "测试@example.com".to_string();
        assert!(ValidatedUserEmail::new(email).is_err());
    }

    #[test]
    fn create_new_user_valid() {
        let new_user_data = NewUserData {
            name: "John Doe".to_string(),
            email: "user@example.com".to_string(),
        };
        let new_user = NewUser::try_from(new_user_data);
        assert!(new_user.is_ok());
    }

    #[test]
    fn create_new_user_invalid_name() {
        let new_user_data = NewUserData {
            name: "".to_string(),
            email: "user@example.com".to_string(),
        };
        let new_user = NewUser::try_from(new_user_data);
        assert!(new_user.is_err());
    }

    #[test]
    fn create_new_user_invalid_email() {
        let new_user_data = NewUserData {
            name: "John Doe".to_string(),
            email: "invalid_email".to_string(),
        };
        let new_user = NewUser::try_from(new_user_data);
        assert!(new_user.is_err());
    }
}
