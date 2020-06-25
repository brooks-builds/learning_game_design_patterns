function setup() {
  createCanvas(770, 700);

  const jumpOverObstacleEvent = new EventSystem();
  const collidedEvent = new EventSystem();

  gameState = new GameState(jumpOverObstacleEvent, collidedEvent);
  world = new World();

  const player = new Player(
    createVector(50, 25),
    jumpOverObstacleEvent,
    collidedEvent
  );
  world.registerEntity(player);
}

function draw() {
  if (inputHandler.isRebinding == false) {
    const commands = inputHandler.handleInput(gameState.running);
    commands.forEach((command) => command(world));
  }

  if (gameState.running && inputHandler.isRebinding == false) {
    clear();
    world.update();
    alpha(255);
    fill(0);
    rect(0, height - 5, width, 5);
    world.render();
    const players = world.getEntityByType(types.player);
    gameState.getObstacles.forEach((obstacle) => {
      obstacle.render();
      obstacle.update(gameState.getRunSpeed, players[0]);
      isPlayerHittingAnObstacle(world, obstacle);
    });
    gameState.setRunSpeed = gameState.getRunSpeed.x - 0.1;
  } else if (gameState.running == false) {
    textSize(30);
    const gameOverText = "Game Over";

    text(
      gameOverText,
      width / 2 - textWidth(gameOverText) / 2,
      height / 2 - 15
    );
    textSize(15);
    const restartGameText = "Press space to restart";
    text(
      restartGameText,
      width / 2 - textWidth(restartGameText) / 2,
      height / 2 + 10
    );
  }
  drawInterface(world);
}

function keyPressed() {
  if (inputHandler.isRebinding) {
    inputHandler.bind(keyCode);
  }
}

function generateCommands() {
  return {
    jump: function (world) {
      const [player] = world.getEntityByType(types.player);
      player.handleInput("jump");
    },
    restartGame: function (world) {
      const [player] = world.getEntityByType(types.player);
      gameState.running = true;
      player.reset();
      gameState.getObstacles.forEach((obstacle) => obstacle.initialize());
      gameState.initializeGameSpeed();
    },
  };
}

let inputHandler = new InputHandler(generateCommands());
let gameState;
let world;

function isPlayerHittingAnObstacle(world, obstacle) {
  const players = world.getEntityByType(types.player);

  if (
    players[0].location.x < obstacle.location.x + obstacle.width &&
    players[0].location.x + players[0].width > obstacle.location.x &&
    players[0].location.y < obstacle.location.y + obstacle.height &&
    players[0].location.y + players[0].height > obstacle.location.y
  ) {
    players[0].collidedEvent.notify(players[0], COLLIDE_WITH_OBSTACLE);
  }
}

function drawInterface(world) {
  const [player] = world.getEntityByType(types.player);
  textSize(18);
  text(`Score: ${player.score}`, 5, 20);
  text(`Space bound to: ${inputHandler.keyBinds.jump.keyCode}`, 5, 40);
  text(
    `Restart Game bound to: ${inputHandler.keyBinds.restartGame.keyCode}`,
    5,
    60
  );
}
