let grid;
let nextObjectId;
let camera;

// next time
// stop every static object from moving
// have the camera draw everying on the screen
// update the camera
// move objects around on the grid based on the cameras position

function setup() {
  createCanvas(gameData.screenWidth, gameData.screenHeight);

  camera = new Camera(createVector(0, 0), createVector(1, 0), width, height);
  nextObjectId = 0;
  grid = new Grid(gameData.cellSize);

  gameData.level.forEach((cell, index) => {
    buildLevel[cell](index * gameData.cellSize, gameData.floorY, grid);
  });
}

function draw() {
  background("black");
  grid.drawGrid();
  grid.update();
  grid.draw();
}

const buildLevel = {
  floor(x, y, grid) {
    const floor = new GameObject(
      nextObjectId,
      x,
      y,
      gameData.cellSize,
      gameData.cellSize,
      new DrawFloor()
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
      new DrawStart()
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
      new DrawSpike()
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
      new DrawEnd()
    );
    grid.add(end);
    nextObjectId += 1;
  },
};
