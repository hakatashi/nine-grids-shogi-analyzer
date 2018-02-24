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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Piece(u8);

impl Board {
    fn get_grid(&self, x: u8, y: u8) -> Piece {
        Piece(((self.grids >> ((y * 3 + x) * 5)) & 0b11111) as u8)
    }

    fn print(&self) {
        println!("grids");

        for y in 0..3 {
            for x in 0..3 {
                println!("({}, {}): {:?}", x, y, self.get_grid(x, y));
            }
        }
    }
}

fn main() {
    let board = Board {
        grids: 0b0000000000000000000_00110_00101_00110_00101_00100_00011_00010_00001_00000,
        hands: 0b00000_000_000_0000_0000_0000_0000_00000,
        player: true,
    };

    board.print();
}
