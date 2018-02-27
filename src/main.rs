#![feature(non_ascii_idents, range_contains)]
#![allow(non_snake_case)]

extern crate integer_sqrt;
extern crate fnv;

mod util;
mod Board;
mod BoardMap;
mod Grid;
mod Piece;

use fnv::FnvHashMap;

fn main() {
    println!("Generate boards from pieces 銀銀歩歩:");

    let mut board_map = BoardMap::BoardMap::from_pieces(vec![
        Piece::Piece::銀将,
        Piece::Piece::銀将,
        Piece::Piece::歩兵,
        Piece::Piece::歩兵,
    ]);

    println!("Number of generated boards: {}", board_map.map.len());
    println!("Depth-0 Wins: {}", board_map.wins);
    println!("Depth-0 Loses: {}", board_map.loses);

    let mut depth = 1;

    loop {
        let mut current_map = BoardMap::BoardMap::Empty();

        for (&board, &state) in board_map.map.iter() {
            if state.result == Board::BoardResult::Unknown {
                let transitions = board.get_possible_transitions();

                let mut is_all_win = true;
                let mut is_any_lose = false;
                let mut min_lose_depth = None;
                let mut max_win_depth = None;

                for transition in transitions {
                    let transition_state = board_map.map.get(&transition).unwrap();

                    match transition_state.result {
                        Board::BoardResult::Win => {
                            match max_win_depth {
                                None => max_win_depth = Some(transition_state.depth.unwrap()),
                                Some(depth) => {
                                    if transition_state.depth.unwrap() > depth {
                                        max_win_depth = Some(transition_state.depth.unwrap());
                                    }
                                }
                            }
                        },
                        Board::BoardResult::Lose => {
                            is_all_win = false;
                            is_any_lose = true;
                            match min_lose_depth {
                                None => min_lose_depth = Some(transition_state.depth.unwrap()),
                                Some(depth) => {
                                    if transition_state.depth.unwrap() < depth {
                                        min_lose_depth = Some(transition_state.depth.unwrap());
                                    }
                                }
                            }
                        },
                        Board::BoardResult::Unknown => {
                            is_all_win = false;
                        },
                    }
                }

                if is_all_win {
                    current_map.map.insert(board, BoardMap::BoardState {
                        result: Board::BoardResult::Lose,
                        depth: Some(max_win_depth.unwrap() + 1),
                    });
                    current_map.loses += 1;
                } else if is_any_lose {
                    current_map.map.insert(board, BoardMap::BoardState {
                        result: Board::BoardResult::Win,
                        depth: Some(min_lose_depth.unwrap() + 1),
                    });
                    current_map.wins += 1;
                }
            }
        }

        println!("Depth-{} Wins: {}", depth, current_map.wins);
        println!("Depth-{} Loses: {}", depth, current_map.loses);

        if current_map.wins == 0 && current_map.loses == 0 {
            break;
        }

        board_map.merge(current_map);

        depth += 1;
    }

    println!("Total Wins: {}", board_map.wins);
    println!("Total Loses: {}", board_map.loses);

    let mut win_map: FnvHashMap<u8, u32> = FnvHashMap::default();
    let mut lose_map: FnvHashMap<u8, u32> = FnvHashMap::default();

    for (&board, &state) in board_map.map.iter() {
        match state.depth {
            None => {},
            Some(depth) => {
                match state.result {
                    Board::BoardResult::Win => {
                        let wins = match win_map.get(&depth) {
                            Some(&wins) => wins,
                            None => 0,
                        };

                        if wins == 0 {
                            println!("First Move-{} Win Board:", depth);
                            board.print();
                        }

                        win_map.insert(depth, wins + 1);
                    },
                    Board::BoardResult::Lose => {
                        let loses = match lose_map.get(&depth) {
                            Some(&count) => count,
                            None => 0,
                        };

                        if loses == 0 {
                            println!("First Move-{} Lose Board:", depth);
                            board.print();
                        }

                        lose_map.insert(depth, loses + 1);
                    },
                    _ => {},
                }
            },
        }
    }

    for i in 0..50 {
        let wins = match win_map.get(&i) {
            Some(&count) => count,
            None => 0,
        };
        let loses = match lose_map.get(&i) {
            Some(&count) => count,
            None => 0,
        };

        println!("Number of Move-{} Win Boards: {}", i, wins);
        println!("Number of Move-{} Lose Boards: {}", i, loses);
    }

    let board = Board::Board::Empty();
    let board = board.set_grid(0, 0, Grid::Grid {piece: Piece::Piece::歩兵, player: 1, promoted: false});
    let board = board.set_grid(1, 0, Grid::Grid {piece: Piece::Piece::銀将, player: 1, promoted: false});
    let board = board.set_grid(2, 0, Grid::Grid {piece: Piece::Piece::王将, player: 1, promoted: false});
    let board = board.set_grid(0, 2, Grid::Grid {piece: Piece::Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(1, 2, Grid::Grid {piece: Piece::Piece::銀将, player: 0, promoted: false});
    let board = board.set_grid(2, 2, Grid::Grid {piece: Piece::Piece::歩兵, player: 0, promoted: false});

    println!("State of This Borad:");
    board.print();
    println!("{:?}", board_map.map.get(&board));

    let board = Board::Board::Empty();
    let board = board.set_grid(0, 2, Grid::Grid {piece: Piece::Piece::王将, player: 1, promoted: false});
    let board = board.set_grid(2, 0, Grid::Grid {piece: Piece::Piece::王将, player: 0, promoted: false});
    let board = board.add_hand(0, Piece::Piece::歩兵, 1);
    let board = board.add_hand(0, Piece::Piece::銀将, 1);
    let board = board.add_hand(1, Piece::Piece::歩兵, 1);
    let board = board.add_hand(1, Piece::Piece::銀将, 1);

    println!("State of This Borad:");
    board.print();
    println!("{:?}", board_map.map.get(&board));
}
