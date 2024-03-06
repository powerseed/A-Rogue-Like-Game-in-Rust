use quicksilver::geom::Vector;
use quicksilver::prelude::{Color};

#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub position: Vector,
    pub glyph: char,
    pub color: Color
}

