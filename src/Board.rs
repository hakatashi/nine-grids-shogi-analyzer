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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoardResult {
    Win,
    Lose,
    Unknown,
}

#[derive(PartialEq, Eq, Debug)]
struct BoardHandInfo {
    first: Vec<u8>,
    second: Vec<u8>,
}

// 座標
#[derive(PartialEq, Eq, Debug)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

#[derive(PartialEq, Eq, Debug)]
pub struct PieceMove {
    pub piece: Piece,
    pub from: Coord,
    pub to: Coord,
    // 移動後の駒が成っているかどうか (もともと成っていた場合も含む)
    pub promote: bool,
}

impl Board {
    pub fn Empty() -> Board {
        Board {
            grids: 0,
            hands: 0,
            player: false,
        }
    }

    pub fn to_blob(&self) -> Vec<u8> {
        vec![
            ((self.grids >> (8 * 7)) & 0xff) as u8,
            ((self.grids >> (8 * 6)) & 0xff) as u8,
            ((self.grids >> (8 * 5)) & 0xff) as u8,
            ((self.grids >> (8 * 4)) & 0xff) as u8,
            ((self.grids >> (8 * 3)) & 0xff) as u8,
            ((self.grids >> (8 * 2)) & 0xff) as u8,
            ((self.grids >> (8 * 1)) & 0xff) as u8,
            ((self.grids >> (8 * 0)) & 0xff) as u8,
            ((self.hands >> (8 * 3)) & 0xff) as u8,
            ((self.hands >> (8 * 2)) & 0xff) as u8,
            ((self.hands >> (8 * 1)) & 0xff) as u8,
            ((self.hands >> (8 * 0)) & 0xff) as u8,
        ]
    }

    pub fn get_grid(&self, x: u8, y: u8) -> Grid {
        assert!(x < 3 && y < 3);
        Grid::from_i(((self.grids >> ((y * 3 + x) * 5)) & 0b11111) as u8)
    }

    pub fn set_grid(&self, x: u8, y: u8, grid: Grid) -> Board {
        assert!(x < 3 && y < 3);
        Board {
            grids: (self.grids & !(0b11111 << ((y * 3 + x) * 5))) | ((grid.to_i() as u64 ) << ((y * 3 + x) * 5)),
            hands: self.hands,
            player: self.player,
        }
    }

    pub fn del_grid(&self, x: u8, y: u8) -> Board {
        self.set_grid(x, y, Grid {piece: Piece::Empty, player: 0, promoted: false})
    }

    fn reverse_grids(grids: u64) -> u64 {
        let mut new_grids: u64 = 0;

        for y in 0..3 {
            let new_y = 2 - y;
            for x in 0..3 {
                let new_x = 2 - x;
                let mut grid = Grid::from_i(((grids >> ((y * 3 + x) * 5)) & 0b11111) as u8);
                grid.player = if grid.player == 0 {1} else {0};
                new_grids |= (grid.to_i() as u64) << ((new_y * 3 + new_x) * 5);
            }
        }

        new_grids
    }

    fn get_hands(&self) -> BoardHandInfo {
        let mut hands = BoardHandInfo {
            first: Vec::with_capacity(7),
            second: Vec::with_capacity(7),
        };

        let mut temp_hands = self.hands;
        for hand_type in 0..7 {
            let size = match hand_type {
                0 | 1 => 3,
                2 | 3 | 4 | 5  => 4,
                6 => 6,
                _ => panic!(),
            };
            let max_pieces = match hand_type {
                0 | 1 => 2,
                2 | 3 | 4 | 5 => 4,
                6 => 7,
                _ => panic!(),
            };
            let hand_info = util::hand_data_to_hand_info((temp_hands & ((1 << size) - 1)) as u8, max_pieces);
            temp_hands >>= size;

            hands.first.push(hand_info.first);
            hands.second.push(hand_info.second);
        }

        hands
    }

    fn set_hands(&self, hand_info: BoardHandInfo) -> Board {
        let mut hands: u32 = 0;
        let mut offset = 0;

        for hand_type in 0..7 {
            let size = match hand_type {
                0 | 1 => 3,
                2 | 3 | 4 | 5  => 4,
                6 => 6,
                _ => panic!(),
            };
            let max_pieces = match hand_type {
                0 | 1 => 2,
                2 | 3 | 4 | 5 => 4,
                6 => 7,
                _ => panic!(),
            };
            let hand_data = util::hand_info_to_hand_data(util::HandInfo {
                first: hand_info.first[hand_type],
                second: hand_info.second[hand_type],
            }, max_pieces);

            assert!(hand_data < (1 << size));

            hands |= (hand_data as u32) << offset;

            offset += size;
        }

        Board {
            grids: self.grids,
            hands: hands,
            player: self.player,
        }
    }

