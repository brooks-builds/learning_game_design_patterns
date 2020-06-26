function setup() {
  createCanvas(770, 700);

  jumpOverObstacleEvent = new EventSystem();
  const collidedEvent = new EventSystem();

  gameState = new GameState(jumpOverObstacleEvent, collidedEvent);
  world = new World();

  const player = new Player(createVector(50, 25), collidedEvent);

  world.registerEntity(player);
  world.registerEntity(new Obstacle(createVector(800, 680)));
  world.registerEntity(new Obstacle(createVector(1200, 680)));
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
    gameState.setRunSpeed = gameState.getRunSpeed.x - 0.1;
    isPlayerHittingAnObstacle(world);
    didWeScore(world);
    if (random() > 0.99) {
      const tree = createTree();
      world.registerEntity(tree);
    }
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
      const obstacles = world.getEntityByType(types.obstacle);

      gameState.running = true;
      player.reset();
      obstacles.forEach((obstacle) => obstacle.initialize());
    },
  };
}

let inputHandler = new InputHandler(generateCommands());
let gameState;
let world;
let jumpOverObstacleEvent;

function isPlayerHittingAnObstacle(world) {
  const [player] = world.getEntityByType(types.player);
  const obstacles = world.getEntityByType(types.obstacle);

  obstacles.forEach((obstacle) => {
    if (
      player.location.x < obstacle.location.x + obstacle.width &&
      player.location.x + player.width > obstacle.location.x &&
      player.location.y < obstacle.location.y + obstacle.height &&
      player.location.y + player.height > obstacle.location.y
    ) {
      player.collidedEvent.notify(player, COLLIDE_WITH_OBSTACLE);
    }
  });
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

function didWeScore(world) {
  const [player] = world.getEntityByType(types.player);
  const obstacles = world.getEntityByType(types.obstacle);

  obstacles.forEach((obstacle) => {
    if (
      obstacle.location.x + obstacle.width / 2 <
        player.location.x + player.width / 2 &&
      !obstacle.jumpedOver
    ) {
      player.incrementScore();
      obstacle.wasJumpedOver();
      increaseSpeed(world);
    }
  });
}

function increaseSpeed(world) {
  world.getEntityByType(types.obstacle).forEach((obstacle) => {
    obstacle.increaseSpeed();
  });
}

function createTree() {
  const treeTypes = ["tree", "tallTree"];
  const treeData = createTreeData(random(treeTypes));
  const green = random(50, 150);
  const treeAlpha = random(1, 10);
  const treeColor = color(0, green, 0, alpha);
  return new Tree(
    width + treeData.trunkWidth + treeData.branchSize,
    height - treeData.trunkHeight,
    treeData,
    treeColor,
    treeAlpha
  );
}
