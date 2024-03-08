mod tile;
mod entity;

use std::collections::HashMap;
use quicksilver::prelude::*;
use tile::Tile;
use crate::entity::Entity;

struct Game {
    title: Asset<Image>,
    mononoki_font_info: Asset<Image>,
    map_size: (usize, usize),
    map: Vec<Tile>,
    entities: Vec<Entity>,
    player_index: usize,
    tile_size_in_px: (usize, usize),
    tile_set: Asset<HashMap<char, Image>>
}

impl State for Game {
    fn new() -> Result<Self> {
        let font_mononoki = "mononoki-Regular.ttf";

        let title = Asset::new(
            Font::load(font_mononoki)
                .and_then(|font| {
                    font.render("Roguelike game in Rust", &FontStyle::new(72.0, Color::BLACK))
                })
        );

        let mononoki_font_info = Asset::new(
            Font::load(font_mononoki)
                .and_then(|font| {
                    font.render(
                        "Mononoki font by Matthias Tellen, terms: SIL Open Font License 1.1",
                        &FontStyle::new(20.0, Color::BLACK),
                    )
                })
        );

        let map_size = (20, 15);
        let map = generate_map(&map_size);
        let mut entities = generate_entities();

        let player_entity = Entity {
            position: (5, 3),
            glyph: '@',
            color: Color::BLUE,
            hp: 3,
            max_hp: 5
        };

        let player_index = entities.len();
        entities.push(player_entity);

        let game_glyphs = "#@g.%";
        let tile_size_in_px = (12, 24);
        let tile_set = Asset::new(
            Font::load(font_mononoki)
                .and_then(move |font| {
                        let tiles = font
                            .render(game_glyphs, &FontStyle::new(tile_size_in_px.1 as f32, Color::WHITE))
                            .expect("Could not render the font tileset.");

                        let mut tileset = HashMap::new();

                        for (index, glyph) in game_glyphs.chars().enumerate() {
                            let position = (index as i32 * tile_size_in_px.0 as i32, 0);
                            let tile = tiles.subimage(
                                Rectangle::new(position, Vector::new(tile_size_in_px.0 as f32, tile_size_in_px.1 as f32))
                            );
                            tileset.insert(glyph, tile);
                        }

                        Ok(tileset)
                    }
                )
        );

        Ok(
            Self {
                title,
                mononoki_font_info,
                map_size,
                map,
                entities,
                player_index,
                tile_size_in_px,
                tile_set
            }
        )
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        self.title.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center(
                        (window.screen_size().x as i32 / 2, 40)
                    ),
                Img(&image)
            );
            Ok(())
        })?;

        self.mononoki_font_info.execute(|image| {
            window.draw(
              &image
                  .area()
                  .translate(
                      (2, window.screen_size().y as i32 - 60)
                  ),
              Img(&image)
            );
            Ok(())
        })?;

        self.tile_set.execute(|tile_set| {
            for tile in &self.map {
                if let Some(image) = tile_set.get(&tile.glyph) {
                    let position_px = (tile.position.0 * self.tile_size_in_px.0, tile.position.1 * self.tile_size_in_px.1);
                    window.draw(
                        &Rectangle::new(Vector::new(position_px.0 as f32, position_px.1 as f32), image.area().size()),
                        Blended(&image, tile.color)
                    )
                }
            }
            Ok(())
        })?;

        Ok(())
    }
}

fn main() {
    run::<Game>(
        "Roguelike game in Rust",
        Vector::new(800, 600),
        Settings {
            scale: quicksilver::graphics::ImageScaleStrategy::Blur,
            ..Default::default()
        }
    );
}

fn generate_map(map_size: &(usize, usize)) -> Vec<Tile> {
    let width = map_size.0;
    let length = map_size.1;

    let mut map = Vec::with_capacity(width * length);

    for x in 0..map_size.0 {
        for y in 0..map_size.1 {
            let mut tile = Tile {
                position: (x, y),
                glyph: '.',
                color: Color::BLACK
            };

            if x == 0 || x == width - 1 || y == 0 || y == length - 1 {
                tile.glyph = '#';
            }

            map.push(tile);
        }
    }

    return map;
}

fn generate_entities() -> Vec<Entity> {
    vec![
        Entity {
            position: (9, 6),
            glyph: 'g',
            color: Color::RED,
            hp: 1,
            max_hp: 1,
        },
        Entity {
            position: (2, 4),
            glyph: 'g',
            color: Color::RED,
            hp: 1,
            max_hp: 1,
        },
        Entity {
            position: (7, 5),
            glyph: '%',
            color: Color::PURPLE,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            position: (4, 8),
            glyph: '%',
            color: Color::PURPLE,
            hp: 0,
            max_hp: 0,
        }
    ]
}