use super::jump_command::JumpCommand;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard;
use ggez::Context;

pub struct InputHandler {
    jump_command: JumpCommand,
    jump_command_keycode: KeyCode,
}

impl InputHandler {
    pub fn new(jump_command: JumpCommand) -> InputHandler {
        let jump_command_keycode = KeyCode::Space;

        InputHandler {
            jump_command,
            jump_command_keycode
        }
    }

    pub fn handle_input(&self, context: &mut Context) -> Option<&JumpCommand> {
        if keyboard::is_key_pressed(context, self.jump_command_keycode) {
            Some(&self.jump_command)
        } else {
            None
        }
    }
}