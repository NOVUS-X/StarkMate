pub mod players{
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct NewPlayer{
        pub username: String,
        pub email: String,
        pub password: String,
        pub real_name: String
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct UpdatePlayer{
        pub username: Option<String>,
        pub real_name: Option<String>,
        pub biography: Option<String>,
        pub country: Option<String>,
        pub flair: Option<String>,
        pub location: Option<String>,
        pub fide_rating: Option<i32>,
        pub social_links: Option<Vec<String>>
    }
}