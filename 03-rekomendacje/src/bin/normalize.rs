use indicatif::ProgressBar;
use inquire::{Select, prompt_text};
use reqwest::{Url, blocking as r};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::{Debug, Write},
    fs,
    iter::{Fuse, FusedIterator},
    mem::MaybeUninit,
    process::exit,
    time::Duration,
};
use strsim::normalized_damerau_levenshtein;

use rekomendacje::{ImdbTitle, Rating, SelectOption, User};

fn bail_usage() -> ! {
    eprintln!(r#"usage: normalize [input file] [...auxilary input files] [output directory]"#);
    exit(1);
}

fn unwrap_usage<T, E: Debug>(res: Result<T, E>) -> T {
    match res {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e:?}");
            bail_usage();
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ImdbSearchResult {
    #[serde(default)]
    titles: Vec<ImdbTitle>,
}

fn is_almost_equal(a: &str, b: &str) -> bool {
    let d = normalized_damerau_levenshtein(&a.to_lowercase(), &b.to_lowercase());
    d >= 0.9
}

fn prompt_movie<'a>(
    name: &str,
    potential: impl Iterator<Item = &'a ImdbTitle>,
    bar: &ProgressBar,
) -> Option<ImdbTitle> {
    let prompt = format!("Found multiple matches for {name:?}");

    let options = potential
        .map(|title| {
            SelectOption::new(title, |title| {
                let mut res = title.primary_title.clone();
                if let Some(year) = title.start_year {
                    write!(res, " ({})", year).ok();
                }
                res
            })
        })
        .collect();

    let mut result = Option::None;

    bar.suspend(|| {
        result = Select::new(&prompt, options)
            .prompt()
            .ok()
            .map(|s| s.into_inner().clone());
    });

    result
}

fn find_og_title(name: &str, bar: &ProgressBar) -> Option<ImdbTitle> {
    let url = Url::parse_with_params(
        "https://api.imdbapi.dev/search/titles",
        &[("query", name), ("limit", "10")],
    )
    .unwrap();
    let response: ImdbSearchResult = r::get(url).unwrap().json().unwrap();

    if response.titles.len() == 1 {
        return Some(response.titles[0].clone());
    } else if response.titles.is_empty() {
        return None;
    }

    let exactly_the_same: Vec<_> = response
        .titles
        .iter()
        .filter(|title| {
            title.primary_title.to_lowercase() == name
                || title.original_title.to_lowercase() == name
        })
        .collect();

    if exactly_the_same.len() == 1 {
        return Some(exactly_the_same[0].clone());
    }

    let basically_the_same: Vec<_> = response
        .titles
        .iter()
        .filter(|title| {
            is_almost_equal(&title.primary_title, name)
                || is_almost_equal(&title.original_title, name)
        })
        .collect();

    if basically_the_same.len() == 1 {
        return Some(basically_the_same[0].clone());
    }

    prompt_movie(name, response.titles.iter(), bar)
}

fn users_from_csv(csv: &str, movies: &mut HashMap<String, String>) -> Vec<User> {
    let mut users = Vec::new();

    let lines = csv.lines().filter_map(|line| {
        let line = line.trim();
        if line.is_empty() { None } else { Some(line) }
    });

    for line in lines {
        let mut components = line.split('\t').map(|s| s.trim()).filter(|s| !s.is_empty());
        let name = components.next().expect("format to be correct");
        let name = name.to_string();

        let ratings = components
            .arr_chunks()
            .map(|[movie_name, rating]| {
                let new_id = movies.len();
                let movie_id = movies
                    .entry(movie_name.to_lowercase())
                    .or_insert_with(|| new_id.to_string())
                    .clone();
                let rating = rating.parse().expect("format to be correct");
                Rating::new(movie_id, rating)
            })
            .collect();

        users.push(User { name, ratings });
    }

    users
}

fn main() {
    let mut args = std::env::args().skip(1);

    if args.len() < 2 {
        bail_usage();
    }
    let aux_count = args.len() - 2;
    let mut get_arg = || {
        if let Some(v) = args.next() {
            v
        } else {
            bail_usage()
        }
    };

    let input = get_arg();
    let aux: Vec<_> = (0..aux_count).map(|_| get_arg()).collect();
    let output = get_arg();

    let raw = unwrap_usage(fs::read_to_string(input));
    unwrap_usage(fs::create_dir_all(&output));

    let aux = aux
        .into_iter()
        .map(|aux_file| unwrap_usage(fs::read_to_string(aux_file)))
        .collect::<Vec<_>>()
        .join("\n");

    let mut movies = HashMap::new();
    let mut users = users_from_csv(&raw, &mut movies);
    let mut aux_users = users_from_csv(&aux, &mut movies);

    let mut normalized_movies = Vec::<ImdbTitle>::new();

    let bar = ProgressBar::new(movies.len() as _);
    bar.enable_steady_tick(Duration::from_millis(100));

    for (name, id) in movies.iter() {
        let title = match find_og_title(name, &bar) {
            Some(og) => og,
            None => {
                let mut new_name = None;
                bar.suspend(|| {
                    new_name = Some(prompt_text(format!(
                        "No potential matches found for {name:?}. Enter alternative title you want to look up:"
                    ))
                    .unwrap());
                });
                let new_name = new_name.unwrap();
                find_og_title(&new_name, &bar).unwrap()
            }
        };
        let normal_movie_id = title.id.clone();
        if !normalized_movies.iter().any(|movie| movie.id == title.id) {
            normalized_movies.push(title);
        }

        for user in users.iter_mut().chain(aux_users.iter_mut()) {
            for rating in user.ratings.iter_mut() {
                if &rating.movie == id {
                    rating.movie = normal_movie_id.clone();
                }
            }
        }

        bar.inc(1);
    }

    bar.finish();

    fs::write(
        format!("{output}/movies.json"),
        serde_json::to_string(&normalized_movies).unwrap(),
    )
    .unwrap();

    fs::write(
        format!("{output}/users.json"),
        serde_json::to_string(&users).unwrap(),
    )
    .unwrap();

    fs::write(
        format!("{output}/extra_users.json"),
        serde_json::to_string(&aux_users).unwrap(),
    )
    .unwrap();
}

// === utilities ===
struct Chunks<I: FusedIterator, const N: usize> {
    iter: I,
}

impl<I: FusedIterator, const N: usize> Iterator for Chunks<I, N> {
    type Item = [I::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = MaybeUninit::<Self::Item>::uninit();

        for i in 0..N {
            let elem = self.iter.next()?;
            // SAFETY: We're writing to the array, not reading from it,
            //         and the index is guaranteed to be in bounds.
            unsafe {
                let r = &mut *res.as_mut_ptr();
                *r.get_unchecked_mut(i) = elem;
            }
        }

        // SAFETY: We know we have filled in all the array elements
        Some(unsafe { res.assume_init() })
    }
}

impl<I: FusedIterator, const N: usize> FusedIterator for Chunks<I, N> {}

trait IterExt: Iterator {
    /// Iterator::array_chunks, but stable
    fn arr_chunks<const N: usize>(self) -> Chunks<Fuse<Self>, N>
    where
        Self: Sized,
    {
        assert!(N != 0, "chunk size must be non-zero");
        Chunks { iter: self.fuse() }
    }
}

impl<I: Iterator> IterExt for I {}
