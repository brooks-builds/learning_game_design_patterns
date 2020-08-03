let grid;

function setup() {
  createCanvas(gameData.screenWidth, gameData.screenHeight);

  grid = new Grid(gameData.cellSize);
}

function draw() {
  background("black");
  grid.drawGrid();
}
