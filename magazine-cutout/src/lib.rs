use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    if note.len() > magazine.len() {
        return false;
    }

    let mut available_words = HashMap::new();

    magazine.iter().for_each(|word| {
        *available_words.entry(word).or_insert(0u32) += 1u32;
    });

    for word in note.iter() {
        match available_words.get_mut(word) {
            Some(count) if *count >= 1 => {
                *count -= 1;
            }
            _ => {
                return false;
            }
        }
    }

    true
}
