class Player extends Entity {
  constructor(location, collidedEvent) {
    super(types.player, drawPlane.foreground);
    this.initialLocation = location.copy();
    this.reset();
    this.width = 5;
    this.height = 20;
    this.collidedEvent = collidedEvent;
    this.state = new JumpingState(this);
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

  incrementScore() {
    this.score += 1;
  }

  reset() {
    this.score = 0;
    this.location = this.initialLocation.copy();
    this.velocity = createVector(0, 0);
    this.acceleration = createVector(0, 0);
  }

  handleInput(command) {
    this.state.handleInput(command, this);
  }
}
