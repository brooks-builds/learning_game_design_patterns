const RELEASE_DOWN = "&145";

class HeroineState {
  constructor() {}

  handleInput(heroine, input) {
    if (input == PRESS_DOWN) {
      return new DuckingState();
    }

    return null;
  }

  update(heroine) {}

  static standing = "standing";
  static ducking = "ducking";
  static jumping = "jumping";
  static diving = "diving";
}

class OnGroundState extends HeroineState {
  static handleInput(heroine, input) {
    if (input == PRESS_B) {
      //jump
    } else if (input == PRESS_DOWN) {
      // duck
    }
  }
}

class DuckingState extends OnGroundState {
  constructor() {
    this.chargeTime = 0;
  }

  handleInput(heroine, input) {
    if (input == RELEASE_DOWN) {
      return new StandingState();
    } else {
      OnGroundState.handleInput(heroine, input);
    }
  }

  update(heroine) {
    this.chargeTime += 1;
    if (this.chargeTime > MAX_CHARGE) {
      heroine.superBomb();
    }
  }
}

class StandingState extends HeroineState {
  constructor() {}

  handleInput(heroine, input) {
    if (input == PRESS_B) {
      heroine.state = HeroineState.jumping;
    }
  }

  update(heroine) {
    this.velocityY = JUMP_VELOCITY;
  }

  enter(heroine) {
    heroine.setGraphics(IMAGE_JUMP);
  }
}

class Heroine {
  constructor() {
    this.state = new StandingState();
    this.equipment = new Gun();
  }

  handleInput(input) {
    // const state = this.state.handleInput(input);

    // if (state != null) {
    //   this.state = state;
    //   this.state.enter(this);
    // }

    this.state.handleInput(input);
    this.equipment.handleInput(input);
  }

  update() {
    this.state.update(this);
  }
}

class jumpingState {
  constructor(heroine) {
    // apply initial up velocity to heroine
  }

  handleInput(heroine, input) {}

  update(heroine) {
    if (heroine.isOnGround()) {
      return new StandingState();
    } else {
      heroine.applyForce(GRAVITY);
    }
  }
}
