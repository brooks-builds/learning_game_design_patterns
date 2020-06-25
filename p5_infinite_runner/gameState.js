class GameState {
  constructor(jumpOverObstacleEvent, collidedEvent) {
    this.gravity = createVector(0, 1);
    this.isRunning = true;
    this.initializeGameSpeed();
    this.floorHeight = 5;

    collidedEvent.addObserver(this);
  }

  initializeGameSpeed() {
    this.runSpeed = createVector(-5, 0);
  }

  onNotify(entity, event) {
    if (event == COLLIDE_WITH_OBSTACLE) {
      this.running = false;
    }
  }

  get running() {
    return this.isRunning;
  }

  set running(newState) {
    this.isRunning = newState;
  }

  get getPlayer() {
    return this.player;
  }

  get getGravity() {
    return this.gravity;
  }

  get getObstacles() {
    return this.obstacles;
  }

  get getRunSpeed() {
    return this.runSpeed;
  }

  get getFloorHeight() {
    return this.floorHeight;
  }

  set setRunSpeed(newRunSpeed) {
    this.runSpeed.x = newRunSpeed;
  }
}
