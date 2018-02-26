#![feature(i128_type)]
#![feature(test)]

extern crate fnv;
extern crate test;

use fnv::FnvHashMap;
use test::Bencher;
use std::ops::Range;

#[derive(PartialEq, Eq, Hash)]
struct Key32x3 {
    n1: u32,
    n2: u32,
    n3: u32,
}

#[derive(PartialEq, Eq, Hash)]
struct Key64And32 {
    n1: u64,
    n2: u32,
}

#[derive(PartialEq, Eq, Hash)]
struct Key64And32AndBool {
    n1: u64,
    n2: u32,
    n3: bool,
}

#[derive(PartialEq, Eq, Hash)]
struct Key128x1 {
    n1: u128,
}

const RANGE: Range<u32> = 0..100000;

#[bench]
fn test32x3(bench: &mut Bencher) {
    let mut map: FnvHashMap<Key32x3, i16> = FnvHashMap::default();

    bench.iter(|| {
        for i in RANGE {
            map.insert(
                Key32x3 {
                    n1: (i % 1000) as u32,
                    n2: (i % 1000) as u32,
                    n3: (i % 1000) as u32,
                },
                100,
            );
        }
    });
}

#[bench]
fn test64_32_bool(bench: &mut Bencher) {
    let mut map: FnvHashMap<Key64And32AndBool, i16> = FnvHashMap::default();

    bench.iter(|| {
        for i in RANGE {
            map.insert(
                Key64And32AndBool {
                    n1: (i % 1000) as u64,
                    n2: (i % 1000) as u32,
                    n3: if i % 2 == 0 { true } else { false },
                },
                100,
            );
        }
    });
}

#[bench]
fn test64_32(bench: &mut Bencher) {
    let mut map: FnvHashMap<Key64And32, i16> = FnvHashMap::default();

    bench.iter(|| {
        for i in RANGE {
            map.insert(
                Key64And32 {
                    n1: (i % 1000) as u64,
                    n2: (i % 1000) as u32,
                },
                100,
            );
        }
    });
}

#[bench]
fn test128x1(bench: &mut Bencher) {
    let mut map: FnvHashMap<Key128x1, i16> = FnvHashMap::default();

    bench.iter(|| {
        for i in RANGE {
            map.insert(
                Key128x1 {
                    n1: (i % 1000) as u128,
                },
                100,
            );
        }
    });
}
