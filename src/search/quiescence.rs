use crate::{
    board::{Board, Move},
    eval::eval::evaluation_for_turn,
    search::{
        Engine,
        engine::SearchContext,
        negamax::{CHECKMATE_SCORE, NEG_INF},
        tt::{TTEntry, TTFlag},
    },
};

impl Engine {
    pub fn quiescence(
        &mut self,
        board: &mut Board,
        context: &mut SearchContext,
        depth: usize,
        mut alpha: i32,
        mut beta: i32,
        ply: usize,
    ) -> i32 {
        context.stats.qnodes += 1;

        if Engine::repetition_in_search(context, board.hash(), board.halfmove_clock() as usize) {
            return 0;
        }
        if board.halfmove_clock() >= 100 {
            return 0;
        }

        let original_alpha = alpha;
        let original_beta = beta;
        let hash = board.hash();

        let mut tt_best_move: Option<Move> = None;

        context.stats.qtt.probes += 1;

        if let Some(entry) = self.qtt.get(hash) {
            debug_assert_eq!(
                entry.hash, hash,
                "QTT hash mismatch: key matched but entry.hash differed"
            );

            context.stats.qtt.hits += 1;
            tt_best_move = entry.best_move;

            if entry.depth >= depth {
                context.stats.qtt.usable_hits += 1;

                match entry.flag {
                    TTFlag::Exact => {
                        context.stats.qtt.exact_returns += 1;
                        return entry.eval;
                    }

                    TTFlag::LowerBound => {
                        alpha = alpha.max(entry.eval);
                    }

                    TTFlag::UpperBound => {
                        beta = beta.min(entry.eval);
                    }
                }

                if alpha >= beta {
                    context.stats.qtt.bound_cutoffs += 1;
                    return entry.eval;
                }
            }
        }

        let in_check = board.in_check(board.side_to_move());

        let mut best_score = NEG_INF;
        let mut best_move: Option<Move> = None;

        let raw_moves = if in_check {
            let evasions = board.all_legal_moves();

            if evasions.is_empty() {
                let score = -CHECKMATE_SCORE + ply as i32;
                self.qtt.insert(
                    hash,
                    TTEntry {
                        hash,
                        depth,
                        eval: score,
                        best_move: None,
                        flag: TTFlag::Exact,
                    },
                );
                return score;
            }

            if depth == 0 {
                // TODO: Later keep searching regardless since in check.
                let score = evaluation_for_turn(board);

                let flag = if score <= original_alpha {
                    TTFlag::UpperBound
                } else if score >= original_beta {
                    TTFlag::LowerBound
                } else {
                    TTFlag::Exact
                };

                self.qtt.insert(
                    hash,
                    TTEntry {
                        hash,
                        depth,
                        eval: score,
                        best_move: None,
                        flag,
                    },
                );
                return score;
            }

            evasions
        } else {
            let stand_pat = evaluation_for_turn(board);
            best_score = stand_pat;

            if stand_pat >= beta {
                self.qtt.insert(
                    hash,
                    TTEntry {
                        hash,
                        depth,
                        eval: stand_pat,
                        best_move: None,
                        flag: TTFlag::LowerBound,
                    },
                );

                return stand_pat;
            }

            if alpha < stand_pat {
                alpha = stand_pat;
            }

            if depth == 0 {
                self.qtt.insert(
                    hash,
                    TTEntry {
                        hash,
                        depth,
                        eval: stand_pat,
                        best_move: None,
                        flag: TTFlag::Exact,
                    },
                );
                return stand_pat;
            }
            board.all_legal_capture_moves()
        };

        // do move ordering here

        for mv in raw_moves.iter() {
            // add see pruning and delta pruning here

            let parent_hash = board.hash();

            let undo = board.make_move(*mv);
            context.repetition_history.push(parent_hash);

            let score = -self.quiescence(board, context, depth - 1, -beta, -alpha, ply + 1);

            context.repetition_history.pop();

            board.undo_move(undo);

            if score > best_score {
                best_score = score;
                best_move = Some(*mv);
            }

            if score > alpha {
                alpha = score;
            }

            if score >= beta {
                self.qtt.insert(
                    hash,
                    TTEntry {
                        hash,
                        depth,
                        eval: score,
                        best_move,
                        flag: TTFlag::LowerBound,
                    },
                );
                return score;
            }
        }

        let flag = if best_score <= original_alpha {
            TTFlag::UpperBound
        } else if best_score >= original_beta {
            TTFlag::LowerBound
        } else {
            TTFlag::Exact
        };
        self.qtt.insert(
            hash,
            TTEntry {
                hash,
                depth,
                eval: best_score,
                best_move,
                flag,
            },
        );

        best_score
    }
}

// fn quiescence(
//     &mut self,
//     board: &mut Board,
//     mut alpha: i32,
//     mut beta: i32,
//     q_depth: usize,
//     search_history: &mut Vec<u64>,
//     q_ply: usize,
// ) -> i32 {
//     self.qnodes += 1;

