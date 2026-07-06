use crate::board::{Board, Move, MoveList, MoveType};
use crate::search::Engine;
use crate::search::engine::SearchContext;
use crate::types::Color;

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
        if mv.promotion.is_some() {
            // later add per piece type
            return 900_000;
        }
        match mv.kind {
            MoveType::Capture => {
                // add SEE here

                return 600_000;
            }
            MoveType::EnPassant => {
                return 850_000;
            }
            MoveType::Castle | MoveType::Normal => {
                if context.killer_moves.contains(ply, mv) {
                    return 800_000;
                }

                // add history heuristics here
                return self.history.get(side_to_move, mv.from, mv.to).min(500_000);
            }
        }
    }
}
