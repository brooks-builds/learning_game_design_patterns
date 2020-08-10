function drawInterface(state) {
  textSize(36);
  fill("white");
  displayText[state]();
}

const displayText = {
  [gameData.states.notStarted]() {
    text("Press RETURN to begin", width / 2 - 175, height / 2);
  },
  [gameData.states.playing]() {},
  [gameData.states.won]() {
    text("You Won!!!!!", width / 2 - 75, height / 2);
    this.reset();
  },
  reset() {
    text("Press RETURN to play again", width / 2 - 200, height / 2 + 50);
  },
  [gameData.states.died]() {
    text("Really?", width / 2 - 15, height / 2);
    this.reset();
  },
  [gameData.states.editing]() {
    text("editing mode", 5, 20);
  },
};
