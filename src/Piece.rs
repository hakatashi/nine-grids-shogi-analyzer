#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Piece {
    Empty,
    王将,
    飛車,
    角行,
    金将,
    銀将,
    桂馬,
    香車,
    歩兵,
}

impl Piece {
    pub fn to_char(&self) -> &str {
        match self {
            &Piece::Empty => "　",
            &Piece::王将 => "王",
            &Piece::飛車 => "飛",
            &Piece::角行 => "角",
            &Piece::金将 => "金",
            &Piece::銀将 => "銀",
            &Piece::桂馬 => "桂",
            &Piece::香車 => "香",
            &Piece::歩兵 => "歩",
        }
    }
}
