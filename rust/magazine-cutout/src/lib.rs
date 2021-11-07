// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_words_count = magazine.iter().fold(HashMap::new(), |mut words, word| {
        *words.entry(word).or_insert(0) += 1;
        words
    });

    let note_words_count = note.iter().fold(HashMap::new(), |mut words, word| {
        *words.entry(word).or_insert(0) += 1;
        words
    });

    note_words_count
        .iter()
        .all(&|(word, count)| magazine_words_count.get(word).unwrap_or(&0) >= count)
}
