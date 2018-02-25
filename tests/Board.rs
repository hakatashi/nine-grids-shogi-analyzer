#![feature(non_ascii_idents)]

extern crate nine_grids_shogi_analyzer;

use nine_grids_shogi_analyzer::Board::{Board, PieceMove, Coord};
use nine_grids_shogi_analyzer::Piece::{Piece};
use nine_grids_shogi_analyzer::Grid::{Grid};

#[test]
fn board_get_possible_moves_test() {
    /*
     * □□□
     * □□□
     * □王□
     */
    let board = Board {grids: 0, hands: 0, player: true};
    let board = board.set_grid(1, 2, Grid {piece: Piece::王将, player: 0, promoted: false});

    let moves = board.get_possible_moves();

    assert_eq!(moves.len(), 5);
    assert_eq!(moves[0], PieceMove {from: Coord {x: 1, y: 2}, to: Coord {x: 0, y: 1}, piece: Piece::王将, promote: false});
    assert_eq!(moves[1], PieceMove {from: Coord {x: 1, y: 2}, to: Coord {x: 1, y: 1}, piece: Piece::王将, promote: false});
    assert_eq!(moves[2], PieceMove {from: Coord {x: 1, y: 2}, to: Coord {x: 2, y: 1}, piece: Piece::王将, promote: false});
    assert_eq!(moves[3], PieceMove {from: Coord {x: 1, y: 2}, to: Coord {x: 0, y: 2}, piece: Piece::王将, promote: false});
    assert_eq!(moves[4], PieceMove {from: Coord {x: 1, y: 2}, to: Coord {x: 2, y: 2}, piece: Piece::王将, promote: false});

    /*
     * □ □ □
     * □ 金 □
     * □ 王 □
     */
    let board = Board {grids: 0, hands: 0, player: true};
    let board = board.set_grid(1, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(1, 1, Grid {piece: Piece::金将, player: 0, promoted: false});
    let moves = board.get_possible_moves();
    assert_eq!(moves.len(), 9);

    /*
     * □ □ □
     * □ 香 □
     * □ 王 □
     */
    let board = Board {grids: 0, hands: 0, player: true};
    let board = board.set_grid(1, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(1, 1, Grid {piece: Piece::香車, player: 0, promoted: false});
    let moves = board.get_possible_moves();
    assert_eq!(moves.len(), 6);

    /*
     * □ □ □
     * □ 銀 □
     * □ 王 □
     */
    let board = Board {grids: 0, hands: 0, player: true};
    let board = board.set_grid(1, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(1, 1, Grid {piece: Piece::銀将, player: 0, promoted: false});
    let moves = board.get_possible_moves();
    assert_eq!(moves.len(), 12);

    /*
     * □ □ □
     * □ 全 □
     * □ 王 □
     */
    let board = Board {grids: 0, hands: 0, player: true};
    let board = board.set_grid(1, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(1, 1, Grid {piece: Piece::銀将, player: 0, promoted: true});
    let moves = board.get_possible_moves();
    assert_eq!(moves.len(), 9);

    /*
     * □ □ □
     * □ 歩v□
     * □ 王 □
     */
    let board = Board {grids: 0, hands: 0, player: true};
    let board = board.set_grid(1, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(1, 1, Grid {piece: Piece::歩兵, player: 1, promoted: false});
    let moves = board.get_possible_moves();
    assert_eq!(moves.len(), 5);

    /*
     * □ □ □
     * □ 歩v□
     * □ 王 角
     */
    let board = Board {grids: 0, hands: 0, player: true};
    let board = board.set_grid(1, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(1, 1, Grid {piece: Piece::歩兵, player: 1, promoted: false});
    let board = board.set_grid(2, 2, Grid {piece: Piece::角行, player: 0, promoted: false});
    let moves = board.get_possible_moves();
    assert_eq!(moves.len(), 5);

    /*
     * □ □ □
     * □ 歩v□
     * □ 王 馬
     */
    let board = Board {grids: 0, hands: 0, player: true};
    let board = board.set_grid(1, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(1, 1, Grid {piece: Piece::歩兵, player: 1, promoted: false});
    let board = board.set_grid(2, 2, Grid {piece: Piece::角行, player: 0, promoted: true});
    let moves = board.get_possible_moves();
    assert_eq!(moves.len(), 6);

    /*
     * □ □ □
     * 歩v歩v歩v
     * 王 桂 □
     */
    let board = Board {grids: 0, hands: 0, player: true};
    let board = board.set_grid(0, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(0, 1, Grid {piece: Piece::歩兵, player: 1, promoted: false});
    let board = board.set_grid(1, 1, Grid {piece: Piece::歩兵, player: 1, promoted: false});
    let board = board.set_grid(2, 1, Grid {piece: Piece::歩兵, player: 1, promoted: false});
    let board = board.set_grid(1, 2, Grid {piece: Piece::桂馬, player: 0, promoted: false});
    let moves = board.get_possible_moves();
    assert_eq!(moves.len(), 6);
}
