pub type Square = Option<Piece>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color { White, Black }

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceKind {
    King, Queen, Rook, Bishop, Knight, Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

pub struct BoardState {
    pub board: [[Square; 8]; 8],
    pub to_move: Color,
    pub castling_rights: String,
    pub en_passant: Option<(usize, usize)>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}