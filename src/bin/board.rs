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

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            self.board.update(seconds);

            if mouse::button_pressed(ctx, mouse::MouseButton::Left) {
                let mouse_position = mouse::position(ctx);

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
        graphics::clear(ctx, dark_blue);

        self.board.draw(ctx)?;

        if debug::is_active() {
            let mouse_position = mouse::position(ctx);

            let x = mouse_position.x;
            let y = mouse_position.y;

            debug::draw_circle(x, y, ctx).unwrap();
        }

        graphics::present(ctx)?;

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
        filesystem::mount(&mut ctx, &path, true);
    }

    let state = MainState::new(&mut ctx, conf.clone()).unwrap();

    event::run(ctx, event_loop, state);
}
