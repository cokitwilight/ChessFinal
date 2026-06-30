pub const COLORS: [Color; 2] = [Color::White, Color::Black];

pub const PIECE_TYPES: [PieceType; 6] = [
    PieceType::Pawn,
    PieceType::Knight,
    PieceType::Bishop,
    PieceType::Rook,
    PieceType::Queen,
    PieceType::King,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
}
// test

impl Color {
    #[inline]
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline]
    pub fn idx(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl PieceType {
    #[inline]
    pub fn idx(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceType,
}
