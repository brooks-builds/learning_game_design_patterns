const standingState = {
  handleInput(command, actor) {
    if (command === "jump") {
      // switch to jump state
      actor.state = new JumpingState(actor);
    }
  },

  update() {},
};
