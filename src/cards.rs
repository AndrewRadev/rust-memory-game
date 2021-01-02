use ggez::{Context, GameResult, graphics};
use ggez::mint::{Point2, Vector2};

const BASE_SCALE_X:  f32 = 1.5;
const BASE_SCALE_Y:  f32 = 1.5;
pub const FLIP_DURATION: f32 = 0.3;

#[derive(Debug, Clone)]
pub enum CardState {
    Front,
    Back,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub state: CardState,
    pub identifier: String,
    image_front: Option<graphics::Image>,
    image_back: Option<graphics::Image>,
    animation: FlipAnimation,
}

impl Card {
    pub fn new(identifier: &str) -> Self {
        Card {
            identifier: String::from(identifier),
            state:      CardState::Back,
            animation:  FlipAnimation::new(FLIP_DURATION),

            // Images loaded later
            image_front: None,
            image_back: None,
        }
    }

    pub fn load(&mut self, ctx: &mut Context) -> GameResult<()> {
        let path = format!("/cards/{}.png", self.identifier);

        self.image_front = Some(graphics::Image::new(ctx, path)?);
        self.image_back  = Some(graphics::Image::new(ctx, "/cards/card_back.png")?);

        Ok(())
    }

    pub fn update(&mut self, seconds: f32) {
        self.animation.update(seconds);

        if matches!(self.animation.state, FlipAnimationState::BeforeFlip) {
            self.flip();
            self.animation.state = FlipAnimationState::AfterFlip;
        }
    }

    pub fn draw(&self, x: f32, y: f32, ctx: &mut Context) -> GameResult<()> {
        if let Some(image) = self.visible_image() {
            graphics::draw(ctx, image, graphics::DrawParam {
                dest: Point2 { x, y },
                offset: Point2 { x: 0.5, y: 0.5 },
                scale: Vector2 {
                    x: BASE_SCALE_X * self.animation.scale_x,
                    y: BASE_SCALE_Y,
                },
                .. Default::default()
            })?;
        }

        Ok(())
    }

    /// Only starts flip animation if it's already done -- so it can be safely called multiple
    /// times in a row.
    ///
    pub fn trigger_flip(&mut self) {
        if matches!(self.animation.state, FlipAnimationState::Stopped) {
            self.animation.state = FlipAnimationState::Started;
        }
    }

    fn visible_image(&self) -> Option<&graphics::Image> {
        match self.state {
            CardState::Front => self.image_front.as_ref(),
            CardState::Back  => self.image_back.as_ref(),
        }
    }

    fn flip(&mut self) {
        match self.state {
            CardState::Front => self.state = CardState::Back,
            CardState::Back  => self.state = CardState::Front,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FlipAnimationState {
    /// The animation will play
    Started,

    /// The animation is at the point where the card should be flipped
    BeforeFlip,

    /// The card has (hopefully) been flipped now
    AfterFlip,

    /// The animation is not playing
    Stopped,
}

#[derive(Debug, Clone)]
struct FlipAnimation {
    pub scale_x: f32,
    pub state: FlipAnimationState,

    /// Number of seconds to animate in one direction
    duration: f32,

    /// Progress of the animation: 0 <= progress <= duration
    progress: f32,

    /// Positive or negative change in direction: -1.0 or +1.0
    direction: f32,
}

impl FlipAnimation {
    fn new(duration: f32) -> Self {
        FlipAnimation {
            scale_x: 1.0,
            state: FlipAnimationState::Stopped,
            progress: 0.0,
            direction: 1.0,
            duration,
        }
    }

    pub fn update(&mut self, seconds: f32) {
        if matches!(self.state, FlipAnimationState::Stopped) {
            return;
        }

        self.progress += self.direction * seconds;

        // Flip conditions:
        if self.progress >= self.duration {
            self.progress = self.duration;
            self.direction = -1.0;
            self.state = FlipAnimationState::BeforeFlip;
        } else if self.progress <= 0.0 {
            self.progress = 0.0;
            self.direction = 1.0;
            self.state = FlipAnimationState::Stopped;
        }

        self.scale_x = 1.0 - (self.progress / self.duration);
    }
}

pub fn all() -> Vec<Card> {
    vec![
        Card::new("card_hearts_A"),
        Card::new("card_hearts_02"),
        Card::new("card_hearts_03"),
        Card::new("card_hearts_04"),
        Card::new("card_hearts_05"),
        Card::new("card_hearts_06"),
        Card::new("card_hearts_07"),
        Card::new("card_hearts_08"),
        Card::new("card_hearts_09"),
        Card::new("card_hearts_10"),
        Card::new("card_hearts_J"),
        Card::new("card_hearts_Q"),
        Card::new("card_hearts_K"),
        Card::new("card_empty"),
        Card::new("card_diamonds_A"),
        Card::new("card_diamonds_02"),
        Card::new("card_diamonds_03"),
        Card::new("card_diamonds_04"),
        Card::new("card_diamonds_05"),
        Card::new("card_diamonds_06"),
        Card::new("card_diamonds_07"),
        Card::new("card_diamonds_08"),
        Card::new("card_diamonds_09"),
        Card::new("card_diamonds_10"),
        Card::new("card_diamonds_J"),
        Card::new("card_diamonds_Q"),
        Card::new("card_diamonds_K"),
        Card::new("card_back"),
        Card::new("card_clubs_A"),
        Card::new("card_clubs_02"),
        Card::new("card_clubs_03"),
        Card::new("card_clubs_04"),
        Card::new("card_clubs_05"),
        Card::new("card_clubs_06"),
        Card::new("card_clubs_07"),
        Card::new("card_clubs_08"),
        Card::new("card_clubs_09"),
        Card::new("card_clubs_10"),
        Card::new("card_clubs_J"),
        Card::new("card_clubs_Q"),
        Card::new("card_clubs_K"),
        Card::new("card_joker_red"),
        Card::new("card_spades_A"),
        Card::new("card_spades_02"),
        Card::new("card_spades_03"),
        Card::new("card_spades_04"),
        Card::new("card_spades_05"),
        Card::new("card_spades_06"),
        Card::new("card_spades_07"),
        Card::new("card_spades_08"),
        Card::new("card_spades_09"),
        Card::new("card_spades_10"),
        Card::new("card_spades_J"),
        Card::new("card_spades_Q"),
        Card::new("card_spades_K"),
        Card::new("card_joker_black"),
    ]
}
