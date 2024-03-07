mod tile;
mod entity;

use quicksilver::prelude::*;
use tile::Tile;
use crate::entity::Entity;

struct Game {
    title: Asset<Image>,
    mononoki_font_info: Asset<Image>,
    map_size: (usize, usize),
    map: Vec<Tile>,
    entities: Vec<Entity>,
    player_index: usize
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

        Ok(
            Self {
                title,
                mononoki_font_info,
                map_size,
                map,
                entities,
                player_index
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

            if x == 0 || x == width - 1 || y == 0 || x == length - 1 {
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