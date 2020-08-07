class PlayerPhysics {
  constructor(movedEvent) {
    this.velocity = createVector(gameData.player.speed, 0);
    this.movedEvent = movedEvent;
    this.moving = true;
  }

  update(location) {
    const oldX = location.x;
    location.add(this.velocity);
    this.movedEvent.notify(events.playerMoved, location);
  }
}

class StaticPhysics {
  update() {}
}
