class GameObject {
  constructor(
    id,
    x,
    y,
    width = gameData.cellSize,
    height = gameData.cellSize,
    drawModule,
    type,
    physicsModule = new StaticPhysics()
  ) {
    this.location = createVector(x, y);
    this.width = width;
    this.height = height;
    this.drawModule = drawModule;
    this.id = id;
    this.physics = physicsModule;
    this.type = type;
  }

  draw() {
    this.drawModule.draw(this.location, this.width, this.height);
  }

  update(nearbyGameObjects) {
    this.physics.update(
      this.location,
      nearbyGameObjects,
      this.width,
      this.height
    );
    this.physics.applyForce(gravityForce);
  }
}
