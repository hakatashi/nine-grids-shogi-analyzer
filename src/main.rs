#![feature(non_ascii_idents)]

extern crate integer_sqrt;

use std::vec::Vec;
use integer_sqrt::IntegerSquareRoot;

mod util;

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

    fn get_hands(&self, player: u8) -> Vec<Piece> {
        let mut hands: Vec<Piece> = Vec::with_capacity(27);

        let mut temp_hands = self.hands;
        for i in 0..7 {
            let size = match i {
                0 | 1 => 3,
                2 | 3 | 4 | 5  => 4,
                6 => 5,
                _ => panic!(),
            };
            let max_pieces = match i {
                0 | 1 => 2,
                2 | 3 | 4 | 5 => 3,
                6 => 7,
                _ => panic!(),
            };
            let hands = util::hand_data_to_hand_info((temp_hands & ((1 << size) - 1)) as u8, max_pieces);
            temp_hands >>= size;
        }

        return hands;
    }

    fn print(&self) {
        println!("== grids ==");

        for y in 0..3 {
            for x in 0..3 {
                println!("({}, {}): {:?}", x, y, self.get_grid(x, y));
            }
        }

        println!("{:?}", self.get_hands(0));
    }
}

fn main() {
    let board = Board {
        grids: 0b0000000000000000000_00110_00101_00110_00101_00100_00011_00010_00001_00000,
        hands: 0b000000000_00000_0000_0000_0000_0000_000_000,
        player: true,
    };

    board.print();
}
