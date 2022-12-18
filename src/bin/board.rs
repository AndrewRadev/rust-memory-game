use ggez::{
    Context,
    ContextBuilder,
    GameResult,
    conf::{Conf, WindowMode},
    event,
    graphics,
    input::mouse,
};
use rand::seq::SliceRandom;

use std::env;
use std::path;

use memory_game::board::Board;
use memory_game::cards;
use memory_game::debug;

#[derive(Debug)]
struct MainState {
    board: Board,
}

impl MainState {
    fn new(ctx: &mut Context, conf: Conf) -> GameResult<MainState> {
        let mut rng = rand::thread_rng();
        let mut board = Board::new(3, 4, conf.window_mode.width, conf.window_mode.height);

        let mut all_cards = cards::all();
        all_cards.shuffle(&mut rng);

        let mut coordinates = Vec::new();
        for x in 0..4 {
            for y in 0..3 {
                coordinates.push((x, y));
            }
        }
        coordinates.shuffle(&mut rng);

        for _ in 0..6 {
            let mut card = all_cards.pop().unwrap();
            let first_coordinate = coordinates.pop().unwrap();
            let second_coordinate = coordinates.pop().unwrap();

            card.load(ctx)?;

            board.set_card(first_coordinate.1, first_coordinate.0, card.clone());
            board.set_card(second_coordinate.1, second_coordinate.0, card);
        }

        Ok(MainState { board })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while ctx.time.check_update_time(DESIRED_FPS) {
            let seconds = ctx.time.delta().as_secs_f32();

            self.board.update(seconds);

            if ctx.mouse.button_pressed(mouse::MouseButton::Left) {
                let mouse_position = ctx.mouse.position();

                let x = mouse_position.x;
                let y = mouse_position.y;

                if let Some(card) = self.board.interact_with_card(x, y) {
                    card.trigger_flip();
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dark_blue = graphics::Color::from_rgb(26, 51, 77);
        let mut canvas = graphics::Canvas::from_frame(ctx, dark_blue);

        self.board.draw(&mut canvas, ctx)?;

        if debug::is_active() {
            let mouse_position = ctx.mouse.position();

            let x = mouse_position.x;
            let y = mouse_position.y;

            debug::draw_circle(x, y, &mut canvas, ctx);
        }

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

    let state = MainState::new(&mut ctx, conf.clone()).unwrap();

    event::run(ctx, event_loop, state);
}
