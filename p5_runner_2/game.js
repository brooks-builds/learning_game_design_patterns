let grid;
let floorObject;

function setup() {
  createCanvas(gameData.screenWidth, gameData.screenHeight);

  floorObject = new GameObject(
    50,
    250,
    gameData.cellSize,
    gameData.cellSize,
    new DrawFloor()
  );
  grid = new Grid(gameData.cellSize);
  grid.add(floorObject);
}

function draw() {
  background("black");
  grid.drawGrid();
  grid.draw();
}
