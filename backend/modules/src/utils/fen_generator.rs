use rand::seq::SliceRandom;
use rand::thread_rng;

pub enum Variant {
    Standard,
    Chess960,
    ThreeCheck,
}

pub fn generate_starting_fen(variant: Variant) -> String {
    match variant {
        Variant::Standard => {
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
        }
        Variant::Chess960 => generate_chess960_fen(),
        Variant::ThreeCheck => {
            // Same as standard, but rules engine will track checks
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
        }
    }
}

fn generate_chess960_fen() -> String {
    let mut rng = thread_rng();
    let mut back_rank = [' '; 8];
    let mut positions: Vec<usize> = (0..8).collect();

    // Place bishops on opposite color squares
    let white_squares: Vec<usize> = positions.iter().cloned().filter(|&i| i % 2 == 0).collect();
    let black_squares: Vec<usize> = positions.iter().cloned().filter(|&i| i % 2 == 1).collect();
    let b1 = *white_squares.choose(&mut rng).unwrap();
    back_rank[b1] = 'B';
    positions.retain(|&x| x != b1);

    let b2 = *black_squares.choose(&mut rng).unwrap();
    back_rank[b2] = 'B';
    positions.retain(|&x| x != b2);

    // Place queen
    let q = *positions.choose(&mut rng).unwrap();
    back_rank[q] = 'Q';
    positions.retain(|&x| x != q);

    // Place knights
    for _ in 0..2 {
        let n = *positions.choose(&mut rng).unwrap();
        back_rank[n] = 'N';
        positions.retain(|&x| x != n);
    }

    // Place rooks and king (king must be between rooks)
    positions.sort();
    let (r1, r2, k) = (positions[0], positions[2], positions[1]);
    back_rank[r1] = 'R';
    back_rank[k] = 'K';
    back_rank[r2] = 'R';

    let back_rank_str: String = back_rank.iter().collect();
    let castling_rights = format!("{}{}{}{}",
        if r1 < k { (b'A' + r1 as u8) as char } else { (b'A' + r2 as u8) as char },
        if r2 < k { (b'A' + r2 as u8) as char } else { (b'A' + r1 as u8) as char },
        if r1 < k { (b'a' + r1 as u8) as char } else { (b'a' + r2 as u8) as char },
        if r2 < k { (b'a' + r2 as u8) as char } else { (b'a' + r1 as u8) as char },
    );

    format!("{}/pppppppp/8/8/8/8/PPPPPPPP/{} w {} - 0 1",
        back_rank_str.to_lowercase(),
        back_rank_str,
        castling_rights
    )
}
