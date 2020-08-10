class Grid {
  constructor(
    cellSize = 100,
    worldHorizontileCount = 1000,
    worldVerticalCount = 4,
    gameObjectMovedIntoNewCellEvent,
    gameObjectMovedOutOfGrid
  ) {
    this.cellHeight = cellSize;
    this.cellWidth = cellSize;
    this.cells = [];
    this.gameObjectMovedIntoNewCellEvent = gameObjectMovedIntoNewCellEvent;
    this.gameObjectMovedOutOfGrid = gameObjectMovedOutOfGrid;
    this.gameObjectsOffGrid = [];

    for (let yCount = 0; yCount < worldVerticalCount; yCount += 1) {
      const yCells = [];
      for (let xCount = 0; xCount < worldHorizontileCount; xCount += 1) {
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
    if (yIndex >= this.verticalCount || xIndex >= this.horizontileCount) return;
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
    this.cells.forEach((xCells) => {
      xCells.forEach((gameObjects) => {
        for (const gameObjectId in gameObjects) {
          this.handleUpdate(gameObjects[gameObjectId]);
        }
      });
    });
  }

  handleUpdate(gameObject) {
    const currentIndexX = Math.floor(gameObject.location.x / this.cellWidth);
    const currentIndexY = Math.floor(gameObject.location.y / this.cellHeight);
    let nearbyGameObjects = [];
    nearbyGameObjects = nearbyGameObjects.concat(
      ...this.putGameObjectsInArray(
        this.getGameObjectsInCell(currentIndexX, currentIndexY)
      ),
      ...this.putGameObjectsInArray(
        this.getGameObjectsInCell(currentIndexX, currentIndexY + 1)
      ),
      ...this.putGameObjectsInArray(
        this.getGameObjectsInCell(currentIndexX + 1, currentIndexY + 1)
      )
    );

    gameObject.update(nearbyGameObjects);

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

    if (this.cells[currentIndexY][currentIndexX]) {
      delete this.cells[currentIndexY][currentIndexX][gameObject.id];
    }

    this.add(gameObject);
    const gameObjectsInCell = this.getGameObjectsInCell(nextIndexX, nextIndexY);
    if (!gameObjectsInCell) {
      this.gameObjectMovedOutOfGrid.notify(gameObject);
      this.gameObjectsOffGrid.push(gameObject);
    } else {
      this.gameObjectMovedIntoNewCellEvent.notify({
        currentCell: gameObjectsInCell,
      });
    }
  }

  getGameObjectsInCell(x, y) {
    if (x >= this.horizontileCount || y >= this.verticalCount) {
      return false;
    }
    return this.cells[y][x];
  }

  putGameObjectsInArray(cell) {
    const result = [];

    for (let id in cell) {
      result.push(cell[id]);
    }

    return result;
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

  removeGameObjectsByType(type) {
    const results = [];

    this.cells.forEach((row) => {
      row.forEach((cell) => {
        for (let objectId in cell) {
          if (cell[objectId].type === type) {
            results.push(cell[objectId]);
            delete cell[objectId];
          }
        }
      });
    });

    this.gameObjectsOffGrid.forEach((gameObject, index) => {
      if (gameObject.type === type) {
        results.push(gameObject);
        this.gameObjectsOffGrid[index] = null;
      }
    });

    this.gameObjectsOffGrid = this.gameObjectsOffGrid.filter(
      (gameObject) => gameObject !== null
    );

    return results;
  }

  handleEditCell(worldX, worldY) {
    const gridCoordinates = this.convertWorldCoordinatesToGridCoordinates(
      worldX,
      worldY
    );

    if (this.isClickingOnEnd(gridCoordinates.x, gridCoordinates.y)) {
      const cell = this.getGameObjectsInCell(
        gridCoordinates.x,
        gridCoordinates.y
      );
      let floor;
      for (let cellIndex in cell) {
        const gameObject = cell[cellIndex];
        if (gameObject.type === gameData.types.floor) {
          floor = gameObject.clone();
          break;
        }
      }
      if (!floor) return;

      this.insertEmptyCells(
        gridCoordinates.x,
        gameData.cloneAmountWhileEditing
      );
      for (
        let count = 0;
        count <= gameData.cloneAmountWhileEditing;
        count += 1
      ) {
        this.add(floor);
        floor = floor.clone();
        floor.location.x += gameData.cellSize;
      }
      nextObjectId += 1;
    } else {
      const gameObjects = this.getGameObjectsInCell(
        gridCoordinates.x,
        gridCoordinates.y
      );

      for (let gameObjectId in gameObjects) {
        const gameObject = gameObjects[gameObjectId];
        if (gameObject.type === gameData.types.floor) {
          gameObject.type = gameData.types.space;
          gameObject.drawModule = new DrawSpace();
        } else if (gameObject.type === gameData.types.space) {
          gameObject.type = gameData.types.floor;
          gameObject.drawModule = new DrawFloor();
        }
      }
    }
  }

  isClickingOnEnd(gridX, gridY) {
    const gameObjectsAbove = this.getGameObjectsInCell(gridX, gridY - 1);
    for (let gameObjectId in gameObjectsAbove) {
      const gameObject = gameObjectsAbove[gameObjectId];
      if (gameObject.type === gameData.types.end) {
        return true;
      }
    }

    return false;
  }

  convertWorldCoordinatesToGridCoordinates(worldX, worldY) {
    return {
      x: Math.floor(worldX / this.cellWidth),
      y: Math.floor(worldY / this.cellHeight),
    };
  }

  insertEmptyCells(gridX, countToInsert) {
    const cellsToInsert = [];
    for (let count = 0; count < countToInsert; count += 1) {
      cellsToInsert.push({});
    }
    this.cells.forEach((row) => {
      row.splice(gridX, 0, ...cellsToInsert);
    });
    this.shiftXCells(gridX, countToInsert);
  }

  shiftXCells(startX, shiftBy = 1) {
    this.cells.forEach((row) => {
      for (let index = startX; index < row.length; index += 1) {
        const cell = row[index];
        for (let objectId in cell) {
          const gameObject = cell[objectId];
          delete cell[objectId];
          gameObject.location.x += gameData.cellSize * shiftBy;
          this.add(gameObject);
        }
      }
    });
  }

  get horizontileCount() {
    return this.cells[0].length;
  }

  get verticalCount() {
    return this.cells.length;
  }
}
