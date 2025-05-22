// benches/factorial.rs
#![feature(test)]
extern crate test;
use std::hint::black_box;

use effective_rust::item30::factorial;

#[bench]
fn bench_factorial(b: &mut test::Bencher) {
    b.iter(|| {
        let result = factorial(black_box(15));
        assert_eq!(result, 1_307_674_368_000);
    });
}
