use std::time::Instant;

use crate::board::{Board, Move};
use crate::search::engine::{Engine, SearchContext, SearchResult, SearchStats};
use crate::search::negamax::{NEG_INF, POS_INF};

// for debug printing
use thousands::Separable;

impl Engine {
    pub fn iterative_deepening(
        &mut self,
        board: &mut Board,
        ctx: &mut SearchContext,
    ) -> SearchResult {
        let mut best_result = SearchResult {
            best_move: None,
            eval: 0,
            depth_reached: 0,
            stats: SearchStats::default(),
            pv: Vec::new(),
        };

        let mut total_time: f64 = 0.0;
        let mut total_nps: f64 = 0.0;

        for depth in 1..=ctx.limits.max_depth {
            if ctx.should_stop() {
                break;
            }

            let start = Instant::now();
            let result = self.search_root(board, ctx, best_result.best_move, depth);
            let elapsed = start.elapsed();
            let nodes_p_sec = (ctx.stats.nodes + ctx.stats.qnodes) as f64 / elapsed.as_secs_f64();
            total_nps += nodes_p_sec;
            total_time += elapsed.as_secs_f64();
            ctx.stats.print_all(depth);
            println!(
                "Time Elapsed: {:.3}. NPS: {}",
                elapsed.as_secs_f64(),
                format!("{:.2}", nodes_p_sec).separate_with_commas()
            );

            if ctx.should_stop() && result.best_move.is_none() {
                break;
            }

            best_result = result;
            best_result.depth_reached = depth;
        }

        println!(
            "\nTotal Time: {:.3}. Total NPS: {}\n",
            total_time,
            format!("{:.2}", total_nps).separate_with_commas()
        );

        best_result.stats = ctx.stats;
        best_result
    }

    fn search_root(
        &mut self,
        board: &mut Board,
        ctx: &mut SearchContext,
        previous_best_move: Option<Move>,
        depth: usize,
    ) -> SearchResult {
        let mut best_eval = i32::MIN;
        let mut best_move = None;

        let mut legal_moves = board.all_legal_moves();

        let tt_best_move = self.tt.get(board.hash).and_then(|entry| entry.best_move);

        self.order_moves(
            board,
            &mut legal_moves,
            board.side_to_move(),
            0,
            ctx,
            previous_best_move,
            tt_best_move,
        );

        for mv in legal_moves.iter() {
            if ctx.should_stop() {
                break;
            }

            let undo = board.make_move(*mv);
            let eval = -self.negamax(board, ctx, depth - 1, NEG_INF + 1, POS_INF - 1, 1);
            board.undo_move(undo);

            if eval > best_eval {
                best_eval = eval;
                best_move = Some(*mv);
            }
        }

        SearchResult {
            best_move,
            eval: best_eval,
            depth_reached: depth,
            stats: ctx.stats.clone(),
            pv: Vec::new(), // TODO: Implement principal variation
        }
    }
}
