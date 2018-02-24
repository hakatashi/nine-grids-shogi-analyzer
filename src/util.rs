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

// 例えばサイズ n (= max_pieces + 1) = 3 に対して
//     [(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (2, 0)]
// のようになるような (a, b) の配列に対してインデックス (= hand_data) からペアを取得する処理
// a = i 未満となる要素の数Nは N = (2n - i) * i / 2
// ここからiを解くと i = n - sqrt(n^2 - 2N)
// つまりNにインデックスを代入して求められるiを切り捨てたものがaの値となる。
pub fn hand_data_to_hand_info(hand_data: u8, max_pieces: u8) -> HandInfo {
    let size = max_pieces + 1;
    let i = size - (size * size - hand_data * 2).integer_sqrt();
    let N = (size * 2 - i) * i / 2;
    return HandInfo {
        first: i,
        second: hand_data - N,
    };
}
