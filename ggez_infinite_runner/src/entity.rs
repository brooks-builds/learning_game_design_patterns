use ggez::nalgebra::Vector2;

pub trait Component {}

pub struct Entity {
    name: &'static str,
    components: Vec<Box<dyn Component>>,
}

pub struct Location(Vector2<f32>);

impl Component for Location {}
