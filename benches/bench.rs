#![feature(test)]
extern crate test;

use html_editor::parse;
use test::Bencher;

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| {
        parse(include_str!("./bench.html")).unwrap();
    })
}
