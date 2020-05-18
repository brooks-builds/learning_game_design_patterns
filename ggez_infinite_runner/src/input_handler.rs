use super::jump_command::JumpCommand;
use super::reset_game_command::ResetGameCommand;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard;
use ggez::Context;

pub struct InputHandler {
    jump_command: JumpCommand,
    jump_command_keycode: KeyCode,
    reset_game_command: ResetGameCommand,
    reset_game_command_keycode: KeyCode,
}

impl InputHandler {
    pub fn new(jump_command: JumpCommand, reset_game_command: ResetGameCommand) -> InputHandler {
        let jump_command_keycode = KeyCode::Space;
        let reset_game_command_keycode = KeyCode::Space;

        InputHandler {
            jump_command,
            jump_command_keycode,
            reset_game_command,
            reset_game_command_keycode,
        }
    }

    pub fn handle_player_input(&self, context: &mut Context) -> Option<&JumpCommand> {
        if keyboard::is_key_pressed(context, self.jump_command_keycode) {
            Some(&self.jump_command)
        } else {
            None
        }
    }

    pub fn handle_game_input(&self, context: &mut Context) -> Option<&ResetGameCommand> {
        if keyboard::is_key_pressed(context, self.reset_game_command_keycode) {
            Some(&self.reset_game_command)
        } else {
            None
        }
    }
}