extern crate fnv;
extern crate rusqlite;

use self::fnv::FnvHashMap;
use self::rusqlite::Connection;
use ::Board::{Board, BoardResult};
use ::Piece::Piece;
use ::Grid::Grid;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BoardState {
    pub result: BoardResult,
    pub depth: Option<u8>,
    pub routes: Option<u32>,
}

pub struct BoardMap {
    pub map: FnvHashMap<Board, BoardState>,
    pub wins: u32,
    pub loses: u32,
}

impl BoardMap {
    pub fn Empty() -> BoardMap {
        BoardMap {
            map: FnvHashMap::default(),
            wins: 0,
            loses: 0,
        }
    }

    pub fn from_pieces(pieces: Vec<Piece>) -> BoardMap {
        let board = Board::Empty();
        let mut board_map = BoardMap::Empty();

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

            // TODO: リファクタリング
            if result == BoardResult::Unknown {
                self.map.insert(board, BoardState {
                    result: result,
                    depth: None,
                    routes: None,
                });
            } else {
                if result == BoardResult::Win {
                    self.wins += 1;
                } else if result == BoardResult::Lose {
                    self.loses += 1;
                }

                self.map.insert(board, BoardState {
                    result: result,
                    depth: Some(if result == BoardResult::Win {0} else {1}),
                    routes: Some(1),
                });
            }

            if self.map.len() % 100000 == 0 {
                println!("BoardMap#place_pieces: {} boards completed", self.map.len());
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

    pub fn merge(&mut self, board_map: BoardMap) {
        self.wins += board_map.wins;
        self.loses += board_map.loses;

        for (&map, &state) in board_map.map.iter() {
            self.map.insert(map, state);
        }
    }

    pub fn write(&self, path: String) {
        let conn = Connection::open(path).unwrap();
        conn.execute("
            CREATE TABLE IF NOT EXISTS boards (
                board BLOB PRIMARY KEY NOT NULL,
                result INTEGER NOT NULL,
                depth INTEGER NOT NULL,
                routes INTEGER NOT NULL
            )
        ", &[]).unwrap();

        conn.query_row("PRAGMA journal_mode = OFF", &[], |_| {}).unwrap();
        conn.execute("PRAGMA synchronous = OFF", &[]).unwrap();

        let mut count = 0;
        let mut percentage = 1;
        let total_count = self.map.len();

        for (&board, &state) in self.map.iter() {
            count += 1;

            if percentage * total_count / 100 == count {
                println!("BoardMap::write: {}% completed ({}/{})", percentage, count, total_count);
                percentage += 1;
            }

            if state.result == BoardResult::Unknown || state.depth == Some(0) {
                continue;
            }

            conn.execute("
                INSERT OR REPLACE INTO boards (board, result, depth, routes) VALUES (?1, ?2, ?3, ?4)
            ", &[
                &board.to_blob(),
                &(match state.result {
                    BoardResult::Lose => 0,
                    BoardResult::Win => 1,
                    _ => panic!(),
                }),
                &state.depth.unwrap(),
                &state.routes.unwrap(),
            ]).unwrap();
        }
    }
}
