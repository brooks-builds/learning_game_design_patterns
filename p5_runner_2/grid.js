class Grid {
  constructor(cellSize = 100) {
    this.horizontileCount = Math.floor(width / cellSize);
    this.verticalCount = Math.floor(height / cellSize);
    this.cellWidth = cellSize;
    this.cellHeight = cellSize;
    this.cells = [];
    this.offScreenLeft = {};
    this.offScreenRight = {};
    this.farOffScreenLeft = {};

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
    this.cells.forEach((xCells, yIndex) => {
      line(0, yIndex * this.cellHeight, width, yIndex * this.cellHeight);
      xCells.forEach((cell, xIndex) => {
        line(xIndex * this.cellWidth, 0, xIndex * this.cellWidth, height);
      });
    });
  }

  add(gameObject) {
    const xIndex = Math.floor(gameObject.location.x / this.cellWidth);
    const yIndex = Math.floor(gameObject.location.y / this.cellHeight);
    if (xIndex === -1) {
      this.offScreenLeft[gameObject.id] = gameObject;
    } else if (xIndex < -1) {
      this.farOffScreenLeft[gameObject.id] = gameObject;
    } else if (xIndex >= this.cells[0].length) {
      this.offScreenRight[gameObject.id] = gameObject;
    } else {
      this.cells[yIndex][xIndex][gameObject.id] = gameObject;
    }
  }

  draw() {
    this.cells.forEach((xCells) => {
      xCells.forEach((gameObjects) => {
        for (const gameObjectId in gameObjects) {
          gameObjects[gameObjectId].draw();
        }
      });
    });
    for (const gameObjectId in this.offScreenLeft) {
      this.offScreenLeft[gameObjectId].draw();
    }
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
