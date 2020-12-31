use ggez::{Context, GameResult, graphics};
use ggez::mint::{Point2, Vector2};

const BASE_SCALE_X:  f32 = 1.5;
const BASE_SCALE_Y:  f32 = 1.5;
const FLIP_DURATION: f32 = 1.0;

#[derive(Debug)]
pub enum CardState {
    Front,
    Back,
}

#[derive(Debug)]
pub struct Card {
    image_front: graphics::Image,
    image_back: graphics::Image,
    state: CardState,
    animation: FlipAnimation,
}

impl Card {
    pub fn load(path: &str, ctx: &mut Context) -> GameResult<Self> {
        let image_front = graphics::Image::new(ctx, path)?;
        let image_back  = graphics::Image::new(ctx, "/cards/card_back.png")?;
        let state       = CardState::Front;
        let animation   = FlipAnimation::new(FLIP_DURATION);

        Ok(Card { image_front, image_back, state, animation })
    }

    pub fn update(&mut self, seconds: f32) {
        self.animation.update(seconds);

        if self.animation.progress >= 1.0 {
            self.flip();
        }
    }

    pub fn draw(&self, x: f32, y: f32, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, self.visible_image(), graphics::DrawParam {
            dest: Point2 { x, y },
            offset: Point2 { x: 0.5, y: 0.5 },
            scale: Vector2 {
                x: BASE_SCALE_X * self.animation.scale_x,
                y: BASE_SCALE_Y * self.animation.scale_y,
            },
            .. Default::default()
        })
    }

    fn visible_image(&self) -> &graphics::Image {
        match self.state {
            CardState::Front => &self.image_front,
            CardState::Back  => &self.image_back,
        }
    }

    fn flip(&mut self) {
        match self.state {
            CardState::Front => self.state = CardState::Back,
            CardState::Back  => self.state = CardState::Front,
        }
    }
}

#[derive(Debug)]
struct FlipAnimation {
    pub scale_x: f32,
    pub scale_y: f32,

    /// Number of seconds to animate in one direction
    duration: f32,

    /// Progress of the animation 0..1
    progress: f32,

    /// Positive or negative change in direction: -1.0 or +1.0
    direction: f32,
}

impl FlipAnimation {
    fn new(duration: f32) -> Self {
        let direction = 1.0;
        let progress  = 0.0;
        let scale_x   = 1.0;
        let scale_y   = 1.0;

        FlipAnimation {
            scale_x, scale_y,
            duration, progress, direction,
        }
    }

    pub fn update(&mut self, seconds: f32) {
        self.progress += self.direction * seconds;

        // Flip conditions:
        if self.progress >= self.duration {
            self.progress = self.duration;
            self.direction = -1.0;
        } else if self.progress <= 0.0 {
            self.progress = 0.0;
            self.direction = 1.0;
        }

        self.scale_x = 1.0 - (self.progress / self.duration);

        // For an additional Y-axis effect:
        // self.scale_y = 1.0 + (self.progress / self.duration) * 0.1;
    }
}
