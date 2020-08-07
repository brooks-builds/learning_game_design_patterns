class PlayerPhysics {
  constructor(movedEvent, movedIntoNewCellEvent) {
    this.velocity = createVector(gameData.player.speed, 0);
    this.movedEvent = movedEvent;
    this.moving = true;
    movedIntoNewCellEvent.registerListener(
      this.movedIntoNewCellEvent.bind(this)
    );
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
}

class StaticPhysics {
  update() {}
}
