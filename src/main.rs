extern crate rand;
extern crate ggez;

use std::env;
use std::path;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

struct MainState {
    text: graphics::Text,
    frames: usize,
}


impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/Hack-Regular.ttf", 48)?;
        let text = graphics::Text::new(ctx, "Hello World", &font)?;

        let s = MainState {
            text: text,
            frames: 0,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let dest_point = graphics::Point2::new(10.0, 10.0);
        graphics::draw(ctx, &self.text, dest_point, 0.0)?;
        graphics::present(ctx);
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS : {}", ggez::timer::get_fps(ctx));
        }
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error : {}", e);
    } else {
        println!("Game exited cleanly");
    }
}
