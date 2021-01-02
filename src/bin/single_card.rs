use ggez::{
    Context,
    ContextBuilder,
    GameResult,
    conf::{Conf, WindowMode},
    event,
    filesystem,
    graphics,
    input::mouse,
    timer,
};

use std::env;
use std::path;

use memory_game::cards::Card;

#[derive(Debug)]
struct MainState {
    conf: Conf,
    card: Card,
}

impl MainState {
    fn new(ctx: &mut Context, conf: Conf) -> GameResult<MainState> {
        let mut card = Card::new("card_hearts_J");
        card.load(ctx)?;

        Ok(MainState { conf, card })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            if mouse::button_pressed(ctx, mouse::MouseButton::Left) {
                self.card.trigger_flip();
            }

            self.card.update(seconds);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dark_blue = graphics::Color::from_rgb(26, 51, 77);
        graphics::clear(ctx, dark_blue);

        // Draw a single card in the center:
        self.card.draw(400.0, 300.0, ctx)?;

        graphics::present(ctx)?;

        Ok(())
    }
}

fn main() {
    let conf = Conf::new().
        window_mode(WindowMode {
            width: 800.0,
            height: 600.0,
            ..Default::default()
        });
    let (mut ctx, event_loop) = ContextBuilder::new("memory-game", "Andrew").
        conf(conf.clone()).
        build().
        unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        filesystem::mount(&mut ctx, &path, true);
    }

    let state = MainState::new(&mut ctx, conf.clone()).unwrap();

    event::run(ctx, event_loop, state);
}
