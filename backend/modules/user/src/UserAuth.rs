use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HashedPassword(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub id: String,
    pub bpass: HashedPassword,
    pub salt: Option<String>,
    pub sha512: Option<bool>,
}

impl AuthData {
    pub fn projection() -> Vec<&'static str> {
        vec!["bpass", "salt", "sha512"]
    }
}