#![doc = include_str!("../README.md")]

use std::env::var;

use discorec::{Dataset, RecommenderBuilder};
use indicatif::ProgressBar;
use inquire::Select;
use rekomendacje::{ImdbTitle, User};

const MOVIES_STR: &str = include_str!(concat!(env!("REKOMENDACJE_DATA_DIR"), "/movies.json"));
const USERS_STR: &str = include_str!(concat!(env!("REKOMENDACJE_DATA_DIR"), "/users.json"));
const EXTRA_USERS_STR: &str =
    include_str!(concat!(env!("REKOMENDACJE_DATA_DIR"), "/extra_users.json"));

const ITERATIONS: u32 = 128;
const FACTORS: u32 = 16;

fn main() {
    let movies: Vec<ImdbTitle> = serde_json::from_str(MOVIES_STR).unwrap();
    let users: Vec<User> = serde_json::from_str(USERS_STR).unwrap();
    let extra_users: Vec<User> = serde_json::from_str(EXTRA_USERS_STR).unwrap();

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
    let bar =
        ProgressBar::new(iterations.into()).with_message("Inicjalizacja silnika rekomendacji");

    let engine = RecommenderBuilder::new()
        .iterations(iterations)
        .factors(factors)
        .callback(|_info| _ = bar.inc(1))
        .fit_explicit(&set);

    bar.finish_and_clear();

    let show_all_users = var("REC_DEBUG_ALL_USERS")
        .ok()
        .is_some_and(|v| matches!(v.to_lowercase().as_str(), "1" | "true"));

    let options = if show_all_users {
        users
            .iter()
            .chain(&extra_users)
            .map(|user| user.name.as_str())
            .collect()
    } else {
        users.iter().map(|user| user.name.as_str()).collect()
    };
    let user = Select::new("Kim jesteś", options).prompt().unwrap();

    let recs = engine.user_recs(&user, movies.len());

    let rec_count = (recs.len() / 2).min(5);

    let top_recs = recs.iter().take(rec_count);
    let anti_recs = recs.iter().rev().take(rec_count);

    println!("Rekomendacje:");
    for &(&id, score) in top_recs {
        let movie = movies.iter().find(|movie| movie.id == id).unwrap();
        println!(
            "{} ({}) - {}",
            movie.original_title,
            movie.start_year.unwrap(),
            score
        );
    }

    println!("\nAnty-rekomendacje:");
    for &(&id, score) in anti_recs {
        let movie = movies.iter().find(|movie| movie.id == id).unwrap();
        println!(
            "{} ({}) - {}",
            movie.original_title,
            movie.start_year.unwrap(),
            score
        );
    }
}
