use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::mint::Point2;

pub fn is_active() -> bool {
    std::env::var("DEBUG").is_ok()
}

pub fn draw_box(rect: graphics::Rect, ctx: &mut Context) -> GameResult<()>  {
    let draw_mode = graphics::DrawMode::Stroke(graphics::StrokeOptions::default().with_line_width(1.0));
    let red = graphics::Color::from_rgb(255, 0, 0);
    let outline = graphics::MeshBuilder::new().
        rectangle(draw_mode, rect, red).
        unwrap().
        build(ctx).
        unwrap();

    graphics::draw(ctx, &outline, graphics::DrawParam::default())?;
    Ok(())
}

pub fn draw_circle(x: f32, y: f32, ctx: &mut Context) -> GameResult<()>  {
    let draw_mode = graphics::DrawMode::Fill(graphics::FillOptions::default());
    let red = graphics::Color::from_rgb(255, 0, 0);
    let circle = graphics::Mesh::new_circle(ctx, draw_mode, Point2 { x, y }, 10.0, 1.0, red).
        unwrap();

    graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
    Ok(())
}
