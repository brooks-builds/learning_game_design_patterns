class PlayerPhysics {
  constructor(movedEvent, movedIntoNewCellEvent, startGameEvent) {
    this.velocity = createVector(gameData.player.speed, 0);
    this.movedEvent = movedEvent;
    this.moving = false;
    movedIntoNewCellEvent.registerListener(
      this.movedIntoNewCellEvent.bind(this)
    );
    startGameEvent.registerListener(this.startGameEvent.bind(this));
  }

  update(location) {
    if (this.moving) {
      const oldX = location.x;
      location.add(this.velocity);
      this.movedEvent.notify(events.playerMoved, location);
    }
  }

  movedIntoNewCellEvent(event, data) {
    const { currentCell } = data;
    console.log(currentCell);

    for (let objectId in currentCell) {
      if (currentCell[objectId].type === "end") this.moving = false;
    }
  }

  startGameEvent(event) {
    this.moving = true;
  }
}

class StaticPhysics {
  update() {}
}
