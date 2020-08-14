use super::States;
use ggez::graphics::{DrawParam, Font, Scale, Text};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub struct Interface {
    not_playing_text: Text,
    won_text: Text,
    restart_text: Text,
    dead_text: Text,
}

impl Interface {
    pub fn new() -> Interface {
        let mut not_playing_text = Text::new("Press ENTER to start");
        not_playing_text.set_font(Font::default(), Scale::uniform(72.0));
        let mut won_text = Text::new("You Won!!!");
        won_text.set_font(Font::default(), Scale::uniform(72.0));
        let mut dead_text = Text::new("Really???");
        dead_text.set_font(Font::default(), Scale::uniform(72.0));
        let mut restart_text = Text::new("Press ENTER to play again");
        restart_text.set_font(Font::default(), Scale::uniform(64.0));

        Interface {
            not_playing_text,
            won_text,
            restart_text,
            dead_text,
        }
    }
    pub fn draw(&self, state: &States, context: &mut Context) -> GameResult {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        match state {
            States::NotStarted => {
                let (text_width, text_height) = self.not_playing_text.dimensions(context);
                let text_width = text_width as f32;
                let text_height = text_height as f32;
                graphics::draw(
                    context,
                    &self.not_playing_text,
                    DrawParam::new().dest(Point2::new(
                        screen_width / 2.0 - text_width / 2.0,
                        screen_height / 2.0 - text_height / 2.0,
                    )),
                )?;
            }
            States::Won => {
                let (text_width, text_height) = self.won_text.dimensions(context);
                let text_width = text_width as f32;
                let text_height = text_height as f32;
                graphics::draw(
                    context,
                    &self.won_text,
                    DrawParam::new().dest(Point2::new(
                        screen_width / 2.0 - text_width / 2.0,
                        screen_height / 2.0 - text_height,
                    )),
                )?;

                let (text_width, text_height) = self.restart_text.dimensions(context);
                let text_width = text_width as f32;
                let text_height = text_height as f32;
                graphics::draw(
                    context,
                    &self.restart_text,
                    DrawParam::new().dest(Point2::new(
                        screen_width / 2.0 - text_width / 2.0,
                        screen_height / 2.0 + text_height / 2.0,
                    )),
                )?;
            }
            States::Died => {
                let (text_width, text_height) = self.dead_text.dimensions(context);
                let text_width = text_width as f32;
                let text_height = text_height as f32;
                graphics::draw(
                    context,
                    &self.dead_text,
                    DrawParam::new().dest(Point2::new(
                        screen_width / 2.0 - text_width / 2.0,
                        screen_height / 2.0 - text_height,
                    )),
                )?;

                let (text_width, text_height) = self.restart_text.dimensions(context);
                let text_width = text_width as f32;
                let text_height = text_height as f32;
                graphics::draw(
                    context,
                    &self.restart_text,
                    DrawParam::new().dest(Point2::new(
                        screen_width / 2.0 - text_width / 2.0,
                        screen_height / 2.0 + text_height / 2.0,
                    )),
                )?;
            }
            _ => (),
        }

        Ok(())
    }
}