    pub fn add_hand(&self, player: u8, piece: Piece, count: i8) -> Board {
        let mut hands = self.get_hands();

        let piece_index = match piece {
            Piece::飛車 => 0,
            Piece::角行 => 1,
            Piece::金将 => 2,
            Piece::銀将 => 3,
            Piece::桂馬 => 4,
            Piece::香車 => 5,
            Piece::歩兵 => 6,
            _ => panic!(),
        };

        match player {
            0 => {
                assert!(hands.first[piece_index] as i8 + count >= 0);
                hands.first[piece_index] = (hands.first[piece_index] as i8 + count) as u8
            },
            1 => {
                assert!(hands.second[piece_index] as i8 + count >= 0);
                hands.second[piece_index] = (hands.second[piece_index] as i8 + count) as u8
            },
            _ => panic!(),
        };

        self.set_hands(hands)
    }

    fn reverse_hands(&self) -> Board {
        let hands = self.get_hands();
        let new_hands = BoardHandInfo {
            first: hands.second,
            second: hands.first,
        };
        self.set_hands(new_hands)
    }

    pub fn reverse(&self) -> Board {
        Board {
            grids: Board::reverse_grids(self.grids),
            hands: self.reverse_hands().hands,
            player: self.player, // player is unused currently
        }
    }

    pub fn get_possible_moves(&self) -> Vec<PieceMove> {
        let mut moves: Vec<PieceMove> = Vec::with_capacity(128);

        for y in 0..3 {
            for x in 0..3 {
                let grid = self.get_grid(x, y);

                if grid.piece == Piece::Empty || grid.player == 1 {
                    continue;
                }

                let piece_moves = grid.get_moves();

                for piece_move in piece_moves {
                    let target_x = x as i8 + piece_move.x;
                    let target_y = y as i8 + piece_move.y;

                    // 移動先が盤外
                    if !(0..3).contains(target_x) || !(0..3).contains(target_y) {
                        continue;
                    }

                    let target_grid = self.get_grid(target_x as u8, target_y as u8);

                    // 移動先に自分の駒がある
                    if target_grid.piece != Piece::Empty && target_grid.player == 0 {
                        continue;
                    }

                    // 大駒は他の駒を飛び越えられない
                    if piece_move.x % 2 == 0 && piece_move.y % 2 == 0 {
                        // 中間点
                        let intermediate_x = x as i8 + piece_move.x / 2;
                        let intermediate_y = y as i8 + piece_move.y / 2;
                        let intermediate_grid = self.get_grid(intermediate_x as u8, intermediate_y as u8);

                        if intermediate_grid.piece != Piece::Empty {
                            continue;
                        }
                    }

                    // 行き所のない駒
                    let force_promotion = if grid.promoted {
                        false
                    } else if target_y == 0 && (grid.piece == Piece::歩兵 || grid.piece == Piece::香車 || grid.piece == Piece::桂馬) {
                        true
                    } else if target_y == 1 && grid.piece == Piece::桂馬 {
                        true
                    } else {
                        false
                    };

                    moves.push(PieceMove {
                        from: Coord {
                            x: x,
                            y: y,
                        },
                        to: Coord {
                            x: target_x as u8,
                            y: target_y as u8,
                        },
                        piece: grid.piece,
                        promote: force_promotion || grid.promoted,
                    });

                    if !force_promotion && (y == 0 || target_y == 0) && grid.is_promotable() {
                        moves.push(PieceMove {
                            from: Coord {
                                x: x,
                                y: y,
                            },
                            to: Coord {
                                x: target_x as u8,
                                y: target_y as u8,
                            },
                            piece: grid.piece,
                            promote: true,
                        });
                    }
                }
            }
        }
        moves
    }

    pub fn get_result(&self) -> BoardResult {
        let moves = self.get_possible_moves();
        let hands = self.get_hands();

        // ステルスメイト
        if moves.len() == 0 {
            let mut is_stealth_mate = true;

            // 打ち駒を打てる場所があるか
            'hand_loop: for (i, &count) in hands.first.iter().enumerate() {
                if count > 0 {
                    let piece = Piece::from_hand_index(i);
                    for y in 0..3 {
                        for x in 0..3 {
                            let grid = self.get_grid(x, y);
                            if grid.piece != Piece::Empty {
                                continue;
                            }

                            let board = self.set_grid(x, y, Grid {piece: piece, player: 0, promoted: false}).add_hand(0, piece, -1);

                            if board.is_valid() {
                                is_stealth_mate = false;
                                break 'hand_loop;
                            }
                        }
                    }
                }
            }

            if is_stealth_mate {
                return BoardResult::Lose;
            }
        }

        for mov in moves {
            let target_grid = self.get_grid(mov.to.x, mov.to.y);

            if target_grid.piece == Piece::王将 && target_grid.player == 1 {
                return BoardResult::Win;
            }
        }

