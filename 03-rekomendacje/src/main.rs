#![doc = include_str!("../README.md")]

use std::{
    env::var,
    path::{Path, PathBuf},
    sync::mpsc,
};

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use directories::ProjectDirs;
use discorec::{Dataset, Recommender, RecommenderBuilder};
use image::ImageReader;
use ratatui::{
    DefaultTerminal, Frame, TerminalOptions, Viewport,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Gauge, List, ListItem, ListState, Paragraph, Wrap},
};
use ratatui_image::{Resize, StatefulImage, picker::Picker};
use rekomendacje::{ImdbTitle, User};
use reqwest::Url;

const MOVIES_STR: &str = include_str!(concat!(env!("REKOMENDACJE_DATA_DIR"), "/movies.json"));
const USERS_STR: &str = include_str!(concat!(env!("REKOMENDACJE_DATA_DIR"), "/users.json"));
const EXTRA_USERS_STR: &str =
    include_str!(concat!(env!("REKOMENDACJE_DATA_DIR"), "/extra_users.json"));

const ITERATIONS: u32 = 128;
const FACTORS: u32 = 16;

struct App<'a> {
    movies: &'a [ImdbTitle],
    users: &'a [User],
    engine: Recommender<&'a str, &'a str>,
    thumb_cache: &'a Path,
    state: AppState<'a>,
}

enum AppState<'a> {
    SelectingUser {
        hovered: usize,
    },
    CheckingRecs {
        top_recs: Vec<&'a str>,
        anti_recs: Vec<&'a str>,
        anti_recs_hovered: bool,
        hovered: usize,
    },
}

impl<'a> App<'a> {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            match event::read()? {
                Event::Key(e) if e.is_press() => {
                    if !self.handle_key(e) {
                        break;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, event: KeyEvent) -> bool {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Char('j') | KeyCode::Down => self.hovered_diff(1),
            KeyCode::Char('k') | KeyCode::Up => self.hovered_diff(-1),
            KeyCode::Enter | KeyCode::Char(' ') => {
                if let AppState::SelectingUser { hovered } = self.state {
                    let recs = self
                        .engine
                        .user_recs(&self.users[hovered].name.as_str(), self.movies.len());
                    let rec_count = (recs.len() / 2).min(5);
                    let top_recs = recs.iter().map(|&(&id, _)| id).take(rec_count).collect();
                    let anti_recs = recs
                        .iter()
                        .map(|&(&id, _)| id)
                        .rev()
                        .take(rec_count)
                        .collect();
                    self.state = AppState::CheckingRecs {
                        top_recs,
                        anti_recs,
                        anti_recs_hovered: false,
                        hovered: 0,
                    };
                }
            }
            KeyCode::Tab => {
                if let AppState::CheckingRecs {
                    ref mut anti_recs_hovered,
                    ref mut hovered,
                    ..
                } = self.state
                {
                    *anti_recs_hovered = !*anti_recs_hovered;
                    *hovered = 0;
                }
            }
            _ => {}
        }
        true
    }

