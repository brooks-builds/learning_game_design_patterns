class Grid {
  constructor(
    cellSize = 100,
    worldHorizontileCount = 1000,
    worldVerticalCount = 4
  ) {
    this.horizontileCount = worldHorizontileCount;
    this.verticalCount = worldVerticalCount;
    this.cellHeight = cellSize;
    this.cellWidth = cellSize;
    this.cells = [];

    for (let yCount = 0; yCount < this.verticalCount; yCount += 1) {
      const yCells = [];
      for (let xCount = 0; xCount < this.horizontileCount; xCount += 1) {
        yCells.push({});
      }
      this.cells.push(yCells);
    }
    this.lineColor = color(255, 255, 255, 100);
  }

  drawGrid() {
    stroke(this.lineColor);
    noFill();
    this.cells.forEach((xCells, yIndex) => {
      xCells.forEach((cell, xIndex) => {
        rect(
          xIndex * this.cellWidth,
          yIndex * this.cellHeight,
          this.cellWidth,
          this.cellHeight
        );
      });
    });
  }

  add(gameObject) {
    const xIndex = Math.floor(gameObject.location.x / this.cellWidth);
    const yIndex = Math.floor(gameObject.location.y / this.cellHeight);
    this.cells[yIndex][xIndex][gameObject.id] = gameObject;
  }

  getGameObjectsInRange(x, y, width, height) {
    let indexX = Math.floor(x / this.cellWidth);
    let indexY = Math.floor(y / this.cellHeight);
    let xCount = Math.floor(width / this.cellWidth);
    let yCount = Math.floor(height / this.cellHeight);
    const gameObjects = [];

    for (
      let cellIndexY = indexY;
      cellIndexY < indexY + yCount;
      cellIndexY += 1
    ) {
      for (
        let cellIndexX = indexX;
        cellIndexX < indexX + xCount;
        cellIndexX += 1
      ) {
        for (let gameObjectId in this.cells[cellIndexY][cellIndexX]) {
          gameObjects.push(this.cells[cellIndexY][cellIndexX][gameObjectId]);
        }
      }
    }

    return gameObjects;
  }

  update() {
    for (const gameObjectId in this.offScreenLeft) {
      this.handleUpdate(this.offScreenLeft[gameObjectId]);
    }
    this.cells.forEach((xCells) => {
      xCells.forEach((gameObjects) => {
        for (const gameObjectId in gameObjects) {
          this.handleUpdate(gameObjects[gameObjectId]);
        }
      });
    });
    for (const gameObjectId in this.offScreenRight) {
      this.handleUpdate(this.offScreenRight[gameObjectId]);
    }
    for (const gameObjectId in this.farOffScreenLeft) {
      this.handleUpdate(this.farOffScreenLeft[gameObjectId]);
    }
  }

  handleUpdate(gameObject) {
    const currentIndexX = Math.floor(gameObject.location.x / this.cellWidth);
    const currentIndexY = Math.floor(gameObject.location.y / this.cellHeight);
    gameObject.update();
    const nextIndexX = Math.floor(gameObject.location.x / this.cellWidth);
    const nextIndexY = Math.floor(gameObject.location.y / this.cellHeight);

    if (
      this.isInSameGridLocation(
        currentIndexX,
        currentIndexY,
        nextIndexX,
        nextIndexY
      )
    )
      return;

    if (currentIndexX >= this.cells[0].length) {
      delete this.offScreenRight[gameObject.id];
    } else if (currentIndexX === -1) {
      delete this.offScreenLeft[gameObject.id];
    } else if (currentIndexX < -1) {
      delete this.farOffScreenLeft[gameObject.id];
    } else {
      if (this.cells[currentIndexY][currentIndexX])
        delete this.cells[currentIndexY][currentIndexX][gameObject.id];
    }

    this.add(gameObject);
  }

  isInSameGridLocation(currentIndexX, currentIndexY, nextIndexX, nextIndexY) {
    if (
      currentIndexX >= this.cells[0].length &&
      nextIndexX >= this.cells[0].length
    )
      return true;

    if (currentIndexX === nextIndexX && currentIndexY === nextIndexY)
      return true;

    if (currentIndexX === -1 && nextIndexX === -1) return true;

    if (currentIndexX < -1 && nextIndexX < -1) return true;

    return false;
  }
}
