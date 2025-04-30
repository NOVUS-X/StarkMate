use crate::helper::password;
use db::db::db::get_db;
use dto::players::{NewPlayer, UpdatePlayer};
use error::error::ApiError;
use entity::player;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

async fn is_username_taken(username: String) -> bool {
    let db = get_db().await;

    let user = player::Entity::find()
        .filter(player::Column::Username.eq(username))
        .one(&db)
        .await
        .unwrap();

    user.is_some()
}

async fn is_email_taken(email: String) -> bool {
    let db = get_db().await;

    let user = player::Entity::find()
        .filter(player::Column::Email.eq(email))
        .one(&db)
        .await
        .unwrap();

    user.is_some()
}

pub async fn get_player_by_id(id: Uuid) -> Result<player::Model, ApiError>{
    let db = get_db().await;

    let user = player::Entity::find()
        .filter(player::Column::Id.eq(id))
        .filter(player::Column::IsEnabled.eq(true))
        .one(&db)
        .await
        .unwrap();

    match user {
        Some(usr) => Ok(usr),
        None => Err(ApiError::NotFound(format!("Player {}", id).to_string()))
    }
}


pub async fn add_player(payload: NewPlayer) -> Result<player::Model, ApiError> {
    let email_available = is_email_taken(payload.email.clone()).await;
    let username_available = is_username_taken(payload.username.clone()).await;
    if email_available && username_available {
        return Err(ApiError::InvalidCredentials);
    }
    let new_player = player::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(payload.username),
        email: Set(payload.email),
        password_hash: Set(password::hash_password(&payload.password).into_bytes()),
        real_name: Set(payload.real_name),
        ..Default::default()
    };

    let db = get_db().await;
    let new_player = new_player.insert(&db).await;

    match new_player {
        Ok(plyr) => Ok(plyr),
        Err(err) => Err(ApiError::DatabaseError(err)),
    }
}



pub async fn update_player(id: Uuid, payload: UpdatePlayer) -> Result<player::Model, ApiError> {
    let db = get_db().await;
    let existing_player = get_player_by_id(id).await?;  

    let mut active_model: player::ActiveModel = existing_player.into();
    

    if let Some(biography) = payload.biography {
        active_model.biography = Set(Some(biography));
    }
    if let Some(real_name) = payload.real_name {
        active_model.real_name = Set(real_name);
    }
    if let Some(country) = payload.country {
        active_model.country = Set(Some(country));
    }
    if let Some(flair) = payload.flair {
        active_model.flair = Set(Some(flair));
    }
    if let Some(location) = payload.location {
        active_model.location = Set(Some(location));
    }
    if let Some(fide_rating) = payload.fide_rating {
        active_model.fide_rating = Set(Some(fide_rating));
    }
    if let Some(social_links) = payload.social_links {
        active_model.social_links = Set(Some(social_links));
    }

    let updated_player = active_model.update(&db).await.map_err(ApiError::DatabaseError)?;
    
    Ok(updated_player)
}


pub async  fn delete_player(id: Uuid) -> Result<(), ApiError> {
    let db = get_db().await;
    let existing_player = get_player_by_id(id).await?;

    let mut active_model: player::ActiveModel = existing_player.into();

    active_model.is_enabled = Set(false);

    active_model.update(&db).await.map_err(ApiError::DatabaseError)?;

    Ok(())
}