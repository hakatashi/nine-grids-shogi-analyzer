extern crate integer_sqrt;

use self::integer_sqrt::IntegerSquareRoot;

// ある駒についての先手後手の持ち駒の数を表す
#[derive(PartialEq, Eq, Debug)]
pub struct HandInfo {
    // 先手
    pub first: u8,
    // 後手
    pub second: u8,
}

pub fn sqrt_ceil(n: u16) -> u16 {
    let sqrt_floor = n.integer_sqrt();
    if sqrt_floor * sqrt_floor == n {
        return sqrt_floor;
    } else {
        return sqrt_floor + 1;
    }
}

// 例えばサイズ n (= max_pieces + 1) = 3 に対して
//     [(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (2, 0)]
// のようになるような (a, b) の配列に対してインデックス (= hand_data) からペアを取得する処理
// a = i 未満となる要素の数Nは N = (2n - i + 1) * i / 2
// ここからiを解くと i = (2n + 1 - sqrt(4n^2 + 4n + 1 - 8N)) / 2
// つまりNにインデックスを代入して求められるiを切り捨てたものがaの値となる。
pub fn hand_data_to_hand_info(hand_data: u8, max_pieces: u8) -> HandInfo {
    assert!(hand_data < (max_pieces + 1) * (max_pieces + 2) / 2);
    let size: u16 = (max_pieces + 1) as u16;
    let i: u16 = (2 * size + 1 - sqrt_ceil(4 * size * size + 4 * size + 1 - 8 * (hand_data as u16))) / 2;
    #[allow(non_snake_case)]
    let N: u16 = (size * 2 - i + 1) * i / 2;
    return HandInfo {
        first: i as u8,
        second: (hand_data as u16 - N) as u8,
    };
}

pub fn hand_info_to_hand_data(hand_info: HandInfo, max_pieces: u8) -> u8 {
    assert!(hand_info.first + hand_info.second <= max_pieces);
    let size = max_pieces + 1;
    return (size * 2 - hand_info.first + 1) * hand_info.first / 2 + hand_info.second;
}
