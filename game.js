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

}

function draw() {
    if (gameRunning) {
        clear();
        fill(0);
        rect(0, height - 5, width, 5);
        player.render();

        if (keyIsDown(KEYCODE_SPACE)) {
            player.jump();
        }
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
    } else {
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

        if (keyIsDown(KEYCODE_SPACE)) {
            gameRunning = true;
            player.reset();
            obstacles.forEach(obstacle => obstacle.initialize());
            initializeGameSpeed();
        }
    }
    textSize(18);
    text(`Score: ${player.score}`, 5, 20);
    text(`Space bound to: ${inputHandler.keyBinds.jump}`, 5, 40);
}

function initializeGameSpeed() {
    runSpeed = createVector(-5, 0);
}

function keyPressed() {
    if (inputHandler.isRebinding) {
        console.log('rebinding');
        inputHandler.bind(keyCode);
    }
}

function generateCommands() {
    return {
        jump: function (actor) {
            actor.jump();
        },
    };
}

let player;
let gravity;
let obstacles = [];
let runSpeed;
let gameRunning;
let inputHandler = new InputHandler(generateCommands());