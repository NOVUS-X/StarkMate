use std::time::{Duration, Instant};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub real_lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayTime {
    pub total: u64,
    pub tv: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emails;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithPerfs;

#[derive(Debug, Serialize, Deserialize)]
pub struct WithPerfsAndEmails {
    pub user: UserWithPerfs,
    pub emails: Emails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotpToken(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDelete {
    pub requested: Instant,
    pub done: bool,
}

impl PlayTime {
    pub fn total_duration(&self) -> Duration {
        Duration::from_secs(self.total)
    }
    
    pub fn tv_duration(&self) -> Duration {
        Duration::from_secs(self.tv)
    }
    
    pub fn non_empty_tv_duration(&self) -> Option<Duration> {
        if self.tv > 0 {
            Some(self.tv_duration())
        } else {
            None
        }
    }
}

pub mod name_rules {
    use regex::Regex;
    
    lazy_static::lazy_static! {
        pub static ref NEW_USERNAME_REGEX: Regex = Regex::new(r"(?i)[a-z][a-z0-9_-]{0,28}[a-z0-9]").unwrap();
        pub static ref NEW_USERNAME_PREFIX: Regex = Regex::new(r"(?i)^[a-z].*").unwrap();
        pub static ref NEW_USERNAME_SUFFIX: Regex = Regex::new(r"(?i).*[a-z0-9]$").unwrap();
        pub static ref NEW_USERNAME_CHARS: Regex = Regex::new(r"(?i)^[a-z0-9_-]*$").unwrap();
        pub static ref NEW_USERNAME_LETTERS: Regex = Regex::new(r"(?i)^([a-z0-9][_-]?)+$").unwrap();
    }
}
