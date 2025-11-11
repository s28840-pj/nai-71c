use indicatif::ProgressBar;
use inquire::Select;
use reqwest::{Url, blocking as r};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Debug, Display, Write},
    fs,
    iter::{Fuse, FusedIterator},
    mem::MaybeUninit,
    process::exit,
    time::Duration,
};
use strsim::normalized_damerau_levenshtein;

#[derive(Debug, Serialize)]
struct Rating {
    movie: String,
    rating: u32,
}

impl Rating {
    fn new(movie: String, rating: u32) -> Self {
        Rating { movie, rating }
    }
}

#[derive(Debug, Serialize)]
struct User {
    #[allow(unused)]
    name: String,
    ratings: Vec<Rating>,
}

fn bail_usage() -> ! {
    eprintln!(r#"usage: normalize [input file] [output directory]"#);
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ImdbTitle {
    id: String,
    primary_title: String,
    original_title: String,
    #[serde(default)]
    start_year: Option<u32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ImdbSearchResult {
    titles: Vec<ImdbTitle>,
}

fn is_almost_equal(a: &str, b: &str) -> bool {
    let d = normalized_damerau_levenshtein(&a.to_lowercase(), &b.to_lowercase());
    d >= 0.9
}

struct SelectOption<T> {
    inner: T,
    fmt: fn(&T) -> String,
}

impl<T> SelectOption<T> {
    fn new(inner: T, fmt: fn(&T) -> String) -> Self {
        Self { inner, fmt }
    }
}

impl<T> Display for SelectOption<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = (self.fmt)(&self.inner);
        write!(f, "{s}")
    }
}

fn prompt_movie<'a>(
    name: &str,
    potential: impl Iterator<Item = &'a ImdbTitle>,
    bar: &ProgressBar,
) -> ImdbTitle {
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
        result = Some(
            Select::new(&prompt, options)
                .prompt()
                .unwrap()
                .inner
                .clone(),
        );
    });

    result.unwrap()
}

fn find_og_title(name: &str, bar: &ProgressBar) -> ImdbTitle {
    let url = Url::parse_with_params(
        "https://api.imdbapi.dev/search/titles",
        &[("query", name), ("limit", "10")],
    )
    .unwrap();
    let response: ImdbSearchResult = r::get(url).unwrap().json().unwrap();

    if response.titles.len() == 1 {
        return response.titles[0].clone();
    } else if response.titles.is_empty() {
        panic!("no potential matches found for {name:?}");
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
        return exactly_the_same[0].clone();
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
        return basically_the_same[0].clone();
    }

    prompt_movie(name, response.titles.iter(), bar)
}

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();

    let mut get_arg = || {
        if let Some(v) = args.next() {
            v
        } else {
            bail_usage()
        }
    };

    let input = get_arg();
    let output = get_arg();

    let raw = unwrap_usage(fs::read_to_string(input));
    unwrap_usage(fs::create_dir_all(&output));

    let mut movies = HashMap::new();
    let mut users = Vec::new();

    let lines = raw.lines().filter_map(|line| {
        let line = line.trim();
        if line.is_empty() { None } else { Some(line) }
    });

    for line in lines {
        let mut components = line.split('\t');
        let name = components.next().expect("format to be correct");
        let name = name.trim().to_string();

        let ratings = components
            .arr_chunks()
            .map(|[movie_name, rating]| {
                let movie_name = movie_name.trim();
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

    let mut normalized_movies = Vec::<ImdbTitle>::new();

    let bar = ProgressBar::new(movies.len() as _);
    bar.enable_steady_tick(Duration::from_millis(100));

    for (name, id) in movies.iter() {
        let title = find_og_title(name, &bar);
        let normal_movie_id = title.id.clone();
        if !normalized_movies.iter().any(|movie| movie.id == title.id) {
            normalized_movies.push(title);
        }

        for user in users.iter_mut() {
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
