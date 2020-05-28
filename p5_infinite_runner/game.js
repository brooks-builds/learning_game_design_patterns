function setup() {
    createCanvas(770, 700);
    gameState = new GameState();
}

function draw() {
    if (inputHandler.isRebinding == false) {
        const commands = inputHandler.handleInput(gameState.running);
        commands.forEach(command => command(gameState.getPlayer));
    }

    if (gameState.running && inputHandler.isRebinding == false) {
        clear();
        gameState.getWorld.run();
        alpha(255);
        fill(0);
        rect(0, height - 5, width, 5);
        gameState.getPlayer.render();
        gameState.getPlayer.update();
        gameState.getPlayer.applyForce(gameState.getGravity);
        gameState.getPlayer.hitGround(height - 5);
        gameState.getObstacles.forEach(obstacle => {
            obstacle.render();
            obstacle.update(gameState.getRunSpeed, gameState.getPlayer);
            gameState.getPlayer.checkIfHitting(obstacle);
        });
        gameState.setRunSpeed = gameState.getRunSpeed.x - 0.1;
    } else if (gameState.running == false) {
        textSize(30);
        const gameOverText = 'Game Over';

        text(gameOverText,
            (width / 2) - (textWidth(gameOverText) / 2),
            (height / 2) - (15)
        );
        textSize(15);
        const restartGameText = 'Press space to restart';
        text(restartGameText,
            (width / 2) - (textWidth(restartGameText) / 2),
            (height / 2) + 10

        );
    }
    textSize(18);
    text(`Score: ${gameState.getPlayer.score}`, 5, 20);
    text(`Space bound to: ${inputHandler.keyBinds.jump.keyCode}`, 5, 40);
    text(`Restart Game bound to: ${inputHandler.keyBinds.restartGame.keyCode}`, 5, 60);
}

function keyPressed() {
    if (inputHandler.isRebinding) {
        inputHandler.bind(keyCode);
    }
}

function generateCommands() {
    return {
        jump: function (actor) {
            actor.jump();
        },
        restartGame: function () {
            gameState.running = true;
            gameState.getPlayer.reset();
            gameState.getObstacles.forEach(obstacle => obstacle.initialize());
            gameState.initializeGameSpeed();
        }
    };
}

let inputHandler = new InputHandler(generateCommands());
let gameState;