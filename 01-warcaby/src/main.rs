//! Simplified implementation of the game checkers in the terminal.

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use minimax::{Negamax, Strategy};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Cell, List, ListState, Row, Table, TableState},
};

use crate::ai::CheckerEval;

const BOARD_SIZE: usize = 8;

mod ai;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Piece {
    Black,
    White,
}

impl Piece {
    fn opposite(self) -> Self {
        match self {
            Piece::Black => Piece::White,
            Piece::White => Piece::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Player,
    Ai,
}

impl Turn {
    fn opposite(self) -> Self {
        match self {
            Turn::Player => Turn::Ai,
            Turn::Ai => Turn::Player,
        }
    }
}

#[derive(Clone)]
pub struct Checkers {
    board: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    player: Piece,
    turn: Turn,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    from: (usize, usize),
    d: (isize, isize),
}

impl Checkers {
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

    fn valid_moves(&self, who: Piece) -> Vec<Move> {
        let going_up = who == self.player;

        let piece_positions = self.board.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, cell)| match cell {
                    Some(p) if *p == who => Some((y, x)),
                    _ => None,
                })
        });

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

    fn piece_for_turn(&self) -> Piece {
        match self.turn {
            Turn::Player => self.player,
            Turn::Ai => self.player.opposite(),
        }
    }

    fn apply_move(&self, m: Move) -> Self {
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
    fn get_winner(&self) -> Option<Turn> {
        let black_won = if self.player == Piece::Black {
            Turn::Player
        } else {
            Turn::Ai
        };

        // First condition
        let (black_count, white_count) = self
            .board
            .iter()
            .flat_map(|row| row.iter().filter_map(|cell| *cell))
            .fold((0, 0), |mut acc, curr| {
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

type AI = Negamax<CheckerEval>;

struct InGame {
    game: Checkers,
    selected: (usize, usize),
    moving_piece: Option<(usize, usize)>,
    valid_moves: Vec<(usize, usize)>,
    ai: AI,
}

#[derive(Default)]
struct PickingSides {
    state: ListState,
}

impl PickingSides {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<Option<InGame>> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            match event::read()? {
                Event::Key(e) if e.is_press() => match self.handle_key(e) {
                    EventResult::Continue => {}
                    EventResult::Quit => return Ok(None),
                    EventResult::End(state) => return Ok(Some(state)),
                },
                _ => {}
            }
        }
    }

    fn handle_key(&mut self, event: KeyEvent) -> EventResult<InGame> {
        match event.code {
            KeyCode::Char('q') => return EventResult::Quit,
            KeyCode::Char('j') | KeyCode::Down => self.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.state.select_previous(),
            KeyCode::Char(' ') | KeyCode::Enter => {
                if let Some(opt) = self.state.selected() {
                    let player = match opt {
                        0 => Piece::White,
                        1 => Piece::Black,
                        _ => unreachable!(),
                    };
                    return EventResult::End(InGame::new(player));
                }
            }
            _ => {}
        }
        EventResult::Continue
    }

    fn render(&mut self, frame: &mut Frame) {
        let options = ["White", "Black"].map(|line| Text::from(line).centered());
        let list = List::new(options).highlight_style(Style::new().bg(Color::Blue));

        frame.render_stateful_widget(list, frame.area(), &mut self.state);
    }
}

struct GameEnded(&'static str);

impl GameEnded {
    pub fn won() -> Self {
        GameEnded("Congratulations, you win! :D")
    }

    pub fn lost() -> Self {
        GameEnded("Unfortunately, you lose :(")
    }

    pub fn draw() -> Self {
        GameEnded("No more available moves, it's a draw")
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            match event::read()? {
                Event::Key(e) if e.is_press() => return Ok(()),
                _ => {}
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let text = Text::from(self.0).centered();
        frame.render_widget(text, frame.area());
    }
}

enum App {
    PickingSides(PickingSides),
    InGame(InGame),
    GameEnded(GameEnded),
}

impl App {
    fn new() -> App {
        App::PickingSides(PickingSides::default())
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            match self {
                App::PickingSides(state) => {
                    let Some(new_state) = state.run(&mut terminal)? else {
                        return Ok(());
                    };
                    *self = App::InGame(new_state);
                }
                App::InGame(state) => {
                    let Some(new_state) = state.run(&mut terminal)? else {
                        return Ok(());
                    };
                    *self = App::GameEnded(new_state);
                }
                App::GameEnded(state) => return state.run(&mut terminal),
            };
        }
    }
}

impl Checkers {
    fn new(player: Piece) -> Self {
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
}

#[must_use]
enum EventResult<S> {
    Continue,
    Quit,
    End(S),
}

impl InGame {
    pub fn new(player: Piece) -> InGame {
        let ai = Negamax::new(CheckerEval, 6);

        let mut state = InGame {
            game: Checkers::new(player),
            selected: (0, 0),
            moving_piece: None,
            valid_moves: Vec::new(),
            ai,
        };

        if state.game.turn == Turn::Ai {
            let res = state.ai_turn();
            // Game should never end after the first turn
            debug_assert!(matches!(res, EventResult::Continue));
        }

        state
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<Option<GameEnded>> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            match event::read()? {
                Event::Key(e) if e.is_press() => match self.handle_key(e) {
                    EventResult::Continue => {}
                    EventResult::Quit => return Ok(None),
                    EventResult::End(state) => return Ok(Some(state)),
                },
                _ => {}
            }
        }
    }

    fn handle_key(&mut self, event: KeyEvent) -> EventResult<GameEnded> {
        match event.code {
            KeyCode::Char('q') => return EventResult::Quit,
            KeyCode::Char('h') | KeyCode::Left if self.selected.1 > 0 => {
                self.selected.1 -= 1;
            }
            KeyCode::Char('l') | KeyCode::Right if self.selected.1 < BOARD_SIZE - 1 => {
                self.selected.1 += 1;
            }
            KeyCode::Char('j') | KeyCode::Down if self.selected.0 < BOARD_SIZE - 1 => {
                self.selected.0 += 1
            }
            KeyCode::Char('k') | KeyCode::Up if self.selected.0 > 0 => self.selected.0 -= 1,
            KeyCode::Char(' ') | KeyCode::Enter
                if self.game.board[self.selected.0][self.selected.1]
                    .is_some_and(|p| p == self.game.player) =>
            {
                self.moving_piece = Some(self.selected);
                self.valid_moves = self
                    .game
                    .valid_moves(self.game.player)
                    .iter()
                    .filter_map(|m| {
                        if m.from == self.selected {
                            Some((
                                m.from.0.checked_add_signed(m.d.0).unwrap(),
                                m.from.1.checked_add_signed(m.d.1).unwrap(),
                            ))
                        } else {
                            None
                        }
                    })
                    .collect();
            }
            KeyCode::Char(' ') | KeyCode::Enter
                if self.moving_piece.is_some()
                    && self.valid_moves.iter().any(|&m| m == self.selected) =>
            {
                let from = self.moving_piece.unwrap();
                let d = (
                    self.selected.0 as isize - from.0 as isize,
                    self.selected.1 as isize - from.1 as isize,
                );
                let move_to_do = Move { from, d };

                debug_assert_eq!(self.game.turn, Turn::Player);
                self.game = self.game.apply_move(move_to_do);
                self.moving_piece = None;
                self.valid_moves = Vec::new();

                if let Some(winner) = self.game.get_winner() {
                    debug_assert_eq!(winner, Turn::Player);
                    return EventResult::End(GameEnded::won());
                }

                return self.ai_turn();
            }
            KeyCode::Char(' ') | KeyCode::Enter | KeyCode::Esc => {
                self.moving_piece = None;
                self.valid_moves = Vec::new();
            }
            _ => {}
        }
        EventResult::Continue
    }

    fn ai_turn(&mut self) -> EventResult<GameEnded> {
        debug_assert_eq!(self.game.turn, Turn::Ai);
        let move_to_do = self.ai.choose_move(&self.game).unwrap();
        self.game = self.game.apply_move(move_to_do);

        if let Some(winner) = self.game.get_winner() {
            debug_assert_eq!(winner, Turn::Ai);
            return EventResult::End(GameEnded::lost());
        }

        // If after AI plays the player has no more valid moves,
        // that must mean the game ended in a draw
        if self.game.valid_moves(self.game.piece_for_turn()).is_empty() {
            return EventResult::End(GameEnded::draw());
        }

        EventResult::Continue
    }

    fn render(&mut self, frame: &mut Frame) {
        let rows = self.game.board.iter().enumerate().map(|(y, row)| {
            Row::new(row.iter().enumerate().map(|(x, cell)| {
                let is_dark = x & 1 == !y & 1;
                let bg = if is_dark { Color::Green } else { Color::Gray };
                let style = Style::new().bg(bg);
                let Some(piece) = cell else {
                    let content = if self.valid_moves.iter().any(|&p| p == (y, x)) {
                        " ● "
                    } else {
                        ""
                    };
                    return Cell::new(content).style(style);
                };
                let mut style = style.fg(match piece {
                    Piece::Black => Color::Red,
                    Piece::White => Color::White,
                });
                if let Some(pos) = self.moving_piece
                    && pos == (y, x)
                {
                    style = style.bg(Color::Blue);
                }
                Cell::new(" ⬤ ").style(style)
            }))
        });
        let widths = std::iter::repeat_n(Constraint::Length(3), BOARD_SIZE);

        let mut state = TableState::new().with_selected_cell(self.selected);
        let table = Table::new(rows, widths)
            .column_spacing(0)
            .cell_highlight_style(Style::new().bg(Color::Magenta));

        let h = Layout::horizontal([Constraint::Length(BOARD_SIZE as u16 * 3)]).flex(Flex::Center);
        let v = Layout::vertical([Constraint::Length(BOARD_SIZE as u16)]).flex(Flex::Center);
        let [area] = h.areas(frame.area());
        let [area] = v.areas(area);

        frame.render_stateful_widget(table, area, &mut state);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut app = App::new();
    app.run(terminal)?;
    ratatui::restore();
    Ok(())
}
