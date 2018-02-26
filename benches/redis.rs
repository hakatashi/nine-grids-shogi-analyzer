#![feature(i128_type)]
#![feature(test)]

extern crate redis;
extern crate test;

use redis::Commands;
use test::Bencher;
use std::ops::Range;

#[derive(PartialEq, Eq, Hash)]
struct Key {
    n1: u64,
    n2: u32,
    n3: bool,
}

impl Key {
    fn as_bytes(&self) -> Vec<u8> {
        vec![
            ((self.n1 >> (8 * 7)) & 0xff) as u8,
            ((self.n1 >> (8 * 6)) & 0xff) as u8,
            ((self.n1 >> (8 * 5)) & 0xff) as u8,
            ((self.n1 >> (8 * 4)) & 0xff) as u8,
            ((self.n1 >> (8 * 3)) & 0xff) as u8,
            ((self.n1 >> (8 * 2)) & 0xff) as u8,
            ((self.n1 >> (8 * 1)) & 0xff) as u8,
            ((self.n1 >> (8 * 0)) & 0xff) as u8,
            ((self.n2 >> (8 * 3)) & 0xff) as u8,
            ((self.n2 >> (8 * 2)) & 0xff) as u8,
            ((self.n2 >> (8 * 1)) & 0xff) as u8,
            ((self.n2 >> (8 * 0)) & 0xff) as u8,
            if self.n3 {1_u8} else {0_u8},
        ]
    }
}

const RANGE: Range<u32> = 0..100000;

#[bench]
fn testredis(bench: &mut Bencher) {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    bench.iter(|| {
        for i in RANGE {
            let _: () = con.set(
                Key {
                    n1: (i % 1000) as u64,
                    n2: (i % 1000) as u32,
                    n3: if i % 2 == 0 { true } else { false },
                }.as_bytes(),
                100,
            ).unwrap();
        }
    });
}
