class PlayerPhysics {
  constructor(
    movedEvent,
    movedIntoNewCellEvent,
    startGameEvent,
    playerWonEvent,
    playerDiedEvent,
    gameObjectMovedOutOfGridEvent,
    playerId,
    jumpEvent
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
    gameObjectMovedOutOfGridEvent.registerListener(
      this.gameObjectMovedOutOfGridEvent.bind(this)
    );
    this.playerId = playerId;
    jumpEvent.registerListener(this.handleJumpEvent.bind(this));
    this.isJumping = false;
  }

  update(location, nearbyGameObjects = [], playerWidth, playerHeight) {
    if (this.moving) {
      const oldLocation = location.copy();
      location.add(this.velocity);
      nearbyGameObjects.forEach((gameObject) => {
        if (gameObject.type !== gameData.types.floor) return;
        if (
          this.isCollidingWith(location, playerWidth, playerHeight, gameObject)
        ) {
          if (oldLocation.y + playerHeight <= gameObject.location.y) {
            location.y = gameObject.location.y - playerHeight;
            this.velocity.y = 0;
            this.isJumping = false;
          } else {
            location.x = gameObject.location.x - playerWidth;
            this.velocity.x = 0;
          }
        }
      });
      this.movedEvent.notify(location);
    }
  }

  isCollidingWith(ourLocation, playerWidth, playerHeight, otherGameObject) {
    const myLeft = ourLocation.x;
    const myRight = ourLocation.x + playerWidth;
    const myTop = ourLocation.y;
    const myBottom = ourLocation.y + playerHeight;
    const otherRight = otherGameObject.location.x + otherGameObject.width;
    const otherLeft = otherGameObject.location.x;
    const otherTop = otherGameObject.location.y;
    const otherBottom = otherGameObject.location.y + otherGameObject.height;
    return (
      myLeft < otherRight &&
      myRight > otherLeft &&
      myTop < otherBottom &&
      myBottom > otherTop
    );
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

  applyForce(force) {
    this.velocity.add(force);
  }

  gameObjectMovedOutOfGridEvent(gameObject) {
    if (gameObject.id === this.playerId) {
      this.playerDiedEvent.notify();
    }
  }

  handleJumpEvent() {
    if (!this.isJumping) {
      this.velocity.y -= gameData.player.jumpForce;
      this.isJumping = true;
    }
  }
}

class StaticPhysics {
  update() {}

  applyForce() {}
}
