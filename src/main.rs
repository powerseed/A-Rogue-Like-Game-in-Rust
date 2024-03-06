use quicksilver::{run, Graphics, Input, Result, Settings, Window};
use quicksilver::geom::Vector;

fn main() {
    run(
        Settings {
            title: "Roguelike game in Rust",
            size: Vector::new(800.0, 600.0),
            ..Settings::default()
        },
        app,
    );
}

async fn app(_window: Window, _gfx: Graphics, mut input: Input) -> Result<()> {
    loop {
        while let Some(_) = input.next_event().await {
            // Normally we'd do some processing here
        }
        // And then we'd do updates and drawing here
        // When this loop ends, the window will close and the application will stop
        // If the window is closed, our application will receive a close event and terminate also
    }
}