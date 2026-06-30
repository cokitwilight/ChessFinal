use crate::types::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Ongoing,
    Checkmate { winner: Color },
    Stalemate,
    DrawByRepetition,
    DrawByFiftyMoveRule,
    DrawByInsufficientMaterial,
}
