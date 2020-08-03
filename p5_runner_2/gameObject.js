class GameObject {
  constructor(
    x,
    y,
    width = gameData.cellSize,
    height = gameData.cellSize,
    drawModule
  ) {
    this.location = createVector(x, y);
    this.width = width;
    this.height = height;
    this.drawModule = drawModule;
  }

  draw() {
    this.drawModule.draw(this.location, this.width, this.height);
  }

  get centerLocation() {
    return createVector(
      this.location.x + this.width / 2,
      this.location.y + this.height / 2
    );
  }
}
