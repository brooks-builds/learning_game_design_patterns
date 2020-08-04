let grid;

function setup() {
  createCanvas(gameData.screenWidth, gameData.screenHeight);

  grid = new Grid(gameData.cellSize);

  gameData.level.forEach((cell, index) => {
    if (cell === "floor") {
      const floorObject = new GameObject(
        index * gameData.cellSize,
        gameData.floorY,
        gameData.cellSize,
        gameData.cellSize,
        new DrawFloor()
      );
      grid.add(floorObject);
    } else if (cell === "start") {
      const floorObject = new GameObject(
        index * gameData.cellSize,
        gameData.floorY,
        gameData.cellSize,
        gameData.cellSize,
        new DrawFloor()
      );
      const startObject = new GameObject(
        index * gameData.cellSize + gameData.cellSize - 5,
        gameData.floorY - gameData.cellSize,
        5,
        gameData.cellSize,
        new DrawStart()
      );
      grid.add(floorObject);
      grid.add(startObject);
    } else if (cell == "spikeUp") {
      const floorObject = new GameObject(
        index * gameData.cellSize,
        gameData.floorY,
        gameData.cellSize,
        gameData.cellSize,
        new DrawFloor()
      );
      const spikeObject = new GameObject(
        index * gameData.cellSize,
        gameData.floorY - gameData.cellSize,
        gameData.cellSize,
        gameData.cellSize,
        new DrawSpike()
      );
      grid.add(floorObject);
      grid.add(spikeObject);
    }
  });
}

function draw() {
  background("black");
  grid.drawGrid();
  grid.draw();
}
