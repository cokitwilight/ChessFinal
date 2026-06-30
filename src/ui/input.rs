use crate::bitboard::{Square, file_of, rank_of, square};
use crate::ui::BoardOrientation;
use eframe::egui;

/// Converts an egui pointer position into bitboard square index.
///
/// Bitboard layout:
///     0  = a1
///     1  = b1
///     8  = a2
///     63 = h8
///
/// egui screen coordinates:
///     x increases left -> right
///     y increases top -> bottom
pub fn screen_pos_to_square(
    pos: egui::Pos2,
    board_rect: egui::Rect,
    orientation: BoardOrientation,
) -> Option<Square> {
    let board_size = board_rect.width().min(board_rect.height());

    if board_size <= 0.0 {
        return None;
    }

    let square_size = board_size / 8.0;

    // Use the actual square board area, even if board_rect accidentally is not square.
    let active_rect = egui::Rect::from_min_size(board_rect.min, egui::vec2(board_size, board_size));

    if !active_rect.contains(pos) {
        return None;
    }

    let col = ((pos.x - active_rect.left()) / square_size).floor() as i32;
    let row = ((pos.y - active_rect.top()) / square_size).floor() as i32;

    if !(0..8).contains(&col) || !(0..8).contains(&row) {
        return None;
    }

    let (file, rank) = match orientation {
        BoardOrientation::WhiteBottom => {
            // top-left is a8
            let file = col;
            let rank = 7 - row;
            (file, rank)
        }
        BoardOrientation::BlackBottom => {
            // top-left is h1
            let file = 7 - col;
            let rank = row;
            (file, rank)
        }
    };

    Some(square(file as u8, rank as u8))
}

/// Converts bitboard square index into the egui rectangle where that square is drawn.
pub fn square_to_rect(
    sq: Square,
    board_rect: egui::Rect,
    orientation: BoardOrientation,
) -> egui::Rect {
    debug_assert!(sq < 64);

    let board_size = board_rect.width().min(board_rect.height());
    let square_size = board_size / 8.0;

    let file = file_of(sq) as i32;
    let rank = rank_of(sq) as i32;

    let (col, row) = match orientation {
        BoardOrientation::WhiteBottom => {
            // a1 is bottom-left, h8 is top-right
            let col = file;
            let row = 7 - rank;
            (col, row)
        }
        BoardOrientation::BlackBottom => {
            // a1 is top-right, h8 is bottom-left
            let col = 7 - file;
            let row = rank;
            (col, row)
        }
    };

    let min = egui::pos2(
        board_rect.left() + col as f32 * square_size,
        board_rect.top() + row as f32 * square_size,
    );

    egui::Rect::from_min_size(min, egui::vec2(square_size, square_size))
}