    fn hovered_diff(&mut self, diff: isize) {
        match &mut self.state {
            AppState::SelectingUser { hovered } => {
                *hovered = hovered.saturating_add_signed(diff).min(self.users.len());
            }
            AppState::CheckingRecs {
                hovered,
                top_recs,
                anti_recs,
                anti_recs_hovered,
            } => {
                let set = if *anti_recs_hovered {
                    anti_recs
                } else {
                    top_recs
                };
                *hovered = hovered.saturating_add_signed(diff).min(set.len());
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        match &self.state {
            AppState::SelectingUser { hovered } => {
                App::render_selecting_user(frame, *hovered, self.users)
            }
            AppState::CheckingRecs {
                hovered,
                top_recs,
                anti_recs,
                anti_recs_hovered,
            } => App::render_recs(
                frame,
                self.thumb_cache,
                self.movies,
                top_recs,
                anti_recs,
                *anti_recs_hovered,
                *hovered,
            ),
        }
    }

    fn render_selecting_user(frame: &mut Frame, hovered: usize, users: &'a [User]) {
        let v = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [label, rest] = v.areas(frame.area());

        let text = Text::raw("Kim jesteś").centered();

        let item_style = Style::new();
        let items = users
            .iter()
            .map(|user| ListItem::new(user.name.as_str()).style(item_style));
        let list =
            List::new(items).highlight_style(Style::new().bg(Color::LightBlue).fg(Color::Black));
        let mut state = ListState::default().with_selected(Some(hovered));

        frame.render_widget(text, label);
        frame.render_stateful_widget(list, rest, &mut state);
    }

    fn thumb_path(movie: &ImdbTitle, cache: &Path) -> Option<PathBuf> {
        std::fs::create_dir_all(cache).unwrap();
        let url = movie.primary_image.as_ref()?.url.as_str();
        let url = Url::parse(url).unwrap();
        let (_, ext) = url.path().rsplit_once('.').unwrap();
        let mut p = cache.join(&movie.id);
        p.set_extension(ext);
        if p.is_file() {
            return Some(p);
        }
        let img = reqwest::blocking::get(url)
            .unwrap()
            .error_for_status()
            .unwrap()
            .bytes()
            .unwrap();
        std::fs::write(&p, img).unwrap();
        Some(p)
    }

    fn render_recs(
        frame: &mut Frame,
        cache: &Path,
        movies: &[ImdbTitle],
        top_recs: &[&str],
        anti_recs: &[&str],
        anti_recs_hovered: bool,
        hovered: usize,
    ) {
        let v = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]);
        let h = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [area, help] = v.areas(frame.area());
        let [left, right] = h.areas(area);

        let v = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [title_area, mut right] = v.areas(right);

        let split = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [top_area, anti_area] = split.areas(left);

        let highlight_style = Style::new().bg(Color::LightBlue).fg(Color::Black);

        let list_from_ids = |ids: &[&str]| {
            List::new(ids.iter().map(|id| {
                let movie = movies.iter().find(|m| m.id == *id).unwrap();

                movie
                    .original_title
                    .as_deref()
                    .unwrap_or(&movie.primary_title)
            }))
            .highlight_style(highlight_style)
        };

        let top_text = Text::raw("Rekomendacje").centered();
        let top_list = list_from_ids(top_recs);
        let mut top_state = ListState::default();

        let anti_text = Text::raw("Anty-rekomendacje").centered();
        let anti_list = list_from_ids(anti_recs);
        let mut anti_state = ListState::default();

        if anti_recs_hovered {
            anti_state.select(Some(hovered));
        } else {
            top_state.select(Some(hovered));
        }

        let selected_movie = {
            let set = if anti_recs_hovered {
                anti_recs
            } else {
                top_recs
            };
            let id = set[hovered];
            movies.iter().find(|m| m.id == id).unwrap()
        };

        let movie_title = Text::raw(
            selected_movie
                .original_title
                .as_deref()
                .unwrap_or(&selected_movie.primary_title),
        )
        .centered();

        if let Some(movie_thumb) = App::thumb_path(selected_movie, cache) {
            let v = Layout::vertical([Constraint::Percentage(60), Constraint::Fill(1)]);
            let [image_area, new_right] = v.areas(right);
            right = new_right;

            // let picker = Picker::from_query_stdio().unwrap();
            let picker = Picker::from_fontsize((8, 12));
            let img = ImageReader::open(movie_thumb).unwrap().decode().unwrap();
            let mut img = picker.new_resize_protocol(img);
            frame.render_stateful_widget(
                StatefulImage::new().resize(Resize::Scale(None)),
                image_area,
                &mut img,
            );
        }

        let directors: Vec<_> = selected_movie
            .directors
            .iter()
            .map(|d| d.display_name.as_str())
            .collect();
        let directors = if directors.is_empty() {
            "BRAK INFORMACJI".to_string()
        } else {
            directors.join(", ")
        };
        let movie_info = format!(
            r#"{desc}

Reżyseria: {directors}
Rok premiery: {year}"#,
            desc = selected_movie.plot.as_deref().unwrap_or("BRAK OPISU"),
            year = selected_movie
                .start_year
                .map_or_else(|| "NIEZNANY".to_string(), |year| year.to_string())
        );
        let movie_info = Text::raw(movie_info);
        let movie_info = Paragraph::new(movie_info).wrap(Wrap { trim: true });

        let help_text =
            Text::raw("Poruszaj się używając strzałek. Przejdź do drugiej sekcji używając [Tab]");

        frame.render_widget(
            top_text,
            Rect {
                x: top_area.left(),
                y: top_area.top(),
                width: top_area.width,
                height: 1,
            },
        );
        frame.render_stateful_widget(
            top_list,
            Rect {
                x: top_area.left(),
                y: top_area.top().saturating_add(1),
                width: top_area.width,
                height: top_area.height.saturating_sub(1),
            },
            &mut top_state,
        );
        frame.render_widget(
            anti_text,
            Rect {
                x: anti_area.left(),
                y: anti_area.top(),
                width: anti_area.width,
                height: 1,
            },
        );
        frame.render_stateful_widget(
            anti_list,
            Rect {
                x: anti_area.left(),
                y: anti_area.top().saturating_add(1),
                width: anti_area.width,
                height: anti_area.height.saturating_sub(1),
            },
            &mut anti_state,
        );
        frame.render_widget(movie_title, title_area);
        frame.render_widget(movie_info, right);
        frame.render_widget(help_text, help);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(24),
    });

