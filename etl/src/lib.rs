use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .map(|(&point, letters)| {
            letters
                .iter()
                .map(move |&letter| (letter.to_ascii_lowercase(), point))
        })
        .flatten()
        .collect()
}
