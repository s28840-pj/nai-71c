#![doc = include_str!("../README.md")]

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    pub movie: String,
    pub rating: u32,
}

impl Rating {
    pub fn new(movie: String, rating: u32) -> Self {
        Rating { movie, rating }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[allow(unused)]
    pub name: String,
    pub ratings: Vec<Rating>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImdbTitle {
    pub id: String,
    pub primary_title: String,
    #[serde(default)]
    pub original_title: Option<String>,
    #[serde(default)]
    pub primary_image: Option<ImdbImage>,
    #[serde(default)]
    pub directors: Vec<ImdbName>,
    #[serde(default)]
    pub plot: Option<String>,
    #[serde(default)]
    pub start_year: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImdbName {
    pub id: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImdbImage {
    pub url: String,
}

pub struct SelectOption<T> {
    inner: T,
    fmt: fn(&T) -> String,
}

impl<T> SelectOption<T> {
    pub fn new(inner: T, fmt: fn(&T) -> String) -> Self {
        Self { inner, fmt }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Display for SelectOption<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = (self.fmt)(&self.inner);
        write!(f, "{s}")
    }
}
