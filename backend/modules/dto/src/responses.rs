use utoipa::ToSchema;

use crate::players::{DisplayPlayer, UpdatedPlayer};

#[derive(Debug, ToSchema)]
pub struct PlayerAddedBody {
    pub player: DisplayPlayer,
}

#[derive(Debug, ToSchema)]
pub struct DeletedBody {}

#[derive(Debug, ToSchema)]
pub struct PlayerUpdatedBody {
    pub player: UpdatedPlayer,
}

#[derive(Debug, ToSchema)]
pub struct PlayerAdded {
    #[schema(example = "New player added")]
    pub message: String,
    pub body: PlayerAddedBody,
}

#[derive(Debug, ToSchema)]
pub struct PlayerFound {
    #[schema(example = "Player found")]
    pub message: String,
    pub body: PlayerAddedBody,
}

#[derive(Debug, ToSchema)]
pub struct PlayerUpdated {
    #[schema(example = "Player updated")]
    pub message: String,
    pub body: PlayerUpdatedBody,
}

#[derive(Debug, ToSchema)]
pub struct PlayerDeleted{
    #[schema(example = "Player deleted")]
    pub  message: String,
    pub body: DeletedBody
}

#[derive(Debug, ToSchema)]
pub struct InvalidCredentialsResponse {
    #[schema(example = "Invalid credentials")]
    pub error: String,
    #[schema(example = 400)]
    pub code: i32,
}

#[derive(Debug, ToSchema)]
pub struct NotFoundResponse {
    #[schema(example = "Player not found")]
    pub error: String,
    #[schema(example = 404)]
    pub code: i32,
}
