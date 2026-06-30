use crate::bitboard::{Square, square_from_algebraic};
use crate::board::Board;
use crate::types::{Color, PieceType};

pub const STARTPOS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const WHITE_KINGSIDE: u8 = 0b0001;
pub const WHITE_QUEENSIDE: u8 = 0b0010;
pub const BLACK_KINGSIDE: u8 = 0b0100;
pub const BLACK_QUEENSIDE: u8 = 0b1000;

// ****************************
// **** FEN IMPLEMENTATION ****
// ****************************

impl Board {
    pub fn from_fen(fen: &str) -> Result<Board, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();

        if parts.len() != 6 {
            return Err(format!(
                "FEN must have 6 fields, found {}: {}",
                parts.len(),
                fen
            ));
        }

        let placement = parts[0];
        let side = parts[1];
        let castling = parts[2];
        let ep = parts[3];
        let halfmove = parts[4];
        let fullmove = parts[5];

        let mut board = Board::empty();

        // -------------------------
        // Piece placement
        // -------------------------
        let mut rank: i32 = 7;
        let mut file: i32 = 0;

        for ch in placement.chars() {
            match ch {
                '/' => {
                    if file != 8 {
                        return Err(format!("Invalid FEN rank width before '/': {}", placement));
                    }

                    rank -= 1;
                    file = 0;

                    if rank < 0 {
                        return Err(format!("Too many ranks in FEN: {}", placement));
                    }
                }

                '1'..='8' => {
                    let empty_count = ch.to_digit(10).unwrap() as i32;
                    file += empty_count;

                    if file > 8 {
                        return Err(format!("Too many files in FEN rank: {}", placement));
                    }
                }

                piece_char => {
                    let Some((color, kind)) = piece_from_fen_char(piece_char) else {
                        return Err(format!("Invalid piece char in FEN: {}", piece_char));
                    };

                    if file >= 8 || rank < 0 {
                        return Err(format!("Piece out of board in FEN: {}", placement));
                    }

                    let sq = (rank * 8 + file) as Square;
                    board.add_piece(color, kind, sq);

                    file += 1;
                }
            }
        }

        if rank != 0 || file != 8 {
            return Err(format!("Invalid FEN board shape: {}", placement));
        }

        // -------------------------
        // Side to move
        // -------------------------
        board.side_to_move = parse_side_to_move(side)?;

        // -------------------------
        // Castling rights
        // -------------------------
        board.castling_rights = parse_castling_rights(castling)?;

        // -------------------------
        // En passant
        // -------------------------
        board.en_passant = if ep == "-" {
            None
        } else {
            Some(square_from_algebraic(ep)?)
        };

        // -------------------------
        // Clocks
        // -------------------------
        board.halfmove_clock = halfmove
            .parse::<u16>()
            .map_err(|_| format!("Invalid halfmove clock: {}", halfmove))?;

        board.fullmove_number = fullmove
            .parse::<u16>()
            .map_err(|_| format!("Invalid fullmove number: {}", fullmove))?;

        board.hash = board.compute_hash_from_scratch();

        // board.assert_valid();

        Ok(board)
    }
}

// *********************
// **** FEN HELPERS ****
// *********************

fn piece_from_fen_char(ch: char) -> Option<(Color, PieceType)> {
    let color = if ch.is_ascii_uppercase() {
        Color::White
    } else {
        Color::Black
    };

    let kind = match ch.to_ascii_lowercase() {
        'p' => PieceType::Pawn,
        'n' => PieceType::Knight,
        'b' => PieceType::Bishop,
        'r' => PieceType::Rook,
        'q' => PieceType::Queen,
        'k' => PieceType::King,
        _ => return None,
    };

    Some((color, kind))
}

fn parse_side_to_move(s: &str) -> Result<Color, String> {
    match s {
        "w" => Ok(Color::White),
        "b" => Ok(Color::Black),
        _ => Err(format!("Invalid side to move: {}", s)),
    }
}
fn parse_castling_rights(s: &str) -> Result<u8, String> {
    if s == "-" {
        return Ok(0);
    }

    let mut rights = 0u8;

    for ch in s.chars() {
        match ch {
            'K' => rights |= WHITE_KINGSIDE,
            'Q' => rights |= WHITE_QUEENSIDE,
            'k' => rights |= BLACK_KINGSIDE,
            'q' => rights |= BLACK_QUEENSIDE,
            _ => return Err(format!("Invalid castling rights char: {}", ch)),
        }
    }

    Ok(rights)
}
