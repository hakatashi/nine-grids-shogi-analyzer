#![feature(non_ascii_idents)]

extern crate integer_sqrt;
#[macro_use] extern crate enum_primitive;

use std::vec::Vec;

mod util;

enum_from_primitive! {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Piece {
        Empty,
        王将,
        飛車,
        角行,
        金将,
        銀将,
        桂馬,
        香車,
        歩兵,
    }
}

impl Piece {
    fn to_char(&self) -> &str {
        match self {
            &Piece::Empty => "　",
            &Piece::王将 => "王",
            &Piece::飛車 => "飛",
            &Piece::角行 => "角",
            &Piece::金将 => "金",
            &Piece::銀将 => "銀",
            &Piece::桂馬 => "桂",
            &Piece::香車 => "香",
            &Piece::歩兵 => "歩",
        }
    }
}

// マス
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Grid {
    piece: Piece,
    player: u8,
    promoted: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Board {
    // マスの状態
    grids: u64,
    // 持ち駒
    hands: u32,
    // 手番 (先手/後手)
    player: bool,
}

#[derive(PartialEq, Eq, Debug)]
struct BoardHandInfo {
    first: Vec<Piece>,
    second: Vec<Piece>,
}

impl Board {
    fn get_grid(&self, x: u8, y: u8) -> Grid {
        match (self.grids >> ((y * 3 + x) * 5)) & 0b11111 {
            0 => Grid {piece: Piece::Empty, player: 0, promoted: false},
            1 => Grid {piece: Piece::王将, player: 0, promoted: false},
            2 => Grid {piece: Piece::飛車, player: 0, promoted: false},
            3 => Grid {piece: Piece::飛車, player: 0, promoted: true},
            4 => Grid {piece: Piece::角行, player: 0, promoted: false},
            5 => Grid {piece: Piece::角行, player: 0, promoted: true},
            6 => Grid {piece: Piece::金将, player: 0, promoted: false},
            7 => Grid {piece: Piece::銀将, player: 0, promoted: false},
            8 => Grid {piece: Piece::銀将, player: 0, promoted: true},
            9 => Grid {piece: Piece::桂馬, player: 0, promoted: false},
            10 => Grid {piece: Piece::桂馬, player: 0, promoted: true},
            11 => Grid {piece: Piece::香車, player: 0, promoted: false},
            12 => Grid {piece: Piece::香車, player: 0, promoted: true},
            13 => Grid {piece: Piece::歩兵, player: 0, promoted: false},
            14 => Grid {piece: Piece::歩兵, player: 0, promoted: true},
            15 => Grid {piece: Piece::王将, player: 1, promoted: false},
            16 => Grid {piece: Piece::飛車, player: 1, promoted: false},
            17 => Grid {piece: Piece::飛車, player: 1, promoted: true},
            18 => Grid {piece: Piece::角行, player: 1, promoted: false},
            19 => Grid {piece: Piece::角行, player: 1, promoted: true},
            20 => Grid {piece: Piece::金将, player: 1, promoted: false},
            21 => Grid {piece: Piece::銀将, player: 1, promoted: false},
            22 => Grid {piece: Piece::銀将, player: 1, promoted: true},
            23 => Grid {piece: Piece::桂馬, player: 1, promoted: false},
            24 => Grid {piece: Piece::桂馬, player: 1, promoted: true},
            25 => Grid {piece: Piece::香車, player: 1, promoted: false},
            26 => Grid {piece: Piece::香車, player: 1, promoted: true},
            27 => Grid {piece: Piece::歩兵, player: 1, promoted: false},
            28 => Grid {piece: Piece::歩兵, player: 1, promoted: true},
            _ => panic!(),
        }
    }

    fn get_hands(&self) -> BoardHandInfo {
        let mut hands = BoardHandInfo {
            first: Vec::with_capacity(27),
            second: Vec::with_capacity(27),
        };

        let mut temp_hands = self.hands;
        for hand_type in 0..7 {
            let size = match hand_type {
                0 | 1 => 3,
                2 | 3 | 4 | 5  => 4,
                6 => 5,
                _ => panic!(),
            };
            let max_pieces = match hand_type {
                0 | 1 => 2,
                2 | 3 | 4 | 5 => 3,
                6 => 7,
                _ => panic!(),
            };
            let piece = match hand_type {
                0 => Piece::飛車,
                1 => Piece::角行,
                2 => Piece::金将,
                3 => Piece::銀将,
                4 => Piece::桂馬,
                5 => Piece::香車,
                6 => Piece::歩兵,
                _ => panic!(),
            };
            let hand_info = util::hand_data_to_hand_info((temp_hands & ((1 << size) - 1)) as u8, max_pieces);
            temp_hands >>= size;

            for _ in 0..hand_info.first {
                hands.first.push(piece);
            }

            for _ in 0..hand_info.second {
                hands.second.push(piece);
            }
        }

        return hands;
    }

    fn print(&self) {
        for y in 0..3 {
            match y {
                0 => println!("┌───┬───┬───┐"),
                1 | 2 => println!("├───┼───┼───┤"),
                _ => panic!(),
            }

            for x in 0..3 {
                let grid = self.get_grid(x, y);
                print!("│{}{}{}", if grid.promoted {"!"} else {" "}, grid.piece.to_char(), if grid.player == 0 {"^"} else {"v"});
            }

            println!("│");
        }

        println!("└───┴───┴───┘");

        println!("{:?}", self.get_hands());
    }
}

fn main() {
    let board = Board {
        grids: 0b0000000000000000000_00111_00111_00110_00101_00100_00011_00010_00001_00000,
        hands: 0b000000000_00010_1000_0100_0001_0100_010_001,
        player: true,
    };

    board.print();
}
