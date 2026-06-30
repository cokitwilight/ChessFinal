use crate::bitboard::Square;
use crate::board::{Board, MoveList};
use crate::moves::pseudo::{all_pseudo_moves, all_pseudo_moves_at};
use crate::types::Color;

pub fn all_legal_moves(board: &mut Board, color: Color, moves: &mut MoveList) {
    debug_assert_eq!(
        board.side_to_move, color,
        "all_legal_moves called with color != board.side_to_move"
    );

    let mut pseudo_moves = MoveList::new();

    all_pseudo_moves(board, color, &mut pseudo_moves);

    for &mv in pseudo_moves.iter() {
        let undo = board.make_move(mv);

        let is_legal = !board.in_check(color);

        board.undo_move(undo);

        if is_legal {
            moves.push(mv);
        }
    }
}

pub fn all_legal_moves_at(board: &mut Board, color: Color, sq: Square, moves: &mut MoveList) {
    debug_assert_eq!(
        board.side_to_move, color,
        "all_legal_moves called with color != board.side_to_move"
    );

    let mut pseudo_moves = MoveList::new();

    all_pseudo_moves_at(board, color, sq, &mut pseudo_moves);

    for &mv in pseudo_moves.iter() {
        let undo = board.make_move(mv);

        let is_legal = !board.in_check(color);

        board.undo_move(undo);

        if is_legal {
            moves.push(mv);
        }
    }
}
