use ::Piece::Piece;

pub struct Grid {
    pub piece: Piece,
}

impl Grid {
    pub fn is_promotable(&self) -> bool {
        self.piece != Piece::Empty && self.piece != Piece::王将
    }
}
