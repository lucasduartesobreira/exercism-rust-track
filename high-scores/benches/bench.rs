#![feature(test)]

extern crate test;

use high_scores::HighScores;

#[bench]
fn bench_personal_top_tree(b: &mut test::Bencher) {
    let high_scores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);
    b.iter(|| assert_eq!(high_scores.personal_top_three(), vec![100, 90, 70]))
}
