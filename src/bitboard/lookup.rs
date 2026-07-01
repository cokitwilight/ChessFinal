use std::sync::OnceLock;

use crate::bitboard::{
    Bitboard, FILE_A, FILE_H, NOT_FILE_A, NOT_FILE_AB, NOT_FILE_GH, NOT_FILE_H, Square, bit,
};

pub struct AttackTables {
    pub knight: [Bitboard; 64],
    pub king: [Bitboard; 64],
    pub white_pawn: [Bitboard; 64],
    pub black_pawn: [Bitboard; 64],
}
impl Default for AttackTables {
    fn default() -> Self {
        Self::new()
    }
}

impl AttackTables {
    pub fn new() -> Self {
        let mut knight = [0u64; 64];
        let mut king = [0u64; 64];
        let mut white_pawn = [0u64; 64];
        let mut black_pawn = [0u64; 64];

        for sq in 0..64 {
            let sq = sq as Square;

            knight[sq as usize] = generate_knight_attacks(sq);
            king[sq as usize] = generate_king_attacks(sq);
            white_pawn[sq as usize] = generate_white_pawn_attacks(sq);
            black_pawn[sq as usize] = generate_black_pawn_attacks(sq);
        }

        Self {
            knight,
            king,
            white_pawn,
            black_pawn,
        }
    }
}

static ATTACK_TABLES: OnceLock<AttackTables> = OnceLock::new();

pub fn attack_tables() -> &'static AttackTables {
    ATTACK_TABLES.get_or_init(AttackTables::new)
}

fn generate_knight_attacks(sq: Square) -> Bitboard {
    let b = bit(sq);

    let mut attacks = 0u64;

    attacks |= (b & NOT_FILE_H) << 17;
    attacks |= (b & NOT_FILE_A) << 15;
    attacks |= (b & NOT_FILE_GH) << 10;
    attacks |= (b & NOT_FILE_AB) << 6;

    attacks |= (b & NOT_FILE_A) >> 17;
    attacks |= (b & NOT_FILE_H) >> 15;
    attacks |= (b & NOT_FILE_AB) >> 10;
    attacks |= (b & NOT_FILE_GH) >> 6;

    attacks
}

fn generate_king_attacks(sq: Square) -> Bitboard {
    let b = bit(sq);

    let mut attacks = 0u64;

    // Vertical
    attacks |= b << 8;
    attacks |= b >> 8;

    // Horizontal
    attacks |= (b & NOT_FILE_H) << 1;
    attacks |= (b & NOT_FILE_A) >> 1;

    // Diagonals upward
    attacks |= (b & NOT_FILE_H) << 9;
    attacks |= (b & NOT_FILE_A) << 7;

    // Diagonals downward
    attacks |= (b & NOT_FILE_H) >> 7;
    attacks |= (b & NOT_FILE_A) >> 9;

    attacks
}

fn generate_white_pawn_attacks(sq: Square) -> Bitboard {
    let b = bit(sq);

    let mut attacks = 0u64;

    attacks |= (b & NOT_FILE_A) << 7;
    attacks |= (b & NOT_FILE_H) << 9;

    attacks
}

fn generate_black_pawn_attacks(sq: Square) -> Bitboard {
    let b = bit(sq);

    let mut attacks = 0u64;

    attacks |= (b & NOT_FILE_A) >> 9;
    attacks |= (b & NOT_FILE_H) >> 7;

    attacks
}
