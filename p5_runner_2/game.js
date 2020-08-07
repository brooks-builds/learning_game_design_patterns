let grid;
let nextObjectId;
let camera;
let state;
let startGameEvent;

function setup() {
  createCanvas(gameData.cameraWidth, gameData.cameraHeight);

  state = gameState.notStarted;
  const playerMovedEvent = new EventSystem();
  const gameObjectMovedIntoNewCellEvent = new EventSystem();
  startGameEvent = new EventSystem();

  grid = new Grid(
    gameData.cellSize,
    gameData.level.length,
    gameData.worldHeight,
    gameObjectMovedIntoNewCellEvent
  );
  nextObjectId = 0;
  const player = new GameObject(
    nextObjectId,
    gameData.player.startX,
    gameData.player.startY,
    gameData.player.bodyWidth,
    gameData.player.bodyHeight,
    new DrawPlayer(),
    "player",
    new PlayerPhysics(
      playerMovedEvent,
      gameObjectMovedIntoNewCellEvent,
      startGameEvent
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
  grid.update();
  camera.update();
  camera.draw(grid);
  textSize(36);
  fill("white");
  text("Press RETURN to begin", width / 2 - 175, height / 2);
}

function keyPressed() {
  if (keyCode === ENTER && state === gameState.notStarted) {
    startGameEvent.notify(events.startingGame);
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
      "floor"
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
      "start"
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
      "spike"
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
      "end"
    );
    grid.add(end);
    nextObjectId += 1;
  },
};

const gameState = {
  notStarted: "not started",
  playing: "playing",
  won: "won",
  died: "died",
};
