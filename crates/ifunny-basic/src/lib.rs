#![allow(unused)]
#![forbid(unsafe_code)]

use easy_base64::encode;
use sha1_smol::Sha1;
use sha256::digest;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum BasicTokenLength {
    Basic112 = 112,
    Basic156 = 156,
}

impl TryFrom<usize> for BasicTokenLength {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            112 => Ok(BasicTokenLength::Basic112),
            156 => Ok(BasicTokenLength::Basic156),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BasicToken(String);

impl BasicToken {
    pub const CLIENT_ID: &str = "MsOIJ39Q28";
    pub const CLIENT_SECRET: &str = "PTDc3H8a)Vi=UYap";
    pub const DEFAULT_LENGTH: BasicTokenLength = BasicTokenLength::Basic112;

    pub fn new(client_id: &str, client_secret: &str, length: BasicTokenLength) -> BasicToken {
        let id = Uuid::new_v4().as_simple().to_string(); // Hypens removed

        let hex = match length {
            BasicTokenLength::Basic112 => id,
            BasicTokenLength::Basic156 => sha256::digest(&id), // Hash with Sha256!
        }
        .to_uppercase(); // Convert to uppercase

        let prefix = format!("{hex}_{client_id}:");

        let suffix = {
            let mut hash = Sha1::new();
            hash.update(format!("{hex}:{client_id}:{client_secret}").as_bytes());
            format!("{}", hash.digest())
        };

        Self(encode(format!("{prefix}{suffix}").as_bytes()))
    }
}

impl std::fmt::Display for BasicToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for BasicToken {
    fn default() -> Self {
        Self::new(Self::CLIENT_ID, Self::CLIENT_SECRET, Self::DEFAULT_LENGTH)
    }
}

impl From<String> for BasicToken {
    fn from(token: String) -> Self {
        BasicToken(token)
    }
}

impl From<BasicToken> for String {
    fn from(token: BasicToken) -> Self {
        token.0
    }
}

impl From<&str> for BasicToken {
    fn from(token: &str) -> Self {
        token.to_owned().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_makes_112() {
        let basic = BasicToken::new(
            BasicToken::CLIENT_ID,
            BasicToken::CLIENT_SECRET,
            BasicTokenLength::Basic112,
        );

        assert_eq!(basic.0.len(), 112);
    }

    #[test]
    fn new_makes_156() {
        let basic = BasicToken::new("client_id", "client_secret", BasicTokenLength::Basic156);

        assert_eq!(basic.0.len(), 156);
    }

    #[test]
    fn default_is_unique() {
        let basic = BasicToken::default();
        let basic2 = BasicToken::default();

        assert_ne!(basic, basic2);
    }

    #[test]
    fn default_is_112() {
        let basic = BasicToken::default();
        assert_eq!(basic.0.len(), 112);
    }

    #[test]
    fn clones_are_equal() {
        let basic = BasicToken::default();
        let basic2 = basic.clone();

        assert_eq!(basic, basic2)
    }

    #[test]
    fn debug_includes_token() {
        let tokens = vec!["Yinkies", "Foo", "Bar"];

        for token in tokens {
            let basic = BasicToken::from(token);
            assert_eq!(format!("{basic:?}"), format!("BasicToken(\"{basic}\")"))
        }
    }

    #[test]
    fn test_display() {
        let basic = BasicToken::default();
        let basic2 = format!("{}", basic);

        assert_eq!(basic2, basic.0)
    }

    #[test]
    fn test_from_string() {
        assert_eq!(BasicToken::from("Yinkies".to_string()).0, "Yinkies");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(BasicToken::from("Yinkies").0, "Yinkies");
    }

    #[test]
    fn test_into_string() {
        let basic: String = BasicToken::from("Yinkies").into();
        assert_eq!(basic, "Yinkies");
    }

    #[test]
    fn basic_length_try_from_usize_112() {
        let basic = BasicTokenLength::try_from(112);

        assert!(basic.is_ok());
    }

    #[test]
    fn basic_length_try_from_usize_156() {
        let basic = BasicTokenLength::try_from(156);

        assert!(basic.is_ok());
    }

    #[test]
    fn basic_length_try_from_usize_0() {
        let basic = BasicTokenLength::try_from(0);

        assert!(basic.is_err());
    }

    #[test]
    fn basic_length_clone() {
        let basic = BasicTokenLength::Basic112;
        let basic2 = basic.clone();

        assert_eq!(basic, basic2);
    }

    #[test]
    fn basic_length_display() {
        let basic = BasicTokenLength::Basic112;
        let basic2 = format!("{basic:?}");

        assert_eq!(basic2, "Basic112");
    }
}
