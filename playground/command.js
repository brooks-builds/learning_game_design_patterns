class InputHandler {
    constructor(jumpCommand) {
        this.jumpCommand = jumpCommand;
    }

    handleInput() {
        if (keyIsDown(this.jumpKeyCode)) return this.jumpButton;

        return null;
    }

    handleMoveInput() {
        const unit = someGlobalGetSelectedUnit();

        if (keyIsDown(this.moveUpKeyCode)) {
            const destinationY = unit.location.y - 1;
            return new MoveUnitCommand(unit, unit.location.x, destinationY);
        }

        return null;
    }

    bindJump(keyCode) {
        this.jumpButton = this.jumpCommand;
        this.jumpKeyCode = keyCode;
    }

    bindUp(keyCode) {
        this.moveUpKeyCode = keyCode;
    }
}

class Command {
    execute() { }
}

class JumpCommand extends Command {
    execute(actor) {
        actor.jump();
    }
}

// setup
const inputHandler = new InputHandler(new JumpCommand());
inputHandler.bindJump(32);

// game loop
const command = inputHandler.handleInput()

if (command) {
    command.execute(actor);
}

// move using example

class MoveUnitCommand extends Command {
    constructor(unit, x, y) {
        this.unit = unit;
        this.x = x;
        this.y = y;
    }

    execute() {
        this.unit.moveTo(x, y);
    }
}