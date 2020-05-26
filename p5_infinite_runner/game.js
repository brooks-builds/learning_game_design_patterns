function setup() {
    createCanvas(770, 700);
    player = new Player(createVector(50, 25));
    gravity = createVector(0, 1);
    obstacles.push(
        new Obstacle(createVector(800, 680)),
        new Obstacle(createVector(1200, 680))
    );
    initializeGameSpeed();
    gameRunning = true;
    world = new World();
}

function draw() {
    if (inputHandler.isRebinding == false) {
        const commands = inputHandler.handleInput(gameRunning);
        commands.forEach(command => command(player));
    }
    
    if (gameRunning && inputHandler.isRebinding == false) {
        clear();
        world.run();
        alpha(255);
        fill(0);
        rect(0, height - 5, width, 5);
        player.render();

        player.update();
        player.applyForce(gravity);
        player.hitGround(height - 5);
        obstacles.forEach(obstacle => {
            obstacle.render();
            obstacle.update(runSpeed, player);
            if (player.isHitting(obstacle)) {
                gameRunning = false;
            }
        });
        runSpeed.x -= 0.01;
    } else if (gameRunning == false) {
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
    text(`Score: ${player.score}`, 5, 20);
    text(`Space bound to: ${inputHandler.keyBinds.jump.keyCode}`, 5, 40);
    text(`Restart Game bound to: ${inputHandler.keyBinds.restartGame.keyCode}`, 5, 60);
}

function createTree() {

}

function initializeGameSpeed() {
    runSpeed = createVector(-5, 0);
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
            gameRunning = true;
            player.reset();
            obstacles.forEach(obstacle => obstacle.initialize());
            initializeGameSpeed();
        }
    };
}

let player;
let gravity;
let obstacles = [];
let runSpeed;
let gameRunning;
let inputHandler = new InputHandler(generateCommands());
let world;