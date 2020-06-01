use ggez::graphics::{Color, DrawMode, DrawParam, Font, Mesh, MeshBuilder, Rect, Scale, Text};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub struct Button {
    location: Point2<f32>,
    text_location: Point2<f32>,
    width: f32,
    height: f32,
    text: Text,
    text_color: Color,
    mesh: Mesh,
}

impl Button {
    pub fn new(x: f32, y: f32, text: &'static str, context: &mut Context) -> GameResult<Button> {
        let location = Point2::new(x, y);
        let mut text = Text::new(text);

        text.set_font(Font::default(), Scale::uniform(24.0));
        let width = text.width(context) as f32 + 10.0;
        let height = text.height(context) as f32 + 10.0;
        let text_location = Point2::new(x + 5.0, y + 5.0);
        let color = Color::new(0.8, 0.8, 0.8, 1.0);
        let text_color = graphics::BLACK;

        let mesh = MeshBuilder::new()
            .rectangle(DrawMode::fill(), Rect::new(x, y, width, height), color)
            .build(context)?;

        Ok(Button {
            location,
            text_location,
            width,
            height,
            text,
            mesh,
            text_color,
        })
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        graphics::draw(context, &self.mesh, DrawParam::default())?;
        graphics::draw(
            context,
            &self.text,
            DrawParam::default()
                .dest(self.text_location)
                .color(self.text_color),
        )?;

        Ok(())
    }

    pub fn is_being_clicked(&self, mouse_location: Point2<f32>) -> bool {
        self.location.x < mouse_location.x
            && self.location.x + self.width > mouse_location.x
            && self.location.y < mouse_location.y
            && self.location.y + self.height > mouse_location.y
    }
}
