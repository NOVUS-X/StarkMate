#[cfg(test)]
mod tests {
    use actix_web::{App, dev::Service, http::StatusCode, test, web};
    use dto::players::{InvalidPlayer, NewPlayer};

    use crate::players::add_player;

    #[actix_web::test]
    async fn test_index_post_no_body() {
        let app =
            test::init_service(App::new().service(web::scope("/v1/players").service(add_player)))
                .await;
        let req = test::TestRequest::post().uri("/v1/players").to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }


    #[actix_web::test]
    async fn test_index_post_with_body() {
        let app =
            test::init_service(App::new().service(web::scope("/v1/players").service(add_player)))
                .await;
        let req = test::TestRequest::post().uri("/v1/players").set_json(NewPlayer::test_player()).to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_index_post_with_invalid_username() {
        let app =
            test::init_service(App::new().service(web::scope("/v1/players").service(add_player)))
                .await;
        let req = test::TestRequest::post().uri("/v1/players").set_json(NewPlayer::invalid_player(InvalidPlayer::Username)).to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }


    #[actix_web::test]
    async fn test_index_post_with_invalid_email() {
        let app =
            test::init_service(App::new().service(web::scope("/v1/players").service(add_player)))
                .await;
        let req = test::TestRequest::post().uri("/v1/players").set_json(NewPlayer::invalid_player(InvalidPlayer::Email)).to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }


    #[actix_web::test]
    async fn test_index_post_with_invalid_password() {
        let app =
            test::init_service(App::new().service(web::scope("/v1/players").service(add_player)))
                .await;
        let req = test::TestRequest::post().uri("/v1/players").set_json(NewPlayer::invalid_player(InvalidPlayer::Password)).to_request();
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }
}
