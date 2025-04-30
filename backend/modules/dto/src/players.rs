use entity::player::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewPlayer {
    pub username: String,
    pub email: String,
    pub password: String,
    pub real_name: String,
}

impl NewPlayer {
    pub fn test_player() -> Self {
        let rnd: i32 = rand::random();
        Self{
            username: format!("Player {}", rnd),
            email: format!("player{}@gmail.com", rnd),
            password: format!("PasswordIsVeryStrong"),
            real_name: format!("A new player")
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePlayer {
    pub username: Option<String>,
    pub real_name: Option<String>,
    pub biography: Option<String>,
    pub country: Option<String>,
    pub flair: Option<String>,
    pub location: Option<String>,
    pub fide_rating: Option<i32>,
    pub social_links: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct DisplayPlayer {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub biography: Option<String>,
    pub country: Option<String>,
    pub flair: Option<String>,
    pub real_name: String,
}

#[derive(Serialize)]
pub struct UpdatedPlayer {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub biography: Option<String>,
    pub country: Option<String>,
    pub flair: Option<String>,
    pub real_name: String,
    pub location: Option<String>,
    pub fide_rating: Option<i32>,
    pub social_links: Option<Vec<String>>,
}

impl From<Model> for UpdatedPlayer {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            biography: value.biography,
            country: value.country,
            flair: value.flair,
            real_name: value.real_name,
            location: value.location,
            fide_rating: value.fide_rating,
            social_links: value.social_links,
        }
    }
}

impl From<Model> for DisplayPlayer {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            biography: value.biography,
            country: value.country,
            flair: value.flair,
            real_name: value.real_name,
        }
    }
}
