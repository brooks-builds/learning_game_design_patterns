class EventSystem {
  constructor() {
    this.onNotify = (event) => {
      console.log(`${event} is not being listened to right now`);
    };
  }

  notify(event, data) {
    this.onNotify(event, data);
  }

  registerListener(callback) {
    this.onNotify = callback;
  }
}

const events = {
  playerMoved: "playerMoved",
  gameObjectMovedCells: "gameObjectMovedCells",
  startingGame: "starting game",
};
