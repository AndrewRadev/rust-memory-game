use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::mint::Point2;

pub fn is_active() -> bool {
    std::env::var("DEBUG").is_ok()
}

pub fn draw_box(rect: graphics::Rect, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult<()>  {
    let draw_mode = graphics::DrawMode::Stroke(graphics::StrokeOptions::default().with_line_width(1.0));
    let red = graphics::Color::from_rgb(255, 0, 0);

    let mut mesh_builder = graphics::MeshBuilder::new();
    let outline_data = mesh_builder.
        rectangle(draw_mode, rect, red)?.
        build();
    let outline = graphics::Mesh::from_data(ctx, outline_data);

    canvas.draw(&outline, graphics::DrawParam::default());
    Ok(())
}

pub fn draw_circle(x: f32, y: f32, canvas: &mut graphics::Canvas, ctx: &mut Context) {
    let draw_mode = graphics::DrawMode::Fill(graphics::FillOptions::default());
    let red = graphics::Color::from_rgb(255, 0, 0);
    let circle = graphics::Mesh::new_circle(ctx, draw_mode, Point2 { x, y }, 10.0, 1.0, red).
        unwrap();

    canvas.draw(&circle, graphics::DrawParam::default());
}
