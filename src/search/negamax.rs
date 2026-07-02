use crate::{
    board::{Board, Move, mv},
    eval::eval::evaluation_for_turn,
    search::engine::{Engine, SearchContext},
};

pub const CHECKMATE_SCORE: i32 = 100000;

impl Engine {
    // Implementation for negamax function
    pub fn negamax(
        &mut self,
        board: &mut Board,
        context: &mut SearchContext,
        depth: usize,
        mut alpha: i32,
        mut beta: i32,
        ply: usize,
    ) -> i32 {
        // Placeholder for the actual negamax implementation
        // This function should implement the negamax search algorithm
        // and return the evaluation score for the given board state.
        context.stats.nodes += 1;

        // ADD DRAWING LOGIC HERE

        if depth == 0 {
            // Placeholder for quiescence search or evaluation function
            return evaluation_for_turn(board); // Replace with actual evaluation
        }

        let mut max_eval = i32::MIN;
        let mut best_move: Option<Move> = None;

        let mut moves = board.all_legal_moves();

        let side_to_move = board.side_to_move();

        // ADD MOVE ORDERING HERE

        if moves.is_empty() {
            if board.in_check(side_to_move) {
                return -CHECKMATE_SCORE + ply as i32;
            } else {
                return 0; // Stalemate
            }
        }

        for (move_index, mv) in moves.iter().enumerate() {
            // Use later for move ordering and heuristics
            // let parent_hash = board.hash;
            // let in_check = board.in_check();
            // let quiet = Engine::is_quiet_move(*mv);
            // let gives_check = board.in_check();

            // Move index used for lmr

            let undo = board.make_move(*mv);

            let eval = -self.negamax(board, context, depth - 1, -beta, -alpha, ply + 1);

            board.undo_move(undo);

            if eval > max_eval {
                max_eval = eval;
                best_move = Some(*mv);
            }

            alpha = alpha.max(eval);

            if alpha >= beta {
                // ADD KILLER MOVE AND HISTORY HEURISTIC HERE
                break;
            }
        }

        // Store the best move in the search context for later use

        max_eval
    }
}

// pub fn negamax(
//         &mut self,
//         board: &mut Board,
//         depth: usize,
//         mut alpha: i32,
//         mut beta: i32,
//         search_history: &mut Vec<u64>,
//         ply: usize,
//     ) -> i32 {
//         self.nodes += 1;

//         // if self.is_repetition_in_search(search_history, board.hash) {
//         //     self.repetition_returns += 1;
//         //     return 0;
//         // }

//         // if board.is_fifty_move_draw() {
//         //     self.fifty_returns += 1;
//         //     return 0;
//         // }

//         if depth == 0 {
//             return 0;
//             // return self.quiescence(board, alpha, beta, MAX_Q_DEPTH, search_history, ply);
//         }

//         let mut moves = board.all_legal_moves();

//         if moves.is_empty() {
//             if board.in_check() {
//                 return -CHECKMATE_SCORE + ply as i32;
//             } else {
//                 return 0;
//             }
//         }

//         let side_to_move = board.get_turn();

//         self.order_moves(board, &mut moves, side_to_move, ply, None, tt_best_move);

//         let mut max_eval = NEG_INF;
//         let mut best_move: Option<Move> = None;

//         // println!("Line 787: negamax");

//         for (move_index, mv) in moves.iter().enumerate() {
//             let parent_hash = board.hash;

//             let in_check = board.in_check();

//             let undo = board.make_move(*mv);
//             search_history.push(parent_hash);

//             let quiet = Engine::is_quiet_move(*mv);

//             let gives_check = board.in_check();

//             // let reduction = 0; // compare with regular

//             let reduction = if quiet && !in_check && !gives_check {
//                 Self::lmr_reduction(depth, move_index)
//             } else {
//                 0
//             };

//             let mut eval;

//             if reduction > 0 {
//                 // Reduced null-window search.
//                 eval = -self.negamax(
//                     board,
//                     depth - 1 - reduction,
//                     -alpha - 1,
//                     -alpha,
//                     search_history,
//                     ply + 1,
//                 );

//                 // If the reduced search says this move may improve alpha,
//                 // re-search it at full depth with a full window.
//                 if eval > alpha {
//                     eval = -self.negamax(board, depth - 1, -beta, -alpha, search_history, ply + 1);
//                 }
//             } else {
//                 // Normal full-depth full-window search.
//                 eval = -self.negamax(board, depth - 1, -beta, -alpha, search_history, ply + 1);
//             }

//             search_history.pop();
//             board.undo_move(undo);

//             if eval > max_eval {
//                 max_eval = eval;
//                 best_move = Some(*mv);
//             }

//             alpha = alpha.max(eval);

//             if alpha >= beta {
//                 if Engine::is_quiet_move(*mv) {
//                     self.store_killer_move(ply, *mv);
//                     self.add_history_bonus(side_to_move, *mv, depth);
//                 }
//                 break;
//             }
//         }

//         let flag = if max_eval <= original_alpha {
//             TTFlag::UpperBound
//         } else if max_eval >= original_beta {
//             TTFlag::LowerBound
//         } else {
//             TTFlag::Exact
//         };

//         self.tt.insert(
//             board.hash,
//             TTEntry {
//                 hash: board.hash,
//                 depth,
//                 eval: max_eval,
//                 best_move,
//                 flag,
//             },
//         );

//         max_eval
//     }
