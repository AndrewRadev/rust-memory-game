use ggez::{
    Context,
    ContextBuilder,
    GameResult,
    conf::{Conf, WindowMode},
    event,
    graphics,
    input::mouse,
};

use std::env;
use std::path;

use memory_game::cards::Card;

#[derive(Debug)]
struct MainState {
    card: Card,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut card = Card::new("card_hearts_J");
        card.load(ctx)?;

        Ok(MainState { card })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while ctx.time.check_update_time(DESIRED_FPS) {
            let seconds = ctx.time.delta().as_secs_f32();

            if ctx.mouse.button_pressed(mouse::MouseButton::Left) {
                self.card.trigger_flip();
            }

            self.card.update(seconds);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dark_blue = graphics::Color::from_rgb(26, 51, 77);
        let mut canvas = graphics::Canvas::from_frame(ctx, dark_blue);

        // Draw a single card in the center:
        self.card.draw(600.0, 500.0, &mut canvas);

        canvas.finish(ctx)?;

        Ok(())
    }
}

fn main() {
    let conf = Conf::new().
        window_mode(WindowMode {
            width: 1200.0,
            height: 1000.0,
            ..Default::default()
        });
    let (mut ctx, event_loop) = ContextBuilder::new("memory-game", "Andrew").
        default_conf(conf.clone()).
        build().
        unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.fs.mount(&path, true);
    }

    let state = MainState::new(&mut ctx).unwrap();

    event::run(ctx, event_loop, state);
}
