extern crate fnv;

use self::fnv::FnvHashMap;
use ::Board::{Board, BoardResult};
use ::Piece::Piece;
use ::Grid::Grid;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BoardState {
    result: BoardResult,
    depth: Option<u8>,
}

pub struct BoardMap {
    pub map: FnvHashMap<Board, BoardState>,
    pub wins: u32,
    pub loses: u32,
}

impl BoardMap {
    pub fn from_pieces(pieces: Vec<Piece>) -> BoardMap {
        let board = Board::Empty();
        let mut board_map = BoardMap {map: FnvHashMap::default(), wins: 0, loses: 0};

        // まず先手後手の王将を置く
        for x in 0..3 {
            for y in 0..3 {
                let board = board.set_grid(x, y, Grid {piece: Piece::王将, player: 0, promoted: false});

                for x in 0..3 {
                    for y in 0..3 {
                        if board.get_grid(x, y).piece != Piece::Empty {
                            continue;
                        }
                        let board = board.set_grid(x, y, Grid {piece: Piece::王将, player: 1, promoted: false});

                        board_map.place_pieces(board, &pieces);
                    }
                }
            }
        }

        board_map
    }

    // 王将以外の駒を配置する
    fn place_pieces(&mut self, board: Board, pieces: &Vec<Piece>) {
        if !board.is_valid() {
            return;
        }

        if pieces.len() == 0 {
            if self.map.contains_key(&board) {
                return;
            }

            let result = board.get_result();

            if result == BoardResult::Unknown {
                self.map.insert(board, BoardState {result: result, depth: None});
            } else {
                if result == BoardResult::Win {
                    self.wins += 1;
                } else if result == BoardResult::Lose {
                    self.loses += 1;
                    println!("Found lose board:");
                    board.print();
                }

                self.map.insert(board, BoardState {result: result, depth: Some(0)});
            }
            return;
        }

        let (&piece, rest_pieces) = pieces.split_first().unwrap();

        // 駒を置く
        for x in 0..3 {
            for y in 0..3 {
                if board.get_grid(x, y).piece != Piece::Empty {
                    continue;
                }

                self.place_pieces(board.set_grid(x, y, Grid {piece: piece, player: 0, promoted: false}), &rest_pieces.to_vec());
                self.place_pieces(board.set_grid(x, y, Grid {piece: piece, player: 1, promoted: false}), &rest_pieces.to_vec());

                if piece.is_promotable() {
                    self.place_pieces(board.set_grid(x, y, Grid {piece: piece, player: 0, promoted: true}), &rest_pieces.to_vec());
                    self.place_pieces(board.set_grid(x, y, Grid {piece: piece, player: 1, promoted: true}), &rest_pieces.to_vec());
                }
            }
        }

        // 持ち駒にする
        self.place_pieces(board.add_hand(0, piece, 1), &rest_pieces.to_vec());
        self.place_pieces(board.add_hand(1, piece, 1), &rest_pieces.to_vec());
    }
}
