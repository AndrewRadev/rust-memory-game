use ggez::{Context, GameResult, graphics};
use ggez::mint::{Point2, Vector2};

#[derive(Debug)]
pub struct Card {
    image_front: graphics::Image,
    image_back: graphics::Image,
}

impl Card {
    pub fn load(path: &str, ctx: &mut Context) -> GameResult<Self> {
        let image_front = graphics::Image::new(ctx, path)?;
        let image_back  = graphics::Image::new(ctx, "/cards/card_back.png")?;

        Ok(Card { image_front, image_back })
    }

    pub fn update(&mut self, _seconds: f32) {
        // Do nothing for now
    }

    pub fn draw(&self, x: f32, y: f32, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.image_front, graphics::DrawParam {
            dest: Point2 { x, y },
            offset: Point2 { x: 0.5, y: 0.5 },
            scale: Vector2 { x: 1.5, y: 1.5 },
            .. Default::default()
        })
    }
}
