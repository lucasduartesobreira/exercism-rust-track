#![feature(test)]
extern crate test;

use prime_factors::factors;

#[bench]
fn dunno(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(factors(93_819_012_551), vec![11, 9539, 894_119]));
}
