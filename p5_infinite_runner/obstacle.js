class Obstacle extends Entity {
  constructor(location) {
    super(types.obstacle, drawPlane.foreground);
    this.initialLocation = location.copy();
    this.width = 15;
    this.height = 15;
    this.increaseSpeedBy = 0.1;
    this.velocity = createVector(-5, 0);
    this.initialVelocity = this.velocity.copy();
    this.jumpedOver = false;
    this.initialize();
  }

  render() {
    fill(0);
    triangle(
      this.location.x,
      this.location.y,
      this.location.x + this.width,
      this.location.y,
      this.location.x + this.width / 2,
      this.location.y + this.height
    );
  }

  update() {
    this.location.add(this.velocity);
    this.reset();
  }

  reset() {
    if (this.location.x + this.width < 0) {
      this.location.x = width + 5;
      this.jumpedOver = false;
    }
  }

  initialize() {
    this.location = this.initialLocation.copy();
    this.velocity = this.initialVelocity.copy();
  }

  isRightOfPlayer(player) {
    return this.location.x > player.location.x + player.width / 2;
  }

  isLeftOfPlayer(player) {
    return this.location.x < player.location.x + player.width / 2;
  }

  wasJumpedOver() {
    this.jumpedOver = true;
  }

  increaseSpeed() {
    this.velocity.x -= 0.5;
  }
}
