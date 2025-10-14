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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Winner {
    Won(Turn),
    Draw,
    InProgress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    Player,
    Ai,
}

impl Piece {
    pub fn opposite(self) -> Self {
        match self {
            Piece::Black => Piece::White,
            Piece::White => Piece::Black,
        }
    }
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
    /// Create a new game board, with the given piece color for the [`Player`](Turn::Player).
    /// The convention assumed here is [`Player`](Turn::Player) is always at the bottom,
    /// and [`AI`](Turn::Ai) is always at the top.
    ///
    /// (Bottom and top referring here to the row index, with "top" row being index 0,
    /// and "bottom" row being BOARD_SIZE - 1)
    ///
    /// A visialization of the board could choose to display the board differently,
    /// or to give control over the [`AI`](Turn::Ai) pieces.
    pub fn new(player: Piece) -> Self {
        let board = std::array::from_fn(|y| {
            if let 3 | 4 = y {
                // Rows 3 and 4 are always fully empty.
                [None; BOARD_SIZE]
            } else {
                // For all other rows, the cell has a piece when:
                //   - For even rows, if the column is odd
                //   - For odd rows, if the column is even
                let offset = !y & 1; // offset will be 1 for even rows, and 0 for odd
                // Upper half is always the AI, lower half is always the human player
                let piece = if y < BOARD_SIZE / 2 {
                    player.opposite()
                } else {
                    player
                };
                // Set piece on the cell if x + offset is even.
                // Thus for even rows, this will be cells 1, 3, 5, ...
                // and for odd rows this will be cells 0, 2, 4, ...
                std::array::from_fn(|x| ((x + offset) & 1 == 0).then_some(piece))
            }
        });

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

    /// Returns who'll move next.
    pub fn turn(&self) -> Turn {
        self.turn
    }

    /// Returns given cell of the board.
    pub fn cell(&self, (y, x): (usize, usize)) -> Option<Piece> {
        self.board[y][x]
    }

    /// Returns the color of player's pieces.
    pub fn player(&self) -> Piece {
        self.player
    }

    /// Returns an iterator over all rows of the board.
    /// Indexing starts at 0.
    pub fn iter_rows(&self) -> impl Iterator<Item = (usize, &[Option<Piece>; BOARD_SIZE])> {
        self.board.iter().enumerate()
    }

    /// Returns an iterator over all cells of the board.
    /// Indexing, both for rows and columns, starts at 0.
    pub fn iter_board(&self) -> impl Iterator<Item = (usize, usize, Option<Piece>)> {
        self.board
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &cell)| (y, x, cell)))
    }

    /// Returns an iterator over all pieces still on the board.
    /// Indexing, both for rows and columns, starts at 0.
    pub fn iter_pieces(&self) -> impl Iterator<Item = (usize, usize, Piece)> {
        self.iter_board().filter_map(|(y, x, cell)| {
            let piece = cell?;
            Some((y, x, piece))
        })
    }

    /// Validates and normalizes a move (e.g. moving it over an opponent, if possible).
    /// For invalid moves returns None.
    fn maybe_move(
        &self,
        who: Piece,
        pos: (usize, usize),
        dy: isize,
        dx: isize,
        final_move: bool,
    ) -> Option<Move> {
        /// Validate target coordinates first (avoid panics).
        debug_assert_eq!(Some(who), self.cell(pos));

        let new_y = pos.0.checked_add_signed(dy).filter(|&n| n < BOARD_SIZE)?;
        let new_x = pos.1.checked_add_signed(dx).filter(|&n| n < BOARD_SIZE)?;

        // If the target cell is empty, this is a valid move.
        // If it's occupied by an opponent, try to jump over it (double the delta).
        // If it's occupied by our own piece or we've already tried a jump (final_move),
        // the move is invalid.
        match self.cell((new_y, new_x)) {
            None => Some(Move {
                from: pos,
                d: (dy, dx),
            }),
            Some(p) if p == who || final_move => None,
            Some(_) => self.maybe_move(who, pos, dy * 2, dx * 2, true),
        }
    }

    /// Returns all valid [`Move`]s for the given color, given the current state of the game.
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

    /// Returns color of the piece who'll move next.
    pub fn piece_for_turn(&self) -> Piece {
        match self.turn {
            Turn::Player => self.player,
            Turn::Ai => self.player.opposite(),
        }
    }

    /// Returns a copy of the board, after applying a valid [`Move`].
    ///
    /// # Panics
    ///
    /// Panics if the given [`Move`] goes out of bounds.
    pub fn apply_move(&self, m: Move) -> Self {
        let mut board = self.board;

        /// If the move is a jump (distance > 1), remove the jumped piece.
        if m.d.0.abs() > 1 {
            /// Get the coordinates of the jumped piece.
            let d = (m.d.0 / 2, m.d.1 / 2);
            let y = m.from.0.checked_add_signed(d.0).unwrap();
            let x = m.from.1.checked_add_signed(d.1).unwrap();

            debug_assert_ne!(board[y][x], None);
            board[y][x] = None;
        }

        /// Move the piece to the target cell and clear the source cell.
        let y = m.from.0.checked_add_signed(m.d.0).unwrap();
        let x = m.from.1.checked_add_signed(m.d.1).unwrap();

        debug_assert_eq!(board[y][x], None);
        board[y][x] = board[m.from.0][m.from.1];
        board[m.from.0][m.from.1] = None;

        /// Switch turn to the other player.
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
    pub fn get_winner(&self) -> Winner {
        let black_won = if self.player == Piece::Black {
            Turn::Player
        } else {
            Turn::Ai
        };

        /// First condition
        /// One side has no pieces left.
        let (black_count, white_count) = self.iter_pieces().fold((0, 0), |mut acc, (.., curr)| {
            match curr {
                Piece::Black => acc.0 += 1,
                Piece::White => acc.1 += 1,
            };
            acc
        });
        if black_count == 0 {
            return Winner::Won(black_won.opposite());
        }
        if white_count == 0 {
            return Winner::Won(black_won);
        }

        /// Second condition
        /// Player pieces are always at the bottom and go towards the top
        /// Check if any of the player's pieces reached the top row
        let player_won = self.board[0]
            .iter()
            .any(|cell| cell.is_some_and(|piece| piece == self.player));
        if player_won {
            return Winner::Won(Turn::Player);
        }

        /// Check if any of the AI's pieces reached the bottom row
        let ai_won = self.board[BOARD_SIZE - 1]
            .iter()
            .any(|cell| cell.is_some_and(|piece| piece == self.player.opposite()));
        if ai_won {
            return Winner::Won(Turn::Ai);
        }

        /// If none of the win conditions are met, check if the next player has any valid moves left,
        /// to determine whether the game ended in draw
        if self.valid_moves(self.piece_for_turn()).is_empty() {
            Winner::Draw
        } else {
            Winner::InProgress
        }
    }
}
