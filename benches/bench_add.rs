// benches/bench_add.rs
#![feature(test)]
extern crate test;
use test::Bencher;
use effective_rust::item30::add;

#[bench]
fn bench_add(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(1);
        let b = test::black_box(2);
        add(a, b)
    });
}
