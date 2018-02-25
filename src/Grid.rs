use ::Piece::Piece;

// 移動量
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub x: i8,
    pub y: i8,
}

const M00: Move = Move {x: -2, y: -2};
const M01: Move = Move {x: -1, y: -2};
const M02: Move = Move {x:  0, y: -2};
const M03: Move = Move {x:  1, y: -2};
const M04: Move = Move {x:  2, y: -2};
const M10: Move = Move {x: -2, y: -1};
const M11: Move = Move {x: -1, y: -1};
const M12: Move = Move {x:  0, y: -1};
const M13: Move = Move {x:  1, y: -1};
const M14: Move = Move {x:  2, y: -1};
const M20: Move = Move {x: -2, y:  0};
const M21: Move = Move {x: -1, y:  0};
// M22 will never used :)
const M23: Move = Move {x:  1, y:  0};
const M24: Move = Move {x:  2, y:  0};
const M30: Move = Move {x: -2, y:  1};
const M31: Move = Move {x: -1, y:  1};
const M32: Move = Move {x:  0, y:  1};
const M33: Move = Move {x:  1, y:  1};
const M34: Move = Move {x:  2, y:  1};
const M40: Move = Move {x: -2, y:  2};
const M41: Move = Move {x: -1, y:  2};
const M42: Move = Move {x:  0, y:  2};
const M43: Move = Move {x:  1, y:  2};
const M44: Move = Move {x:  2, y:  2};

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

    pub fn is_promotable(&self) -> bool {
        self.piece != Piece::Empty && self.piece != Piece::王将 && self.piece != Piece::金将 && !self.promoted
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

    fn get_raw_moves(&self) -> Vec<Move> {
        /*
           ○○○
           ○金○
             ○
         */
        let 金将Moves = vec![
                 M11, M12, M13,
                 M21,      M23,
                      M32,
        ];

        match self.piece {
            Piece::Empty => vec![],
            Piece::王将 => {
                assert!(self.promoted == false);
                /*
                   ○○○
                   ○王○
                   ○○○
                 */
                vec![
                         M11, M12, M13,
                         M21,      M23,
                         M31, M32, M33,
                ]
            },
            Piece::飛車 => {
                if !self.promoted {
                    /*
                         ○
                         ○
                     ○○飛○○
                         ○
                         ○
                     */
                    vec![
                                  M02,
                                  M12,
                        M20, M21,      M23, M24,
                                  M32,
                                  M42,
                    ]
                } else {
                    /*
                         ○
                       ○○○
                     ○○龍○○
                       ○○○
                         ○
                     */
                    vec![
                                  M02,
                             M11, M12, M13,
                        M20, M21,      M23, M24,
                             M31, M32, M33,
                                  M42,
                    ]
                }
            },
            Piece::角行 => {
                if !self.promoted {
                    /*
                     ○      ○
                       ○  ○
                         角
                       ○  ○
                     ○      ○
                     */
                    vec![
                        M00,                M04,
                             M11,      M13,

                             M31,      M33,
                        M40,                M44,
                    ]
                } else {
                    /*
                     ○      ○
                       ○○○
                       ○馬○
                       ○○○
                     ○      ○
                     */
                    vec![
                        M00,                M04,
                             M11, M12, M13,
                             M21,      M23,
                             M31, M32, M33,
                        M40,                M44,
                    ]
                }
            },
            Piece::金将 => {
                assert!(self.promoted == false);
                金将Moves
            },
            Piece::銀将 => {
                if !self.promoted {
                    /*
                       ○○○
                         銀
                       ○  ○
                     */
                    vec![
                             M11, M12, M13,

                             M31,      M33,
                    ]
                } else {
                    金将Moves
                }
            },
            Piece::桂馬 => {
                if !self.promoted {
                    /*
                       ○  ○

                         桂
                     */
                    vec![
                             M01,      M03,
                    ]
                } else {
                    金将Moves
                }
            },
            Piece::香車 => {
                if !self.promoted {
                    /*
                         ○
                         ○
                         香
                     */
                    vec![
                                  M02,
                                  M12,
                    ]
                } else {
                    金将Moves
                }
            },
            Piece::歩兵 => {
                if !self.promoted {
                    /*
                         ○
                         歩
                     */
                    vec![
                                  M12,
                    ]
                } else {
                    金将Moves
                }
            },
        }
    }

    pub fn get_moves(&self) -> Vec<Move> {
        let raw_moves = self.get_raw_moves();
        if self.player == 0 {
            raw_moves
        } else {
            raw_moves.iter().map(|&m| Move {x: -m.x, y: -m.y}).collect::<Vec<_>>()
        }
    }
}
