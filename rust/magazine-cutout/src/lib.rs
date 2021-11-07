// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_word_count: HashMap<&str, i32> = HashMap::new();
    let mut note_word_count: HashMap<&str, i32> = HashMap::new();

    for word in magazine {
        *magazine_word_count.entry(word).or_insert(0) += 1;
    }

    for word in note {
        *note_word_count.entry(word).or_insert(0) += 1;
    }

    note_word_count
        .iter()
        .all(|(word, count)| *magazine_word_count.entry(word).or_insert(0) >= *count)
}
