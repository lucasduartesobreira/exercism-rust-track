use std::collections::{HashMap, HashSet};

/// Check which of the possible anagrams are anagrams of the word
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_uppercase = word.to_uppercase();

    let word_letter_counting = count_word_letters(word_uppercase.as_str());
    let mut anagrams = HashSet::new();

    possible_anagrams
        .iter()
        .filter(|anagram| word.len() == anagram.len() && word_uppercase != anagram.to_uppercase())
        .for_each(|anagram| {
            let anagram_letter_counting = count_word_letters(&anagram.to_uppercase());

            if anagram_letter_counting == word_letter_counting {
                anagrams.insert(*anagram);
            }
        });
    anagrams
}

/// Count how many appearances each letter has in the word
fn count_word_letters(word: &str) -> HashMap<char, usize> {
    let mut letter_counting = HashMap::new();

    word.chars().for_each(|c| {
        *letter_counting.entry(c).or_insert(0) += 1;
    });

    letter_counting
}
