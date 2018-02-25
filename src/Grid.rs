use ::Piece::Piece;

// マス
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Grid {
    pub piece: Piece,
    pub player: u8,
    pub promoted: bool,
}

impl Grid {
    pub fn to_i(&self) -> u8 {
        let piece: u8 = match self.piece {
            Piece::Empty => 0,
            Piece::王将 => 1,
            Piece::飛車 => 2,
            Piece::角行 => 4,
            Piece::金将 => 6,
            Piece::銀将 => 7,
            Piece::桂馬 => 9,
            Piece::香車 => 11,
            Piece::歩兵 => 13,
        };

        return piece + self.promoted as u8 + 14 * self.player;
    }
}
