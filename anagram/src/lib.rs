use std::collections::HashSet;

/// Check which of the possible anagrams are anagrams of the word
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_letter_counting = sort(&word_lower);

    possible_anagrams
        .iter()
        /*
         *.filter(|candidate| {
         *    let candidate_lower = candidate.to_lowercase();
         *    word_lower.len() == candidate_lower.len() && word_lower != candidate_lower
         *})
         */
        .filter(|candidate| {
            let candidate_lower = candidate.to_lowercase();
            let anagram_letter_counting = sort(&candidate_lower);

            word_lower.len() == candidate_lower.len()
                && word_lower != candidate_lower
                && anagram_letter_counting == word_letter_counting

            /*
             *if anagram_letter_counting == word_letter_counting {
             *    return true;
             *}
             *false
             */
        })
        .copied()
        .collect()
}

/// Count how many appearances each letter has in the word
fn sort(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}
