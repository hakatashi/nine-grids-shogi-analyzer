#![allow(non_snake_case)]

extern crate integer_sqrt;
extern crate fnv;

mod util;
mod Board;
mod BoardMap;
mod Grid;
mod Piece;

use fnv::FnvHashMap;
use std::env;
use std::fs;

fn main() {
    fs::create_dir_all("boards").unwrap();

    let piece_config = match env::args().nth(1) {
        Some(config) => config,
        None => {
            panic!("Please specify config as args");
        },
    };

    let pieces: Vec<Piece::Piece> = piece_config.chars().filter_map(|letter| {
        match letter.to_digit(10) {
            Some(digit) => {
                Some(Piece::Piece::from_hand_index(digit as usize))
            },
            None => None,
        }
    }).collect();

    let cloned_pieces = pieces.clone();
    let piece_indices = cloned_pieces.iter().map(|piece| piece.to_hand_index().to_string());
    let filename = format!("boards/{}.sqlite3", piece_indices.collect::<Vec<_>>().concat());

    println!("Generate boards from pieces {:?}:", pieces);

    let mut board_map = BoardMap::BoardMap::from_pieces(pieces);

    println!("Number of generated boards: {}", board_map.map.len());
    println!("Depth-0 Wins: {}", board_map.wins);
    println!("Depth-0 Loses: {}", board_map.loses);

    let mut depth = 1;
    let mut 打ち歩詰め_count = 0;

    loop {
        let mut current_map = BoardMap::BoardMap::Empty();

        println!("Digging Depth-{}...", depth);

        for (&board, &state) in board_map.map.iter() {
            if state.result == Board::BoardResult::Unknown {
                let transitions = board.get_possible_transitions();
                let transition_count = transitions.len();

                let mut is_all_win = true;
                let mut is_any_lose = false;
                let mut min_lose_depth = None;
                let mut max_win_depth = None;
                let mut win_routes = 0_u32;
                let mut lose_routes = 0_u32;
                // 非合法手の数
                let mut win_0_count = 0_u16;

                for transition in transitions {
                    let transition_state = match board_map.map.get(&transition) {
                        None => {
                            println!("The following board was not found in map:");
                            transition.print();
                            println!("Transitioned from the following board:");
                            board.print();
                            panic!();
                        },
                        Some(state) => state,
                    };

                    match transition_state.result {
                        Board::BoardResult::Win => {
                            let new_depth = match transition_state.depth {
                                None => {
                                    println!("Depth of the following board was not set:");
                                    transition.print();
                                    panic!();
                                },
                                Some(depth) => depth,
                            };

                            let new_routes = match transition_state.routes {
                                None => {
                                    println!("Routes of the following board was not set:");
                                    transition.print();
                                    panic!();
                                },
                                Some(routes) => routes,
                            };

                            match max_win_depth {
                                None => {
                                    max_win_depth = Some(new_depth);
                                    win_routes = new_routes;
                                },
                                Some(depth) => {
                                    if new_depth == depth {
                                        win_routes += new_routes;
                                    } else if new_depth > depth {
                                        max_win_depth = Some(new_depth);
                                        win_routes = new_routes;
                                    }
                                },
                            }

                            if new_depth == 0 {
                                win_0_count += 1;
                            }
                        },
                        Board::BoardResult::Lose => {
                            let new_depth = match transition_state.depth {
                                None => {
                                    println!("Depth of the following board was not set:");
                                    transition.print();
                                    panic!();
                                },
                                Some(depth) => depth,
                            };

                            let new_routes = match transition_state.routes {
                                None => {
                                    println!("Routes of the following board was not set:");
                                    transition.print();
                                    panic!();
                                },
                                Some(routes) => routes,
                            };

                            // 打ち歩詰め
                            if new_depth == 1 && board.is_transition_打ち歩(transition) {
                                打ち歩詰め_count += 1;
                                win_0_count += 1;
                                continue;
                            }

                            is_all_win = false;
                            is_any_lose = true;

                            match min_lose_depth {
                                None => {
                                    min_lose_depth = Some(new_depth);
                                    lose_routes = new_routes;
                                },
                                Some(depth) => {
                                    if new_depth == depth {
                                        lose_routes += new_routes;
                                    } else if new_depth < depth {
                                        min_lose_depth = Some(new_depth);
                                        lose_routes = new_routes;
                                    }
                                },
                            }
                        },
                        Board::BoardResult::Unknown => {
                            is_all_win = false;
                        },
                    }
                }

                if is_all_win {
                    let max_win_depth = match max_win_depth {
                        None => {
                            println!("Transition of the following board was not found:");
                            board.print();
                            panic!();
                        },
                        Some(depth) => depth,
                    };

                    current_map.map.insert(board, BoardMap::BoardState {
                        result: Board::BoardResult::Lose,
                        depth: Some(max_win_depth + 1),
                        routes: Some(win_routes),
                        is_good: Some(false),
                    });
                    current_map.loses += 1;
                } else if is_any_lose {
                    let min_lose_depth = match min_lose_depth {
                        None => {
                            println!("Transition of the following board was not found:");
                            board.print();
                            panic!();
                        },
                        Some(depth) => depth,
                    };

                    current_map.map.insert(board, BoardMap::BoardState {
                        result: Board::BoardResult::Win,
                        depth: Some(min_lose_depth + 1),
                        routes: Some(lose_routes),
                        // 合法手の数が3つ以上かつ理想盤面の場合にフラグを立てる
                        is_good: Some(transition_count as u16 - win_0_count >= 3 && board.is_good()),
                    });
                    current_map.wins += 1;
                }
            }
        }

        if current_map.wins == 0 && current_map.loses == 0 {
            break;
        }

        board_map.merge(current_map);

        depth += 1;
    }

    let mut win_map: FnvHashMap<u8, u32> = FnvHashMap::default();
    let mut lose_map: FnvHashMap<u8, u32> = FnvHashMap::default();
    let mut unknown_count = 0_u32;

    for (&board, &state) in board_map.map.iter() {
        if state.result == Board::BoardResult::Unknown {
            if unknown_count == 0 {
                println!("Example of Unknown Board:");
                board.print();
                println!("{:?}", state);
            }
            unknown_count += 1;
        }

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
                            println!("Example of Move-{} Win Board:", depth);
                            board.print();
                            println!("{:?}", state);
                        }

                        win_map.insert(depth, wins + 1);
                    },
                    Board::BoardResult::Lose => {
                        let loses = match lose_map.get(&depth) {
                            Some(&count) => count,
                            None => 0,
                        };

                        if loses == 0 {
                            println!("Example of Move-{} Lose Board:", depth);
                            board.print();
                            println!("{:?}", state);
                        }

                        lose_map.insert(depth, loses + 1);
                    },
                    _ => {},
                }
            },
        }
    }

    println!("Total Boards: (wins: {}, loses: {}, unknowns: {})", board_map.wins, board_map.loses, unknown_count);
    println!("Total Possible 打ち歩詰めs: {}", 打ち歩詰め_count);

    for i in 0..60 {
        let wins = match win_map.get(&i) {
            Some(&count) => count,
            None => 0,
        };
        let loses = match lose_map.get(&i) {
            Some(&count) => count,
            None => 0,
        };

        println!("Move-{} Boards: (wins: {}, loses: {})", i, wins, loses);
    }

    let board = Board::Board::Empty();
    let board = board.set_grid(1, 0, Grid::Grid {piece: Piece::Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(1, 1, Grid::Grid {piece: Piece::Piece::飛車, player: 0, promoted: false});
    let board = board.set_grid(2, 2, Grid::Grid {piece: Piece::Piece::王将, player: 1, promoted: false});
    let board = board.add_hand(1, Piece::Piece::飛車, 1);

    println!("State of This Borad:");
    board.print();
    println!("{:?}", board_map.map.get(&board));

    println!("State of This Borad:");
    board.reverse().print();
    println!("{:?}", board_map.map.get(&(board.reverse())));

    let board = Board::Board::Empty();
    let board = board.set_grid(0, 0, Grid::Grid {piece: Piece::Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(2, 2, Grid::Grid {piece: Piece::Piece::王将, player: 1, promoted: false});
    let board = board.add_hand(0, Piece::Piece::銀将, 1);
    let board = board.add_hand(1, Piece::Piece::金将, 1);

    println!("State of This Borad:");
    board.print();
    println!("{:?}", board_map.map.get(&board));

    println!("State of This Borad:");
    board.reverse().print();
    println!("{:?}", board_map.map.get(&(board.reverse())));

    println!("Writing out to {}:", filename);

    board_map.write(filename.to_string());
}
