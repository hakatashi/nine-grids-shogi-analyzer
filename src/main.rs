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
    println!("Wins: {}", board_map.wins);
    println!("Loses: {}", board_map.loses);
}
