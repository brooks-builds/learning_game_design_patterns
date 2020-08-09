class PlayerPhysics {
  constructor(
    movedEvent,
    movedIntoNewCellEvent,
    startGameEvent,
    playerWonEvent,
    playerDiedEvent
  ) {
    this.velocity = createVector(gameData.player.speed, 0);
    this.movedEvent = movedEvent;
    this.moving = false;
    movedIntoNewCellEvent.registerListener(
      this.movedIntoNewCellEvent.bind(this)
    );
    startGameEvent.registerListener(this.startGameEvent.bind(this));
    this.playerWonEvent = playerWonEvent;
    this.playerDiedEvent = playerDiedEvent;
  }

  update(location) {
    if (this.moving) {
      const oldX = location.x;
      location.add(this.velocity);
      this.movedEvent.notify(location);
    }
  }

  movedIntoNewCellEvent(data) {
    const { currentCell } = data;
    for (let objectId in currentCell) {
      if (currentCell[objectId].type === gameData.types.end) {
        this.moving = false;
        this.playerWonEvent.notify();
      } else if (currentCell[objectId].type === gameData.types.spikeUp) {
        this.playerDiedEvent.notify();
        this.moving = false;
      }
    }
  }

  startGameEvent() {
    this.moving = true;
  }
}

class StaticPhysics {
  update() {}
}
