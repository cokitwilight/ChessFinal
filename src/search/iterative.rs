use crate::board::Board;
use crate::search::engine::{Engine, SearchContext, SearchResult, SearchStats};

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

        for depth in 1..=ctx.limits.max_depth {
            if ctx.should_stop() {
                break;
            }

            let result = self.search_root(board, ctx, depth);

            if ctx.should_stop() && result.best_move.is_none() {
                break;
            }

            best_result = result;
            best_result.depth_reached = depth;
        }

        best_result.stats = ctx.stats;
        best_result
    }

    fn search_root(
        &mut self,
        board: &mut Board,
        ctx: &mut SearchContext,
        depth: usize,
    ) -> SearchResult {
        let mut best_eval = i32::MIN;
        let mut best_move = None;

        let legal_moves = board.all_legal_moves();

        for mv in legal_moves.iter() {
            if ctx.should_stop() {
                break;
            }

            let undo = board.make_move(*mv);
            let eval = -self.negamax(
                board,
                ctx,
                depth - 1,
                i32::MIN + 1,
                i32::MAX - 1,
                ctx.ply + 1,
            );
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
