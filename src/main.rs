#![feature(non_ascii_idents)]
#![allow(non_snake_case)]

extern crate integer_sqrt;

mod util;
mod Board;
mod Grid;
mod Piece;

fn main() {
    let board = Board::Board {
        grids: 0b0000000000000000000_00111_00111_00110_00101_00100_00011_00010_00001_00000,
        hands: 0b000000000_00010_1000_0100_0001_0100_010_001,
        player: true,
    };

    board.print();

    let board = board.set_grid(1, 1, Grid::Grid {piece: Piece::Piece::飛車, player: 1, promoted: false});

    board.print();

    let board = board.del_grid(2, 2);

    board.print();
}