    let movies: Vec<ImdbTitle> = serde_json::from_str(MOVIES_STR)?;
    let users: Vec<User> = serde_json::from_str(USERS_STR)?;
    let extra_users: Vec<User> = serde_json::from_str(EXTRA_USERS_STR)?;

    let mut set = Dataset::new();

    for (user, rating) in users.iter().chain(&extra_users).flat_map(|user| {
        user.ratings
            .iter()
            .map(|rating| (user.name.as_str(), rating))
    }) {
        set.push(user, rating.movie.as_str(), rating.rating as f32);
    }

    let iterations = var("REC_ITERATIONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(ITERATIONS);
    let factors = var("REC_FACTORS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(FACTORS);

    let engine = std::thread::scope(|s| -> Result<_> {
        enum Event {
            Tick,
            End,
        }

        let (tx, rx) = mpsc::channel();

        let engine = s.spawn(move || {
            let result = RecommenderBuilder::new()
                .iterations(iterations)
                .factors(factors)
                .callback(|_info| _ = tx.send(Event::Tick))
                .fit_explicit(&set);
            tx.send(Event::End).ok();
            result
        });

        let mut progress = 0.;
        loop {
            terminal.draw(|frame| {
                let area = frame.area();
                let label = Text::raw("Ładowanie silnika rekomendacji").centered();
                let bar = Gauge::default()
                    .gauge_style(Style::default().fg(Color::Yellow).bg(Color::Black))
                    .ratio(progress / iterations as f64);
                frame.render_widget(
                    label,
                    Rect {
                        x: area.left(),
                        y: area.top(),
                        width: area.width,
                        height: 1,
                    },
                );
                frame.render_widget(
                    bar,
                    Rect {
                        x: area.left(),
                        y: area.top().saturating_add(1),
                        width: area.width,
                        height: 1,
                    },
                );
            })?;

            match rx.recv() {
                Ok(Event::End) => break,
                Ok(Event::Tick) => progress += 1.,
                // TODO: Handle errors?
                Err(_) => break,
            }
        }

        Ok(engine.join().unwrap())
    })?;

    let dirs = ProjectDirs::from("pj", "s28840", "rekomendacje").unwrap();
    let thumb_cache = dirs.cache_dir();
    let mut app = App {
        movies: &movies,
        users: &users,
        thumb_cache,
        engine,
        state: AppState::SelectingUser { hovered: 0 },
    };

    app.run(terminal)?;

    ratatui::restore();

    Ok(())
}
