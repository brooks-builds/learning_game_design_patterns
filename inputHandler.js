class InputHandler {
    constructor(commands) {
        this.keyBinds = {
            jump: 32 // space
        };
        this.commands = commands;
    }

    bind(keyCode) {
        const rebindingKey = this.whatIsRebinding;
        if (rebindingKey) {
            this.keyBinds[rebindingKey] = keyCode;
        }
    }

    get isRebinding() {
        for (let command in this.keyBinds) {
            if (this.keyBinds[command] == null) return true;
        }
        return false;
    }

    get whatIsRebinding() {
        for (let command in this.keyBinds) {
            if (this.keyBinds[command] == null) return command;
        }
        return null;
    }

    startRebinding(command) {
        this.keyBinds[command] = null;
    }

    handleInput() {
        // what keycode is being pressed?
        // what command is associated with that keycode?
        // return command
        // otherwise return null
    }
}