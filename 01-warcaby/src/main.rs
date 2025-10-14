//! Simplified implementation of the game checkers in the terminal.

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use minimax::Negamax;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Cell, List, ListState, Row, Table, TableState},
};

use crate::{
    ai::CheckerEval,
    game::{BOARD_SIZE, Checkers, Move, Piece, Turn, Winner},
};

mod ai;
mod game;

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

        /// If AI moves first, perform its move immediately.
        if state.game.turn() == Turn::Ai {
            let res = state.ai_turn();
            /// Game should never end after the first turn
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
        /// Handle keyboard navigation, selection, confirmation and cancellation.
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
                if self
                    .game
                    .cell(self.selected)
                    .is_some_and(|p| p == self.game.player()) =>
            {
                /// Start moving: mark source and compute valid destinations for that piece.
                self.moving_piece = Some(self.selected);
                self.valid_moves = self
                    .game
                    .valid_moves(self.game.player())
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
                if self.moving_piece.is_some() && self.valid_moves.contains(&self.selected) =>
            {
                /// Confirm move: build the Move from selected source->dest and apply it.
                let from = self.moving_piece.unwrap();
                let d = (
                    self.selected.0 as isize - from.0 as isize,
                    self.selected.1 as isize - from.1 as isize,
                );
                let move_to_do = Move { from, d };

                debug_assert_eq!(self.game.turn(), Turn::Player);
                self.game = self.game.apply_move(move_to_do);
                self.moving_piece = None;
                self.valid_moves = Vec::new();

                match self.game.get_winner() {
                    Winner::Won(winner) => {
                        debug_assert_eq!(winner, Turn::Player);
                        return EventResult::End(GameEnded::won());
                    }
                    Winner::Draw => return EventResult::End(GameEnded::draw()),
                    Winner::InProgress => return self.ai_turn(),
                }
            }
            KeyCode::Char(' ') | KeyCode::Enter | KeyCode::Esc => {
                /// Cancel any in-progress selection/move.
                self.moving_piece = None;
                self.valid_moves = Vec::new();
            }
            _ => {}
        }
        EventResult::Continue
    }

    fn ai_turn(&mut self) -> EventResult<GameEnded> {
        /// Let the AI pick its move and apply it, then check for end condition.
        debug_assert_eq!(self.game.turn(), Turn::Ai);
        let move_to_do = self.ai.choose_move(&self.game).unwrap();
        self.game = self.game.apply_move(move_to_do);

        match self.game.get_winner() {
            Winner::Won(winner) => {
                debug_assert_eq!(winner, Turn::Ai);
                EventResult::End(GameEnded::lost())
            }
            Winner::Draw => EventResult::End(GameEnded::draw()),
            Winner::InProgress => EventResult::Continue,
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        /// Draw board cells, pieces, highlight valid destinations and currently moving piece.
        let rows = self.game.iter_rows().map(|(y, row)| {
            Row::new(row.iter().enumerate().map(|(x, cell)| {
                let is_dark = x & 1 == !y & 1;
                let bg = if is_dark { Color::Green } else { Color::Gray };
                let style = Style::new().bg(bg);
                let Some(piece) = cell else {
                    let content = if self.valid_moves.contains(&(y, x)) {
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
                    /// Change the board color for the piece currently selected for movement.
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
