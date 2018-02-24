#![feature(test)]

extern crate fnv;
extern crate test;

use fnv::FnvHashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Board {
    // マスの状態
    grids: u64,
    // 持ち駒
    hands: u32,
    // 手番 (先手/後手)
    player: bool,
}

fn main() {
    let board = Board {
        grids: 0b0000000000000000000_00000_00000_00000_00000_00000_00000_00000_00000_00000,
        hands: 0b00000_000_000_0000_0000_0000_0000_00000,
        player: true,
    };

    println!("{:?}", board);
}
