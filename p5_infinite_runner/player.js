class Player {
  constructor(location, jumpOverObstacleEvent, collidedEvent) {
    this.initialLocation = location.copy();
    this.reset();
    this.width = 5;
    this.height = 20;
    this.jumpOverObstacleEvent = jumpOverObstacleEvent;
    this.collidedEvent = collidedEvent;
    this.state = new JumpingState(this);

    this.jumpOverObstacleEvent.addObserver(this);
  }

  render() {
    fill(0);
    rect(this.location.x, this.location.y, this.width, this.height);
  }

  applyForce(force) {
    this.acceleration.add(force);
  }

  update() {
    this.velocity.add(this.acceleration);
    this.location.add(this.velocity);
    this.acceleration.mult(0);
    this.state.update(this);
  }

  checkIfHitting(obstacle) {
    if (
      this.location.x < obstacle.location.x + obstacle.width &&
      this.location.x + this.width > obstacle.location.x &&
      this.location.y < obstacle.location.y + obstacle.height &&
      this.location.y + this.height > obstacle.location.y
    ) {
      this.collidedEvent.notify(this, COLLIDE_WITH_OBSTACLE);
    }
  }

  incrementScore() {
    this.score += 1;
  }

  reset() {
    this.score = 0;
    this.location = this.initialLocation.copy();
    this.velocity = createVector(0, 0);
    this.acceleration = createVector(0, 0);
  }

  onNotify(entity, event) {
    if (event == EVENT_JUMPED_OVER_OBSTACLE) {
      entity.incrementScore();
    }
  }

  handleInput(command) {
    this.state.handleInput(command, this);
  }
}
