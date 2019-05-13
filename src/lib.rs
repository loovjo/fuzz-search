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
    let pattern = pattern.nfc().collect::<String>();

    let mut items_scores = items
        .into_iter()
        .map(|name| {
            let x = &name.borrow().nfc().collect::<String>();
            (name, fuzzy_search_score_no_norm(&pattern, x))
        })
        .collect::<Vec<_>>();

    items_scores.sort_by_key(|(_, x)| -x);

    items_scores.into_iter().map(|(name, _)| name).take(n)
}
