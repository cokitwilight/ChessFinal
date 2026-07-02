use crate::bitboard::Square;
use crate::types::Color;

#[derive(Clone, Copy, Debug)]
pub struct HistoryTable {
    table: [[[i32; 64]; 64]; 2],
}

impl Default for HistoryTable {
    fn default() -> Self {
        Self {
            table: [[[0; 64]; 64]; 2],
        }
    }
}

impl HistoryTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, color: Color, from: Square, to: Square) -> i32 {
        self.table[color as usize][from as usize][to as usize]
    }

    pub fn add_bonus(&mut self, color: Color, from: Square, to: Square, depth: usize) {
        let bonus = (depth * depth) as i32;
        self.table[color as usize][from as usize][to as usize] += bonus;
    }

    pub fn age(&mut self) {
        for color in 0..2 {
            for from in 0..64 {
                for to in 0..64 {
                    self.table[color][from][to] /= 2;
                }
            }
        }
    }
}
