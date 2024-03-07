use quicksilver::geom::Vector;
use quicksilver::graphics::Color;

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub position: (usize, usize),
    pub glyph: char,
    pub color: Color,
    pub hp: i32,
    pub max_hp: i32
}