//     if self.is_repetition_in_search(search_history, board.hash) || board.is_fifty_move_draw() {
//         return 0;
//     }

//     let original_alpha = alpha;
//     let original_beta = beta;
//     let hash = board.hash;

//     let mut tt_best_move: Option<Move> = None;

//     // Probe quiescence TT.
//     self.qtt_probes += 1;

//     if let Some(entry) = self.qtt.get(&hash) {
//         debug_assert_eq!(
//             entry.hash, hash,
//             "QTT hash mismatch: key matched but entry.hash differed"
//         );

//         self.qtt_hits += 1;
//         tt_best_move = entry.best_move;

//         if entry.q_depth >= q_depth {
//             self.qtt_usable_hits += 1;

//             match entry.flag {
//                 TTFlag::Exact => {
//                     self.qtt_exact_returns += 1;
//                     return entry.eval;
//                 }

//                 TTFlag::LowerBound => {
//                     alpha = alpha.max(entry.eval);
//                 }

//                 TTFlag::UpperBound => {
//                     beta = beta.min(entry.eval);
//                 }
//             }

//             if alpha >= beta {
//                 self.qtt_bound_cutoffs += 1;
//                 return entry.eval;
//             }
//         }
//     }

//     let in_check = board.in_check();

//     let mut best_score = NEG_INF;
//     let mut best_move: Option<Move> = None;

//     // let mut stand_pat = NEG_INF;

//     let raw_moves = if in_check {
//         let evasions = board.all_legal_moves();

//         if evasions.is_empty() {
//             let score = -CHECKMATE_SCORE + q_ply as i32;

//             self.store_qtt(hash, q_depth, score, TTFlag::Exact, None);

//             return score;
//         }

//         if q_depth == 0 {
//             let score = self.evaluation_for_turn(board);

//             let flag = if score <= original_alpha {
//                 TTFlag::UpperBound
//             } else if score >= original_beta {
//                 TTFlag::LowerBound
//             } else {
//                 TTFlag::Exact
//             };

//             self.store_qtt(hash, q_depth, score, flag, None);

//             return score;
//         }

//         evasions
//     } else {
//         let stand_pat = self.evaluation_for_turn(board);
//         best_score = stand_pat;

//         if stand_pat >= beta {
//             self.store_qtt(hash, q_depth, stand_pat, TTFlag::LowerBound, None);
//             return stand_pat;
//         }

//         if alpha < stand_pat {
//             alpha = stand_pat;
//         }

//         if q_depth == 0 {
//             let score = self.evaluation_for_turn(board);

//             let flag = if score <= original_alpha {
//                 TTFlag::UpperBound
//             } else if score >= original_beta {
//                 TTFlag::LowerBound
//             } else {
//                 TTFlag::Exact
//             };

//             self.store_qtt(hash, q_depth, stand_pat, flag, None);

//             return stand_pat;
//         }

//         board.all_legal_capture_moves() // returns promotions, captures, and en_passant only
//     };

//     let mut qmoves = self.q_moves_with_see(board, raw_moves); // stores See values so they aren't computed twice

//     if let Some(tt_mv) = tt_best_move {
//         if let Some(index) = qmoves.iter().position(|mv_s| mv_s.mv == tt_mv) {
//             qmoves.swap(0, index);
//         }
//     }

//     for mv_s in qmoves {
//         // since !in_check, stand_pat will always = evaluation_for_turn in order to reach this path
//         let mv = mv_s.mv;
//         let see_score = mv_s.see;

//         if !in_check && self.can_delta_prune(board, mv, best_score, alpha, see_score) {
//             continue;
//         }

//         if !in_check && (mv.kind == MoveType::Capture || mv.kind == MoveType::EnPassant) {
//             if see_score < -100 {
//                 // the SEE capture loses a pawn
//                 continue;
//             }
//         }

//         let parent_hash = board.hash;

//         let undo = board.make_move(mv);

//         search_history.push(parent_hash);

//         let score =
//             -self.quiescence(board, -beta, -alpha, q_depth - 1, search_history, q_ply + 1);

//         search_history.pop();

//         board.undo_move(undo);

//         if score > best_score {
//             best_score = score;
//             best_move = Some(mv);
//         }

//         if score > alpha {
//             alpha = score;
//         }

//         if alpha >= beta {
//             self.store_qtt(hash, q_depth, best_score, TTFlag::LowerBound, best_move);

//             return best_score;
//         }
//     }

//     let flag = if best_score <= original_alpha {
//         TTFlag::UpperBound
//     } else if best_score >= original_beta {
//         TTFlag::LowerBound
//     } else {
//         TTFlag::Exact
//     };

//     self.store_qtt(hash, q_depth, best_score, flag, best_move);

//     best_score
// }
