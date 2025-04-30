use actix_web::{test, App};
use starkmate_backend::routes::games::create_game;
use starkmate_backend::models::game::{CreateGameRequest, GameVariant, TimeControl};
use sqlx::{PgPool, postgres::PgPoolOptions};

#[actix_rt::test]
async fn test_create_standard_game_two_players() {
    let pool = setup_test_db().await;

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(create_game)
    ).await;

    let payload = CreateGameRequest {
        players: vec!["alice".to_string(), "bob".to_string()],
        variant: GameVariant::Standard,
        time_control: TimeControl {
            initial: 600,
            increment: 5,
            delay_type: None,
        },
        rated: true,
    };

    let req = test::TestRequest::post()
        .uri("/games/new")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_create_chess960_game_with_ai() {
    let pool = setup_test_db().await;

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(create_game)
    ).await;

    let payload = CreateGameRequest {
        players: vec!["human_player".to_string()],
        variant: GameVariant::Chess960,
        time_control: TimeControl {
            initial: 300,
            increment: 2,
            delay_type: None,
        },
        rated: false,
    };

    let req = test::TestRequest::post()
        .uri("/games/new")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

// Helps to test DB connection (can be from test-specific db)
async fn setup_test_db() -> PgPool {
    PgPoolOptions::new()
        .connect("postgres://user:pass@localhost/starkmate_test")
        .await
        .expect("Test DB connection failed")
}
