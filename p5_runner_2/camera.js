class Camera {
  constructor(x = 0, y = 0, width, height, playerMovedEvent) {
    this.location = createVector(x, y);
    this.width = width;
    this.height = height;
    playerMovedEvent.registerListener(this.onNotifyPlayerMoved.bind(this));
  }

  draw(grid, editMode = false) {
    const gameObjects = grid.getGameObjectsInRange(
      this.location.x,
      this.location.y,
      this.width + gameData.cellSize,
      this.height
    );

    push();
    translate(-this.location.x, this.location.y);
    gameObjects.forEach((gameObject) => gameObject.draw(editMode));
    pop();
  }

  update() {}

  onNotifyPlayerMoved(playerLocation) {
    this.location.x = playerLocation.x - gameData.cameraChaseX;
  }
}
