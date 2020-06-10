class GameState {
  constructor() {
    const jumpOverObstacleEvent = new EventSystem();
    const collidedEvent = new EventSystem();

    this.player = new Player(
      createVector(50, 25),
      jumpOverObstacleEvent,
      collidedEvent
    );
    this.obstacles = [];
    this.obstacles.push(
      new Obstacle(createVector(800, 680), jumpOverObstacleEvent),
      new Obstacle(createVector(1200, 680), jumpOverObstacleEvent)
    );
    this.gravity = createVector(0, 1);
    this.isRunning = true;
    this.world = new World();
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

  get getWorld() {
    return this.world;
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
