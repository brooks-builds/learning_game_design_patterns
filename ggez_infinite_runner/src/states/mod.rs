pub mod jumping_state;
pub mod standing_state;

use super::Commands;

pub trait State {
    fn handle_input(
        &self,
        _command: &Commands,
        state_data: &mut Box<dyn StateData>,
    ) -> Option<Box<dyn State>>;
    fn update(&self);
}

pub trait StateData {
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn get_location_as_point(&self) -> ggez::nalgebra::Point2<f32>;
}
