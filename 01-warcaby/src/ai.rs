use crate::{BOARD_SIZE, Checkers, Move, game::Winner};

pub struct CheckersRules;

impl minimax::Game for CheckersRules {
    type S = Checkers;
    type M = Move;

    fn generate_moves(state: &Self::S, moves: &mut Vec<Self::M>) {
        let mut possible_moves = state.valid_moves(state.piece_for_turn());
        moves.append(&mut possible_moves);
    }

    fn apply(state: &mut Self::S, m: Self::M) -> Option<Self::S> {
        Some(state.apply_move(m))
    }

    fn get_winner(state: &Self::S) -> Option<minimax::Winner> {
        let just_played = state.turn().opposite();
        match state.get_winner() {
            Winner::Won(winner) => {
                // If our logic is sound, the winner always will be the player that just moved
                debug_assert_eq!(winner, just_played);
                Some(minimax::Winner::PlayerJustMoved)
            }
            Winner::Draw => Some(minimax::Winner::Draw),
            Winner::InProgress => None,
        }
    }
}

pub struct CheckerEval;

impl minimax::Evaluator for CheckerEval {
    type G = CheckersRules;

    /// The score evaluation is very simple.
    /// The farther a piece is from its home edge, the higher the value.
    /// Then, pieces of the current player are added, while pieces of the opponent are subtracted.
    fn evaluate(&self, s: &<Self::G as minimax::Game>::S) -> minimax::Evaluation {
        let to_move = s.piece_for_turn();

        s.iter_pieces()
            .map(|(y, _, piece)| {
                let value = if piece == s.player() {
                    BOARD_SIZE - y
                } else {
                    y + 1
                } as i16;
                let mult = if to_move == piece { 2 } else { -3 };
                value * mult
            })
            .sum()
    }
}
