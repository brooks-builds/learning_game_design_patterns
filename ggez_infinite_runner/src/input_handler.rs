use super::jump_command::JumpCommand;
use super::reset_game_command::ResetGameCommand;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::Context;

enum Rebinding {
    Nothing,
    Jump,
    ResetGame,
}

pub struct InputHandler {
    jump_command: JumpCommand,
    pub jump_command_keycode: KeyCode,
    reset_game_command: ResetGameCommand,
    pub reset_game_command_keycode: KeyCode,
    rebinding: Rebinding,
}

impl InputHandler {
    pub fn new(jump_command: JumpCommand, reset_game_command: ResetGameCommand) -> InputHandler {
        let jump_command_keycode = KeyCode::Space;
        let reset_game_command_keycode = KeyCode::Space;
        let rebinding = Rebinding::Nothing;

        InputHandler {
            jump_command,
            jump_command_keycode,
            reset_game_command,
            reset_game_command_keycode,
            rebinding,
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

    pub fn not_binding(&self) -> bool {
        if let Rebinding::Nothing = self.rebinding {
            true
        } else {
            false
        }
    }

    pub fn is_rebinding(&self) -> bool {
        !self.not_binding()
    }

    pub fn start_binding_jump(&mut self) {
        self.rebinding = Rebinding::Jump;
    }

    pub fn start_binding_reset_game(&mut self) {
        self.rebinding = Rebinding::ResetGame;
    }

    pub fn bind_key(&mut self, keycode: KeyCode) {
        match self.rebinding {
            Rebinding::Jump => self.jump_command_keycode = keycode,
            Rebinding::ResetGame => self.reset_game_command_keycode = keycode,
            Rebinding::Nothing => (),
        };

        self.rebinding = Rebinding::Nothing;
    }

    pub fn get_jump_keycode(&self) -> String {
        if let Rebinding::Jump = self.rebinding {
            "rebinding".to_owned()
        } else {
            format!("{:?}", self.jump_command_keycode)
        }
    }

    pub fn get_reset_game_keycode(&self) -> String {
        if let Rebinding::ResetGame = self.rebinding {
            "rebinding".to_owned()
        } else {
            format!("{:?}", self.reset_game_command_keycode)
        }
    }
}
