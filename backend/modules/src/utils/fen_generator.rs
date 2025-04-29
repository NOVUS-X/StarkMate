// src/utils/fen_generator.rs

use rand::seq::SliceRandom;

pub fn generate_fen(variant: &str) -> String {
    match variant {
        "chess960" => generate_chess960_fen(),
        _ => "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
    }
}

fn generate_chess960_fen() -> String {
    let mut back_rank = ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'];
    let mut rng = rand::thread_rng();
    back_rank.shuffle(&mut rng);
    // Use placeholder randomization here
    format!(
        "{}pppppppp/8/8/8/8/PPPPPPPP/{} w KQkq - 0 1",
        back_rank.iter().collect::<String>().to_lowercase(),
        back_rank.iter().collect::<String>()
    )
}
