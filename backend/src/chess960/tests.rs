#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[tokio::test]
    async fn test_get_position_by_number() {
        let app = test::init_service(
            App::new().configure(configure_routes)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/chess960/position?number=1")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[tokio::test]
    async fn test_get_random_position() {
        let app = test::init_service(
            App::new().configure(configure_routes)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/chess960/position?random=true")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[tokio::test]
    async fn test_get_fen() {
        let app = test::init_service(
            App::new().configure(configure_routes)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/chess960/fen/1")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[tokio::test]
    async fn test_verify_fen() {
        let app = test::init_service(
            App::new().configure(configure_routes)
        ).await;

        let library = Chess960Generator::generate_all_positions();
        let valid_fen = &library.positions.get(&1).unwrap().fen;

        let req = test::TestRequest::post()
            .uri("/api/chess960/verify")
            .set_json(&FenVerifyRequest {
                fen: valid_fen.clone(),
            })
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[test]
    fn test_chess960_rules_compliance() {
        let library = Chess960Generator::generate_all_positions();
        
        for (id, position) in &library.positions {
            // Test king between rooks
            let king_pos = position.white_king_pos;
            let [rook1, rook2] = position.white_rook_positions;
            assert!(rook1 < king_pos && king_pos < rook2, 
                "Position {}: King not between rooks", id);
            
            // Test bishops on opposite colors
            let [bishop1, bishop2] = position.white_bishop_positions;
            assert!((bishop1 + bishop2) % 2 == 1, 
                "Position {}: Bishops on same color", id);
            
            // Test FEN format
            let fen_parts: Vec<&str> = position.fen.split(' ').collect();
            assert_eq!(fen_parts.len(), 6, "Position {}: Invalid FEN format", id);
        }
    }
}