# Twilight Chess

Twilight Chess is a chess engine written entirely in Rust. 

The goal of this project is to build a solid chess engine with clean architecture. The project remains education and focuses on learning engine design, move ordering, heuristics, and low level optimizations rather than compete with chess giants like StockFish or Leela.

## Project Status

This project is currently being ported from a 2D array board representation to bitboards.

The move generator, board representation, GUI, Zobrist hashing, and game-end detection are currently working. Search and evaluation features are being ported next.

## Current Features

- Simple Bitboard Representation
- GUI with player vs player support
- Legal Move highlighting
- Promotion option slider
- Checkmate + Stalemate detection
- Perft Validated Move Generation
- Zobrist Hashing

## In Progress

- [ ] Static Evaluation
- [ ] Negamax Search with Alpha Beta pruning
- [ ] Quiescence Search
- [ ] Transposition Tables for regualar and quiescence search
- [ ] Killer Moves Heuristic
- [ ] History Heuristics
- [ ] SEE(Static Exchange Evaluation)
- [ ] Iterative Deepening
- [ ] Aspiration Windows
- [ ] Delta Pruning
- [ ] Simple Opening Book
- [ ] Gui with Player vs Bot

## Planned Features

- [ ] Null Move Pruning
- [ ] Magic Bitboards
- [ ] UCI Support
- [ ] SMP Search

## Requirements

- Rust
- Cargo

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run --release
```

## Testing

Run all tests:

```bash
cargo test --release
```
