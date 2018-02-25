extern crate fnv;

use self::fnv::FnvHashMap;
use ::Board::{Board, BoardResult};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BoardState {
    result: BoardResult,
    depth: Option<u8>,
}

pub struct BoardMap(pub FnvHashMap<Board, BoardState>);
