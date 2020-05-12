class InputHandler {
    constructor(commands) {
        this.keyBinds = {
            jump: { keyCode: 32, gameRunning: true }, // space
            restartGame: { keyCode: 32, gameRunning: false }, // space
        };
        this.commands = commands;
    }

    bind(keyCode) {
        const rebindingKey = this.whatIsRebinding;
        if (rebindingKey) {
            this.keyBinds[rebindingKey].keyCode = keyCode;
        }
    }

    get isRebinding() {
        for (let command in this.keyBinds) {
            if (this.keyBinds[command].keyCode == null) return true;
        }
        return false;
    }

    get whatIsRebinding() {
        for (let command in this.keyBinds) {
            if (this.keyBinds[command].keyCode == null) return command;
        }
        return null;
    }

    startRebinding(command) {
        this.keyBinds[command].keyCode = null;
    }

    handleInput(gameRunning) {
        const commands = [];
        for (let commandName in this.keyBinds) {
            const isPressingKey = keyIsDown(this.keyBinds[commandName].keyCode);
            const canWeRunCommand = this.keyBinds[commandName].gameRunning == gameRunning;
            if (isPressingKey && canWeRunCommand) {
                commands.push(this.commands[commandName]);
            }
        }
        // return command
        return commands;
    }
}