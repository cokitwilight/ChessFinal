use crate::bitboard::Square;
use crate::types::PieceType;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum MoveType {
    Normal = 0,
    Capture = 1,
    EnPassant = 2,
    Castle = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub kind: MoveType,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub const NULL: Move = Move {
        from: 0,
        to: 0,
        kind: MoveType::Normal,
        promotion: None,
    };

    pub fn is_capture(self) -> bool {
        matches!(self.kind, MoveType::Capture | MoveType::EnPassant)
    }

    pub fn is_promotion(self) -> bool {
        self.promotion.is_some()
    }

    pub fn is_castle(self) -> bool {
        matches!(self.kind, MoveType::Castle)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct MoveList {
    moves: [Move; 256],
    len: usize,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [Move::NULL; 256],
            len: 0,
        }
    }

    pub fn push(&mut self, mv: Move) {
        assert!(
            self.len < self.moves.len(),
            "MoveList overflow: more than 256 moves generated"
        );

        self.moves[self.len] = mv;
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn as_slice(&self) -> &[Move] {
        &self.moves[..self.len]
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Move> {
        self.as_slice().iter()
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}
