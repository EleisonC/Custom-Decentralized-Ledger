use validator::validate_email;

#[derive(Clone, Debug)]
pub struct Email(String);


impl Email {
    pub fn parse(email: String) -> Result<Self, String> {
        if !validate_email(&email) {
            return Err("Invalid email address".to_string());
        } else {
            Ok(Self(email))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_validates_emails() {
        assert!(Email::parse("example@example.com".to_string()).is_ok());
        assert!(Email::parse("invalid-email".to_string()).is_err());
    }

    #[test]
    fn it_converts_emails_to_strings() {
        let email = Email::parse("example@example.com".to_string()).unwrap();
        assert_eq!(email.as_ref(), "example@example.com");
    }
}


