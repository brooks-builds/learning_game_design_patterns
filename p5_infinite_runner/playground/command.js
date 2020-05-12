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
    undo() { }
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
        this.xBefore = 0;
        this.yBefore = 0;
    }

    execute() {
        this.xBefore = this.unit.location.x;
        this.yBefore = this.unit.location.y;
        this.unit.moveTo(x, y);
    }

    undo() {
        this.unit.moveTo(this.xBefore, this.yBefore);
    }
}
