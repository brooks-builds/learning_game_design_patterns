let grid;
let nextObjectId;
let camera;
let state;
let startGameEvent;
let resetGameEvent;
let playerMovedEvent;
let gravityForce;
let jumpEvent;
const SPACE = 32;

const commands = {
  jump: "jump",
};

function setup() {
  createCanvas(gameData.cameraWidth, gameData.cameraHeight);

  state = gameData.states.notStarted;
  const gameObjectMovedIntoNewCellEvent = new EventSystem();
  const playerWonEvent = new EventSystem();
  const playerDiedEvent = new EventSystem();
  const gameObjectMovedOutOfGrid = new EventSystem();
  playerMovedEvent = new EventSystem();
  resetGameEvent = new EventSystem();
  startGameEvent = new EventSystem();
  jumpEvent = new EventSystem();

  gravityForce = createVector(0, gameData.gravityForce);

  startGameEvent.registerListener(() => (state = gameData.states.playing));
  playerWonEvent.registerListener(() => (state = gameData.states.won));
  resetGameEvent.registerListener(resetGame);
  playerDiedEvent.registerListener(() => (state = gameData.states.died));

  grid = new Grid(
    gameData.cellSize,
    gameData.level.length,
    gameData.worldHeight,
    gameObjectMovedIntoNewCellEvent,
    gameObjectMovedOutOfGrid
  );
  nextObjectId = 0;
  const player = new GameObject(
    nextObjectId,
    gameData.player.startX,
    gameData.player.startY,
    gameData.player.bodyWidth,
    gameData.player.bodyHeight,
    new DrawPlayer(),
    gameData.types.player,
    new PlayerPhysics(
      playerMovedEvent,
      gameObjectMovedIntoNewCellEvent,
      startGameEvent,
      playerWonEvent,
      playerDiedEvent,
      gameObjectMovedOutOfGrid,
      nextObjectId,
      jumpEvent
    )
  );
  nextObjectId += 1;
  grid.add(player);
  camera = new Camera(0, 0, width, height, playerMovedEvent);

  gameData.level.forEach((cell, index) => {
    buildLevel[cell](index * gameData.cellSize, gameData.floorY, grid);
  });
}

function draw() {
  background("black");
  // grid.drawGrid();
  if (state === gameData.states.playing) {
    grid.update();
  }
  camera.update();
  camera.draw(grid);

  drawInterface(state);
}

function keyPressed() {
  if (keyCode === ENTER && state === gameData.states.notStarted) {
    startGameEvent.notify();
  } else if (
    keyCode === ENTER &&
    (state === gameData.states.won || state === gameData.states.died)
  ) {
    resetGameEvent.notify();
  }

  if (keyCode === SPACE && state === gameData.states.playing) {
    jumpEvent.notify();
  }
}

const buildLevel = {
  floor(x, y, grid) {
    const floor = new GameObject(
      nextObjectId,
      x,
      y,
      gameData.cellSize,
      gameData.cellSize,
      new DrawFloor(),
      gameData.types.floor
    );
    grid.add(floor);
    nextObjectId += 1;
  },

  start(x, y, grid) {
    this.floor(x, y, grid);
    const start = new GameObject(
      nextObjectId,
      x + gameData.cellSize - 5,
      y - gameData.cellSize,
      5,
      gameData.cellSize,
      new DrawStart(),
      gameData.types.start
    );
    grid.add(start);
    nextObjectId += 1;
  },

  spikeUp(x, y, grid) {
    this.floor(x, y, grid);
    const spike = new GameObject(
      nextObjectId,
      x,
      y - gameData.cellSize,
      gameData.cellSize,
      gameData.cellSize,
      new DrawSpike(),
      gameData.types.spikeUp
    );
    grid.add(spike);
    nextObjectId += 1;
  },

  space() {},

  end(x, y, grid) {
    this.floor(x, y, grid);
    const end = new GameObject(
      nextObjectId,
      x,
      y - gameData.cellSize,
      5,
      gameData.cellSize,
      new DrawEnd(),
      gameData.types.end
    );
    grid.add(end);
    nextObjectId += 1;
  },
};

function resetGame() {
  state = gameData.states.notStarted;
  let player = grid.removeGameObjectsByType(gameData.types.player)[0];
  player.location.x = gameData.player.startX;
  player.location.y = gameData.player.startY;
  player.physics.velocity = createVector(gameData.player.speed, 0);
  grid.add(player);
  playerMovedEvent.notify(player.location);
}
