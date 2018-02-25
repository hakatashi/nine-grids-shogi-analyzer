use ::Piece::Piece;

// マス
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Grid {
    pub piece: Piece,
    pub player: u8,
    pub promoted: bool,
}

impl Grid {
    pub fn from_i(number: u8) -> Grid {
        match number {
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

    pub fn to_i(&self) -> u8 {
        if self.piece == Piece::Empty {
            return 0;
        }

        let piece: u8 = match self.piece {
            Piece::Empty => 0,
            Piece::王将 => 1,
            Piece::飛車 => 2,
            Piece::角行 => 4,
            Piece::金将 => 6,
            Piece::銀将 => 7,
            Piece::桂馬 => 9,
            Piece::香車 => 11,
            Piece::歩兵 => 13,
        };

        piece + self.promoted as u8 + 14 * self.player
    }
}
