pub const BOARD_SIZE: usize = 8;

#[derive(Clone)]
pub struct Checkers {
    board: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    player: Piece,
    turn: Turn,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub from: (usize, usize),
    pub d: (isize, isize),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Piece {
    Black,
    White,
}

impl Piece {
    pub fn opposite(self) -> Self {
        match self {
            Piece::Black => Piece::White,
            Piece::White => Piece::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    Player,
    Ai,
}

impl Turn {
    pub fn opposite(self) -> Self {
        match self {
            Turn::Player => Turn::Ai,
            Turn::Ai => Turn::Player,
        }
    }
}

impl Checkers {
    pub fn new(player: Piece) -> Self {
        let mut board = [[None; BOARD_SIZE]; BOARD_SIZE];

        for y in 0..BOARD_SIZE {
            if let 3 | 4 = y {
                continue;
            }
            let start = !y & 1;
            for x in (start..BOARD_SIZE).step_by(2) {
                board[y][x] = Some(if y < BOARD_SIZE / 2 {
                    player.opposite()
                } else {
                    player
                });
            }
        }

        let turn = if player == Piece::White {
            Turn::Player
        } else {
            Turn::Ai
        };

        Checkers {
            board,
            player,
            turn,
        }
    }

    pub fn turn(&self) -> Turn {
        self.turn
    }

    pub fn cell(&self, (y, x): (usize, usize)) -> Option<Piece> {
        self.board[y][x]
    }

    pub fn player(&self) -> Piece {
        self.player
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = (usize, &[Option<Piece>; BOARD_SIZE])> {
        self.board.iter().enumerate()
    }

    pub fn iter_board(&self) -> impl Iterator<Item = (usize, usize, Option<Piece>)> {
        self.board
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &cell)| (y, x, cell)))
    }

    pub fn iter_pieces(&self) -> impl Iterator<Item = (usize, usize, Piece)> {
        self.iter_board().filter_map(|(y, x, cell)| {
            let piece = cell?;
            Some((y, x, piece))
        })
    }

    fn maybe_move(
        &self,
        who: Piece,
        pos: (usize, usize),
        dy: isize,
        dx: isize,
        final_move: bool,
    ) -> Option<Move> {
        debug_assert_eq!(Some(who), self.board[pos.0][pos.1]);

        let new_y = pos.0.checked_add_signed(dy).filter(|&n| n < BOARD_SIZE)?;
        let new_x = pos.1.checked_add_signed(dx).filter(|&n| n < BOARD_SIZE)?;

        match self.board[new_y][new_x] {
            None => Some(Move {
                from: pos,
                d: (dy, dx),
            }),
            Some(p) if p == who || final_move => None,
            Some(_) => self.maybe_move(who, pos, dy * 2, dx * 2, true),
        }
    }

    pub fn valid_moves(&self, who: Piece) -> Vec<Move> {
        let going_up = who == self.player;

        let piece_positions = self
            .iter_pieces()
            .filter_map(|(y, x, piece)| (piece == who).then_some((y, x)));

        let mut moves = Vec::new();

        let dy = if going_up { -1 } else { 1 };
        for pos in piece_positions {
            if let Some(left) = self.maybe_move(who, pos, dy, -1, false) {
                moves.push(left);
            }
            if let Some(right) = self.maybe_move(who, pos, dy, 1, false) {
                moves.push(right);
            }
        }

        moves
    }

    pub fn piece_for_turn(&self) -> Piece {
        match self.turn {
            Turn::Player => self.player,
            Turn::Ai => self.player.opposite(),
        }
    }

    pub fn apply_move(&self, m: Move) -> Self {
        let mut board = self.board;

        if m.d.0.abs() > 1 {
            let d = (m.d.0 / 2, m.d.1 / 2);
            let y = m.from.0.checked_add_signed(d.0).unwrap();
            let x = m.from.1.checked_add_signed(d.1).unwrap();

            debug_assert_ne!(board[y][x], None);
            board[y][x] = None;
        }

        let y = m.from.0.checked_add_signed(m.d.0).unwrap();
        let x = m.from.1.checked_add_signed(m.d.1).unwrap();

        debug_assert_eq!(board[y][x], None);
        board[y][x] = board[m.from.0][m.from.1];
        board[m.from.0][m.from.1] = None;

        Checkers {
            board,
            player: self.player,
            turn: self.turn.opposite(),
        }
    }

    /// Returns which player (if any) has won the game.
    /// The winner is decided in one of those cases:
    ///   - When the other player has lost all of their pieces
    ///   - When the player has reached the end of the board with one of their pieces
    pub fn get_winner(&self) -> Option<Turn> {
        let black_won = if self.player == Piece::Black {
            Turn::Player
        } else {
            Turn::Ai
        };

        // First condition
        let (black_count, white_count) = self.iter_pieces().fold((0, 0), |mut acc, (.., curr)| {
            match curr {
                Piece::Black => acc.0 += 1,
                Piece::White => acc.1 += 1,
            };
            acc
        });
        if black_count == 0 {
            return Some(black_won.opposite());
        }
        if white_count == 0 {
            return Some(black_won);
        }

        // Second condition
        // Player pieces are always at the bottom and go towards the top
        let player_won = self.board[0]
            .iter()
            .any(|cell| cell.is_some_and(|piece| piece == self.player));
        if player_won {
            return Some(Turn::Player);
        }

        let ai_won = self.board[BOARD_SIZE - 1]
            .iter()
            .any(|cell| cell.is_some_and(|piece| piece == self.player.opposite()));
        if ai_won {
            return Some(Turn::Ai);
        }

        None
    }
}
