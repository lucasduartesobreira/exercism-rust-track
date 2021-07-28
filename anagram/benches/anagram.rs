#![feature(test)]
extern crate test;

use std::collections::HashSet;
use test::Bencher;

#[bench]
#[ignore]
fn test_multiple_anagrams(b: &mut Bencher) {
    let word = "allergy";

    let inputs = [
        "gallery",
        "ballerina",
        "regally",
        "clergy",
        "largely",
        "leading",
    ];

    let outputs = vec!["gallery", "regally", "largely"];

    b.iter(|| process_anagram_case(word, &inputs, &outputs));
}

fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
    let result = anagram::anagrams_for(word, inputs);

    let expected: HashSet<&str> = expected.iter().cloned().collect();

    assert_eq!(result, expected);
}
