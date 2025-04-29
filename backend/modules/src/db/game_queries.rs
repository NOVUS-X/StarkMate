use uuid::Uuid;
use sqlx::PgPool;

pub async fn insert_game(
    pool: &PgPool,
    white: &str,
    black: &str,
    variant: &str,
    fen: &str,
    rated: bool,
) -> Result<String, sqlx::Error> {
    let game_id = Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO games (id, white_player, black_player, variant, fen, rated)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        game_id,
        white,
        black,
        variant,
        fen,
        rated,
    )
    .execute(pool)
    .await?;

    Ok(game_id)
}
