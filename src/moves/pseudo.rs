use crate::bitboard::Square;
use crate::board::{Board, MoveList};
use crate::moves::king::pseudo_king_moves_at;
use crate::moves::knight::pseudo_knight_moves_at;
use crate::moves::pawn::pseudo_pawn_moves_at;
use crate::moves::sliders::{pseudo_bishop_moves_at, pseudo_queen_moves_at, pseudo_rook_moves_at};
use crate::moves::{
    king::pseudo_king_moves,
    knight::pseudo_knight_moves,
    pawn::pseudo_pawn_moves,
    sliders::{pseudo_bishop_moves, pseudo_queen_moves, pseudo_rook_moves},
};
use crate::types::Color;

pub fn all_pseudo_moves(board: &Board, color: Color, moves: &mut MoveList) {
    pseudo_pawn_moves(board, color, moves);
    pseudo_knight_moves(board, color, moves);
    pseudo_bishop_moves(board, color, moves);
    pseudo_rook_moves(board, color, moves);
    pseudo_queen_moves(board, color, moves);
    pseudo_king_moves(board, color, moves);
}

pub fn all_pseudo_moves_at(board: &Board, color: Color, sq: Square, moves: &mut MoveList) {
    pseudo_pawn_moves_at(board, color, sq, moves);
    pseudo_knight_moves_at(board, color, sq, moves);
    pseudo_bishop_moves_at(board, color, sq, moves);
    pseudo_rook_moves_at(board, color, sq, moves);
    pseudo_queen_moves_at(board, color, sq, moves);
    pseudo_king_moves_at(board, color, sq, moves);
}
