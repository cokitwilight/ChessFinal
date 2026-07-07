use crate::board::{Board, Move, MoveList, MoveType};
use crate::search::Engine;
use crate::search::engine::SearchContext;
use crate::types::{Color, PieceType};

use std::cmp::Reverse;

impl Engine {
    pub fn order_moves(
        &self,
        board: &Board,
        moves: &mut MoveList,
        side_to_move: Color,
        ply: usize,
        context: &mut SearchContext,
        previous_best_move: Option<Move>,
        tt_best_move: Option<Move>,
    ) {
        moves.sort_by_score(|mv| {
            self.move_order_score(
                board,
                mv,
                side_to_move,
                ply,
                context,
                previous_best_move,
                tt_best_move,
            )
        });
    }
    pub fn move_order_score(
        &self,
        board: &Board,
        mv: Move,
        side_to_move: Color,
        ply: usize,
        context: &SearchContext,
        previous_best_move: Option<Move>,
        tt_best_move: Option<Move>,
    ) -> i32 {
        if Some(mv) == previous_best_move {
            return 2_000_000;
        }
        if Some(mv) == tt_best_move {
            return 1_500_000;
        }
        match mv.kind {
            MoveType::Capture => {
                // add SEE here
                let mvv_lva = mvv_lva_score(board, mv, side_to_move);

                return 900_000 + mvv_lva;
            }
            MoveType::EnPassant => {
                let mvv_lva = mvv_lva_score(board, mv, side_to_move);

                return 850_000 + mvv_lva;
            }
            MoveType::Castle | MoveType::Normal => {
                if mv.promotion.is_some() {
                    return 800_000 + promotion_score(mv.promotion.unwrap());
                }
                if context.killer_moves.contains(ply, mv) {
                    return 700_000;
                }

                // add history heuristics here
                return self.history.get(side_to_move, mv.from, mv.to).min(500_000);
            }
        }
    }
}

fn mvv_lva_score(board: &Board, mv: Move, side_to_move: Color) -> i32 {
    let attacker = board
        .piece_at(mv.from)
        .expect("move_order_score called with no attacker on mv.from")
        .kind;

    let victim = match mv.kind {
        MoveType::Capture => {
            board
                .piece_at(mv.to)
                .expect("capture move has no victim on mv.to")
                .kind
        }
        MoveType::EnPassant => PieceType::Pawn,

        _ => return 0,
    };

    let victim_value = mvv_lva_piece_value(victim);
    let attacker_value = mvv_lva_piece_value(attacker);

    // Main MVV-LVA idea:
    //
    // Higher victim value = better.
    // Lower attacker value = better.
    //
    // Multiply victim value so victim importance dominates attacker penalty.
    let mut score = victim_value * 10 - attacker_value;

    if let Some(promo) = mv.promotion {
        score += promotion_score(promo);
    }

    score
}

fn promotion_score(piece: PieceType) -> i32 {
    match piece {
        PieceType::Queen => 8_000,
        PieceType::Rook => 4_000,
        PieceType::Bishop => 3_000,
        PieceType::Knight => 3_000,
        PieceType::Pawn => 0,
        PieceType::King => 0,
    }
}

fn mvv_lva_piece_value(piece: PieceType) -> i32 {
    match piece {
        PieceType::Pawn => 100,
        PieceType::Knight => 300,
        PieceType::Bishop => 300,
        PieceType::Rook => 500,
        PieceType::Queen => 900,
        PieceType::King => 10_000,
    }
}
