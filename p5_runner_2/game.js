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
let editMode;

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
  editMode = false;

  gravityForce = createVector(0, gameData.gravityForce);

  startGameEvent.registerListener(() => (state = gameData.states.playing));
  playerWonEvent.registerListener(() => (state = gameData.states.won));
  resetGameEvent.registerListener(resetGame);
  playerDiedEvent.registerListener(() => (state = gameData.states.died));

  const level = [
    gameData.types.floor,
    gameData.types.floor,
    gameData.types.start,
    ...gameData.level,
    gameData.types.end,
    gameData.types.floor,
    gameData.types.floor,
  ];
  grid = new Grid(
    gameData.cellSize,
    level.length,
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
  level.forEach((cell, index) => {
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
  camera.draw(grid, editMode);

  drawInterface(state);

  if (editMode && keyIsDown(gameData.commands.editingCameraMoveLeft)) {
    camera.location.x -= gameData.cellSize / 4;
  } else if (editMode && keyIsDown(gameData.commands.editingCameraMoveRight)) {
    camera.location.x += gameData.cellSize / 4;
  }
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

  if (keyCode === gameData.commands.toggleEditing) {
    editMode = !editMode;
    if (editMode) {
      state = gameData.states.editing;
    } else {
      resetGame();
    }
  }
}

function mouseClicked() {
  if (editMode) {
    grid.handleEditCell(mouseX + camera.location.x, mouseY);
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

  space(x, y, grid) {
    const space = new GameObject(
      nextObjectId,
      x,
      y,
      gameData.cellSize,
      gameData.cellSize,
      new DrawSpace(),
      gameData.types.space
    );
    grid.add(space);
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
