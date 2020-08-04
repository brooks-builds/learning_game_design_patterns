let grid;

function setup() {
  createCanvas(gameData.screenWidth, gameData.screenHeight);

  grid = new Grid(gameData.cellSize);

  gameData.level.forEach((cell, index) => {
    buildLevel[cell](index * gameData.cellSize, gameData.floorY, grid);
  });
}

function draw() {
  background("black");
  grid.drawGrid();
  grid.draw();
}

const buildLevel = {
  floor(x, y, grid) {
    const floor = new GameObject(
      x,
      y,
      gameData.cellSize,
      gameData.cellSize,
      new DrawFloor()
    );
    grid.add(floor);
  },

  start(x, y, grid) {
    this.floor(x, y, grid);
    const start = new GameObject(
      x + gameData.cellSize - 5,
      y - gameData.cellSize,
      5,
      gameData.cellSize,
      new DrawStart()
    );
    grid.add(start);
  },

  spikeUp(x, y, grid) {
    this.floor(x, y, grid);
    const spike = new GameObject(
      x,
      y - gameData.cellSize,
      gameData.cellSize,
      gameData.cellSize,
      new DrawSpike()
    );
    grid.add(spike);
  },

  space() {},

  end(x, y, grid) {
    this.floor(x, y, grid);
    const end = new GameObject(
      x,
      y - gameData.cellSize,
      5,
      gameData.cellSize,
      new DrawEnd()
    );
    grid.add(end);
  },
};
