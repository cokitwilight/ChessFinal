use crate::board::Move;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TTFlag {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone, Copy, Debug)]
pub struct TTEntry {
    pub hash: u64,
    pub depth: usize,
    pub eval: i32,
    pub best_move: Option<Move>,
    pub flag: TTFlag,
}

#[derive(Clone, Debug)]
pub struct TranspositionTable<Entry> {
    table: HashMap<u64, Entry>,
}

impl<Entry> TranspositionTable<Entry> {
    pub fn new(_mb: usize) -> Self {
        // implement size in mb
        Self {
            table: HashMap::with_capacity(_mb),
        }
    }

    pub fn get(&self, key: u64) -> Option<&Entry> {
        self.table.get(&key)
    }

    pub fn insert(&mut self, key: u64, entry: Entry) {
        self.table.insert(key, entry);
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }
}