        BoardResult::Unknown
    }

    pub fn get_possible_transitions(&self) -> Vec<Board> {
        let mut boards: Vec<Board> = Vec::with_capacity(256);
        let moves = self.get_possible_moves();

        for mov in moves {
            let target_grid = self.get_grid(mov.to.x, mov.to.y);

            if target_grid.piece == Piece::王将 {
                continue;
            }

            let new_board = self.del_grid(mov.from.x, mov.from.y).set_grid(mov.to.x, mov.to.y, Grid {piece: mov.piece, promoted: mov.promote, player: 0});

            if target_grid.player == 1 {
                boards.push(new_board.add_hand(0, target_grid.piece, 1).reverse());
            } else {
                boards.push(new_board.reverse());
            }
        }

        let hands = self.get_hands();

        // 打ち駒
        for (i, &count) in hands.first.iter().enumerate() {
            if count > 0 {
                let piece = Piece::from_hand_index(i);
                for y in 0..3 {
                    for x in 0..3 {
                        let grid = self.get_grid(x, y);
                        if grid.piece != Piece::Empty {
                            continue;
                        }

                        let board = self.set_grid(x, y, Grid {piece: piece, player: 0, promoted: false}).add_hand(0, piece, -1);

                        if board.is_valid() {
                            boards.push(board.reverse());
                        }
                    }
                }
            }
        }

        boards
    }

    pub fn is_valid(&self) -> bool {
        for y in 0..3 {
            for x in 0..3 {
                let grid = self.get_grid(x, y);
                if grid.piece == Piece::Empty {
                    continue;
                }

                // 行き所のない駒
                if !grid.promoted && (
                    (grid.player == 0 && y == 0) ||
                    (grid.player == 1 && y == 2)
                ) && (
                    grid.piece == Piece::歩兵 ||
                    grid.piece == Piece::香車 ||
                    grid.piece == Piece::桂馬
                ) {
                    return false;
                }

                if !grid.promoted && y == 1 && grid.piece == Piece::桂馬 {
                    return false;
                }

                // 二歩
                if grid.piece == Piece::歩兵 && grid.promoted == false {
                    for target_y in (y + 1)..3 {
                        let target_grid = self.get_grid(x, target_y);
                        if target_grid.piece == Piece::歩兵 && target_grid.promoted == false && target_grid.player == grid.player {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    // 盤面が理想盤面(初期盤面としてふさわしい盤面)かを判定するメソッド
    // 具体的には、以下の2つのいずれかを満たす盤面かを判定する。
    // * 以下の2つを満たす
    //   * 盤面に成駒が一つもない (a)
    //   * 以下の条件のうち少なくとも1つを満たす
    //     * 盤面に置かれている王以外の駒が1つ以下 (b)
    //     * 持ち駒がなく(m)、それぞれのプレイヤーの駒がすべて自陣に配置されている (c)
    //     * 持ち駒がなく(m)、それぞれのプレイヤーの駒がすべて敵陣に配置されている (d)
    // * 以下の2つを満たす
    //   * 盤面に王将以外の先手の駒がない (e)
    //   * 先手の持ち駒がない、もしくは歩1つのみ (f)
    pub fn is_good(&self) -> bool {
        let mut a_flag = true;
        let mut b_count = 0_u8;
        let mut c_flag = true;
        let mut d_flag = true;
        let mut e_flag = true;
        let mut f_flag = true;
        let mut m_flag = true;

        for y in 0..3 {
            for x in 0..3 {
                let grid = self.get_grid(x, y);

                if grid.promoted {
                    a_flag = false;
                }

                if grid.piece != Piece::Empty && grid.piece != Piece::王将 {
                    b_count += 1;

                    if grid.player == 0 {
                        e_flag = false;
                    }
                }

                if y != 0 && grid.player == 1 {
                    c_flag = false;
                }

                if y != 2 && grid.player == 0 {
                    c_flag = false;
                }

                if y != 0 && grid.player == 0 {
                    d_flag = false;
                }

                if y != 2 && grid.player == 1 {
                    d_flag = false;
                }
            }
        }

        let hands = self.get_hands();

        for (i, &count) in hands.first.iter().enumerate() {
            let piece = Piece::from_hand_index(i);

            if count > 0 {
                m_flag = false;
            }

            if count > (if piece == Piece::歩兵 {1} else {0}) {
                f_flag = false;
            }
        }

        (a_flag && (b_count <= 1 || (m_flag && c_flag) || (m_flag && d_flag))) || (e_flag && f_flag)
    }

    pub fn is_transition_打ち歩(&self, transition: Board) -> bool {
        let from_hands = self.get_hands();
        let to_hands = transition.get_hands();
        from_hands.first[6] - to_hands.second[6] == 1
    }

    pub fn print(&self) {
        for y in 0..3 {
            for x in 0..3 {
                let grid = self.get_grid(x, y);
                print!("{}", grid.to_string());
            }

            println!("");
        }

        let hands = self.get_hands();

        print!("☗");
        let mut total_count = 0;

        for (i, &count) in hands.first.iter().enumerate() {
            let piece = Piece::from_hand_index(i);

            for _ in 0..count {
                print!("{}", piece.to_char());
            }

            total_count += count;
        }

        if total_count == 0 {
            print!("なし");
        }

        println!("");

        print!("☖");
        let mut total_count = 0;

        for (i, &count) in hands.second.iter().enumerate() {
            let piece = Piece::from_hand_index(i);

            for _ in 0..count {
                print!("{}", piece.to_char());
            }

            total_count += count;
        }

        if total_count == 0 {
            print!("なし");
        }

        println!("");
        println!("");
    }
}
