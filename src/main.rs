mod tile;

use quicksilver::prelude::*;
use tile::Tile;

struct Game {
    title: Asset<Image>,
    mononoki_font_info: Asset<Image>
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

        Ok(
            Self {
                title,
                mononoki_font_info
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

fn generate_map(width: i32, length: i32) -> Vec<Tile> {
    let mut map = Vec::with_capacity((width * length) as usize);

    for x in 0..width {
        for y in 0..length {
            let mut tile = Tile {
                position: Vector::new(x, y),
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