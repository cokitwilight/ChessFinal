use crate::{
    board::Board,
    eval::{phase::MAX_PHASE, pst::pst_bonus},
    types::{Color, PieceType},
};

pub fn evaluation(board: &Board) -> i32 {
    let mut total_eval = 0;

    let w_pawns = board.pieces(Color::White, PieceType::Pawn).count_ones() as i32;
    let b_pawns = board.pieces(Color::Black, PieceType::Pawn).count_ones() as i32;

    let w_knights = board.pieces(Color::White, PieceType::Knight).count_ones() as i32;
    let b_knights = board.pieces(Color::Black, PieceType::Knight).count_ones() as i32;

    let w_bishops = board.pieces(Color::White, PieceType::Bishop).count_ones() as i32;
    let b_bishops = board.pieces(Color::Black, PieceType::Bishop).count_ones() as i32;

    let w_rooks = board.pieces(Color::White, PieceType::Rook).count_ones() as i32;
    let b_rooks = board.pieces(Color::Black, PieceType::Rook).count_ones() as i32;

    let w_queens = board.pieces(Color::White, PieceType::Queen).count_ones() as i32;
    let b_queens = board.pieces(Color::Black, PieceType::Queen).count_ones() as i32;

    let material = 100 * (w_pawns - b_pawns)
        + 310 * (w_knights - b_knights)
        + 330 * (w_bishops - b_bishops)
        + 500 * (w_rooks - b_rooks)
        + 900 * (w_queens - b_queens);

    let mut phase = (w_knights + b_knights + w_bishops + b_bishops)
        + 2 * (w_rooks + b_rooks)
        + 4 * (w_queens + b_queens);
    phase = phase.min(MAX_PHASE);

    let pst_bonus = pst_bonus(board, phase);

    total_eval += pst_bonus + material;

    total_eval
}

pub fn evaluation_for_turn(board: &Board) -> i32 {
    let eval = evaluation(board);
    match board.side_to_move() {
        Color::White => eval,
        Color::Black => -eval,
    }
}
