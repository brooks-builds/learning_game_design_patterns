class JumpingState {
  constructor(actor) {
    const jumpForce = createVector(0, -10);
    actor.applyForce(jumpForce);
  }

  handleInput(_command, _actor) {}

  update(actor) {
    if (actor.location.y + actor.height > height - gameState.getFloorHeight) {
      actor.location.y = height - actor.height - gameState.getFloorHeight;
      actor.velocity.y = 0;
      actor.state = standingState;
    } else {
      actor.applyForce(gameState.getGravity);
    }
  }
}
