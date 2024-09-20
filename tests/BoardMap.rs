extern crate nine_grids_shogi_analyzer;

use nine_grids_shogi_analyzer::Board::{Board};
use nine_grids_shogi_analyzer::BoardMap::{BoardMap};
use nine_grids_shogi_analyzer::Piece::{Piece};
use nine_grids_shogi_analyzer::Grid::{Grid};

#[test]
fn board_map_from_pieces_test() {
    let board_map = BoardMap::from_pieces(vec![]);

    assert_eq!(board_map.map.len(), 72);

    /* 王v王 ・
       ・ ・ ・
       ・ ・ ・ */
    let board = Board::Empty();
    let board = board.set_grid(0, 0, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(1, 0, Grid {piece: Piece::王将, player: 1, promoted: false});
    assert!(board_map.map.contains_key(&board));

    /*v王 ・ ・
       ・ ・ ・
       ・ ・ 王 */
    let board = Board::Empty();
    let board = board.set_grid(2, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(0, 0, Grid {piece: Piece::王将, player: 1, promoted: false});
    assert!(board_map.map.contains_key(&board));

    /* 王 ・ ・
       ・ ・ ・
       ・ ・ ・ */
    let board = Board::Empty();
    let board = board.set_grid(0, 0, Grid {piece: Piece::王将, player: 0, promoted: false});
    assert!(!board_map.map.contains_key(&board));

    // 金将 x 2

    let board_map = BoardMap::from_pieces(vec![Piece::金将, Piece::金将]);

    // 持ち駒0個 9 * 8 * (7 * 6 / 2)[金将の位置] * 4[金将の先手後手 = (先先, 先後, 後先, 後後)] = 6048
    // 持ち駒1個 9 * 8 * 7[金将の位置] * 2[金将の先手後手 = (先, 後)] * 2[持ち駒の先手後手の枚数] = 2126
    // 持ち駒2個 9 * 8 * 3[持ち駒の先手後手の枚数 = (2, 0), (1, 1), (0, 2)] = 216
    // 計: 8280
    assert_eq!(board_map.map.len(), 8280);

    /* 王v王 金
      v金 ・ ・
       ・ ・ ・ */
    let board = Board::Empty();
    let board = board.set_grid(0, 0, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(1, 0, Grid {piece: Piece::王将, player: 1, promoted: false});
    let board = board.set_grid(2, 0, Grid {piece: Piece::金将, player: 0, promoted: false});
    let board = board.set_grid(0, 1, Grid {piece: Piece::金将, player: 1, promoted: false});
    assert!(board_map.map.contains_key(&board));

    /*v王 ・ ・
       ・v金 ・
       ・ ・ 王
       ☗金 */
    let board = Board::Empty();
    let board = board.set_grid(0, 0, Grid {piece: Piece::王将, player: 1, promoted: false});
    let board = board.set_grid(1, 1, Grid {piece: Piece::金将, player: 1, promoted: false});
    let board = board.set_grid(2, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.add_hand(0, Piece::金将, 1);
    assert!(board_map.map.contains_key(&board));

    /* 王 ・ ・
       ・ ・ ・
       ・ ・v王
       ☖金金 */
    let board = Board::Empty();
    let board = board.set_grid(0, 0, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(2, 2, Grid {piece: Piece::王将, player: 1, promoted: false});
    let board = board.add_hand(1, Piece::金将, 2);
    assert!(board_map.map.contains_key(&board));

    // 歩兵 x 2

    let board_map = BoardMap::from_pieces(vec![Piece::歩兵, Piece::歩兵]);

    /*vとv王 ・
       ・ ・ ・
       ・ 王 と */
    let board = Board::Empty();
    let board = board.set_grid(0, 0, Grid {piece: Piece::歩兵, player: 1, promoted: true});
    let board = board.set_grid(1, 0, Grid {piece: Piece::王将, player: 1, promoted: false});
    let board = board.set_grid(1, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(2, 2, Grid {piece: Piece::歩兵, player: 0, promoted: true});
    assert!(board_map.map.contains_key(&board));

    /*v王v歩 ・
       ・ 歩 ・
       ・ ・ 王 */
    let board = Board::Empty();
    let board = board.set_grid(0, 0, Grid {piece: Piece::王将, player: 1, promoted: false});
    let board = board.set_grid(1, 0, Grid {piece: Piece::歩兵, player: 1, promoted: false});
    let board = board.set_grid(1, 1, Grid {piece: Piece::歩兵, player: 0, promoted: false});
    let board = board.set_grid(2, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    assert!(board_map.map.contains_key(&board));

    /* ・v王 と
       ・ ・ ・
       ・ 王 と */
    let board = Board::Empty();
    let board = board.set_grid(1, 0, Grid {piece: Piece::王将, player: 1, promoted: false});
    let board = board.set_grid(2, 0, Grid {piece: Piece::歩兵, player: 0, promoted: true});
    let board = board.set_grid(1, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(2, 2, Grid {piece: Piece::歩兵, player: 0, promoted: true});
    assert!(board_map.map.contains_key(&board));

    /*v王 歩 ・
      v歩 ・ ・
       ・ ・ 王 */
    let board = Board::Empty();
    let board = board.set_grid(0, 0, Grid {piece: Piece::王将, player: 1, promoted: false});
    let board = board.set_grid(1, 0, Grid {piece: Piece::歩兵, player: 0, promoted: false});
    let board = board.set_grid(0, 1, Grid {piece: Piece::歩兵, player: 1, promoted: false});
    let board = board.set_grid(2, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    assert!(!board_map.map.contains_key(&board));

    /*v王 ・ ・
      v歩 ・ ・
       ・v歩 王 */
    let board = Board::Empty();
    let board = board.set_grid(0, 0, Grid {piece: Piece::王将, player: 1, promoted: false});
    let board = board.set_grid(0, 1, Grid {piece: Piece::歩兵, player: 1, promoted: false});
    let board = board.set_grid(1, 2, Grid {piece: Piece::歩兵, player: 1, promoted: false});
    let board = board.set_grid(2, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    assert!(!board_map.map.contains_key(&board));

    /*v王 ・ ・
       ・ ・ 歩
       ・ 王 歩 */
    let board = Board::Empty();
    let board = board.set_grid(0, 0, Grid {piece: Piece::王将, player: 1, promoted: false});
    let board = board.set_grid(2, 1, Grid {piece: Piece::歩兵, player: 0, promoted: false});
    let board = board.set_grid(1, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    let board = board.set_grid(2, 2, Grid {piece: Piece::歩兵, player: 0, promoted: false});
    assert!(!board_map.map.contains_key(&board));

    /*v王v歩 ・
       ・v歩 ・
       ・ 王 ・ */
    let board = Board::Empty();
    let board = board.set_grid(0, 0, Grid {piece: Piece::王将, player: 1, promoted: false});
    let board = board.set_grid(1, 0, Grid {piece: Piece::歩兵, player: 1, promoted: false});
    let board = board.set_grid(1, 1, Grid {piece: Piece::歩兵, player: 1, promoted: false});
    let board = board.set_grid(1, 2, Grid {piece: Piece::王将, player: 0, promoted: false});
    assert!(!board_map.map.contains_key(&board));
}
