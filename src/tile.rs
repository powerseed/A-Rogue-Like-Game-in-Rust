use quicksilver::geom::Vector;
use quicksilver::prelude::{Color};

#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub position: (usize, usize),
    pub glyph: char,
    pub color: Color
}

