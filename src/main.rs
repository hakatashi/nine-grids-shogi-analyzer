#![feature(non_ascii_idents, range_contains)]
#![allow(non_snake_case)]

extern crate integer_sqrt;

mod util;
mod Board;
mod BoardMap;
mod Grid;
mod Piece;

fn main() {
    let board = Board::Board {
        grids: 0b0000000000000000000_00111_00111_00110_00101_00100_00011_01010_00001_00000,
        hands: 0b000000000_00010_1000_0100_0001_0100_001_001,
        player: true,
    };

    board.print();

    println!("Place Piece:");
    let board = board.set_grid(1, 1, Grid::Grid {piece: Piece::Piece::飛車, player: 1, promoted: false});
    let board = board.set_grid(0, 1, Grid::Grid {piece: Piece::Piece::歩兵, player: 1, promoted: false});

    board.print();

    println!("Remove Piece:");
    let board = board.del_grid(2, 2);

    board.print();

    println!("Add Hand:");
    let board = board.add_hand(0, Piece::Piece::歩兵, 1);

    board.print();

    println!("Reverse:");
    let board = board.reverse();

    board.print();

    println!("This board is: {:?}", board.get_result());

    let transitions = board.get_possible_transitions();

    println!("== Possible Transitions ==");

    for transition in transitions {
        transition.print();
    }

    println!("Generate boards from pieces 銀銀歩歩:");

    let board_map = BoardMap::BoardMap::from_pieces(vec![
        Piece::Piece::銀将,
        Piece::Piece::銀将,
        Piece::Piece::歩兵,
        Piece::Piece::歩兵,
    ]);

    println!("Number of generated boards: {}", board_map.map.len());
    println!("Depth-0 Wins: {}", board_map.wins);
    println!("Depth-0 Loses: {}", board_map.loses);

    let mut depth_1_map = BoardMap::BoardMap::Empty();

    for (&board, state) in board_map.map.iter() {
        if state.result == Board::BoardResult::Unknown {
            let transitions = board.get_possible_transitions();

            let mut is_all_win = true;
            let mut is_all_lose = true;
            let mut is_any_lose = false;
            let mut is_any_unknown = false;

            for transition in transitions {
                let transition_state = board_map.map.get(&transition).unwrap();

                match transition_state.result {
                    Board::BoardResult::Win => {
                        is_all_lose = false;
                    },
                    Board::BoardResult::Lose => {
                        is_all_win = false;
                        is_any_lose = true;
                    },
                    Board::BoardResult::Unknown => {
                        is_all_lose = false;
                        is_all_win = false;
                        is_any_unknown = true;
                    },
                }

                if !is_all_lose && !is_all_win && is_any_lose && is_any_unknown {
                    break;
                }
            }

            if is_all_win {
                depth_1_map.map.insert(board, BoardMap::BoardState {result: Board::BoardResult::Lose, depth: Some(1)});
                depth_1_map.loses += 1;
                if depth_1_map.loses == 100000 {
                    println!("100000th Depth-1 Lose Board:");
                    board.print();
                }
            } else if is_all_lose {
                depth_1_map.map.insert(board, BoardMap::BoardState {result: Board::BoardResult::Win, depth: Some(1)});
                depth_1_map.wins += 1;
            } else if !is_any_unknown && is_any_lose {
                depth_1_map.map.insert(board, BoardMap::BoardState {result: Board::BoardResult::Win, depth: Some(1)});
                depth_1_map.wins += 1;
            }
        }
    }

    println!("Depth-1 Wins: {}", depth_1_map.wins);
    println!("Depth-1 Loses: {}", depth_1_map.loses);
}
