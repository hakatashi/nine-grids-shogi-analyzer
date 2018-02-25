use std::vec::Vec;
use ::Grid::Grid;
use ::Piece::Piece;
use ::util;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Board {
    // マスの状態
    pub grids: u64,
    // 持ち駒
    pub hands: u32,
    // 手番 (先手/後手)
    pub player: bool,
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

    pub fn set_grid(&self, x: u8, y: u8, grid: Grid) -> Board {
        return Board {
            grids: (self.grids & !(0b11111 << ((y * 3 + x) * 5))) | ((grid.to_i() as u64 ) << ((y * 3 + x) * 5)),
            hands: self.hands,
            player: self.player,
        };
    }

    pub fn del_grid(&self, x: u8, y: u8) -> Board {
        return self.set_grid(x, y, Grid {piece: Piece::Empty, player: 0, promoted: false});
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

    pub fn print(&self) {
        for y in 0..3 {
            println!("────────────────");

            for x in 0..3 {
                let grid = self.get_grid(x, y);
                print!("│{}{}{}", if grid.promoted {"!"} else {" "}, grid.piece.to_char(), if grid.player == 0 {"^"} else {"v"});
            }

            println!("│");
        }

        println!("────────────────");

        println!("{:?}", self.get_hands());
    }
}
