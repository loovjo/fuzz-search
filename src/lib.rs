#[macro_use]
extern crate lazy_static;
extern crate unicode_normalization;

use std::borrow::Borrow;
use unicode_normalization::UnicodeNormalization;

mod fuzz_search;
pub use fuzz_search::fuzzy_search_score;
use fuzz_search::fuzzy_search_score_no_norm;

pub fn best_matches<T>(pattern: &str, items: Vec<T>, n: usize) -> impl Iterator<Item = T>
where
    T: Borrow<str>,
{
    best_matches_scores(pattern, items, n).map(|(name, _)| name).take(n)
}

pub fn best_matches_scores<T>(
    pattern: &str,
    items: Vec<T>,
    n: usize,
) -> impl Iterator<Item = (T, isize)>
where
    T: Borrow<str>,
{
    let pattern = pattern.nfc().collect::<String>();

    let mut items_scores = items
        .into_iter()
        .map(|name| {
            let x = &name.borrow().nfc().collect::<String>();
            (name, fuzzy_search_score_no_norm(&pattern, x))
        })
        .collect::<Vec<_>>();

    items_scores.sort_by_key(|(_, x)| -x);

    items_scores.into_iter().take(n)
}

pub fn best_matches_scores_key<T, F, K>(
    pattern: &str,
    items: Vec<T>,
    f: F,
    n: usize,
) -> impl Iterator<Item = (T, isize)>
where
    F: Fn(&T) -> K,
    K: Borrow<str>,
{
    let pattern = pattern.nfc().collect::<String>();

    let mut items_scores = items
        .into_iter()
        .map(|name| {
            let x = &f(&name).borrow().nfc().collect::<String>();
            (name, fuzzy_search_score_no_norm(&pattern, x))
        })
        .collect::<Vec<_>>();

    items_scores.sort_by_key(|(_, x)| -x);

    items_scores.into_iter().take(n)
}
