class GameObject {
  constructor(input, physics, graphics) {
    this.velocity = { x: 0, y: 0 };
    this.x = 0;
    this.y = 0;
    this.input = input; // input is new PlayerInput();
    this.physics = physics;
    this.graphics = graphics;
  }

  update(world, graphics) {
    this.input.update(this);
    this.physics.update(this, world);
    this.graphics.update(this, graphics);
  }
}

class InputComponent {
  constructor() {}

  update(bjorn) {
    throw new Error("update must be overwritten");
  }
}

class PlayerInputComponent extends InputComponent {
  constructor() {
    super();
    this.walkAcceleration = 1;
  }
  update(bjorn) {
    const direction = Controller.getControllerDirection();
    if (direction == LEFT) {
      bjorn.velocity.x -= this.walkAcceleration;
    } else if (direction == RIGHT) {
      bjorn.velocity.x += this.walkAcceleration;
    }
  }
}

class DemoInputComponent extends InputComponent {
  constructor() {
    super();
    this.walkAcceleration = 1;
  }

  update(bjorn) {
    // write ai to control bjorn
    bjorn.velocity.x += this.walkAcceleration;
  }
}

class BjornPhysicsComponent {
  constructor() {
    this.volume = 100;
  }
  update(bjorn, world) {
    bjorn.x += bjorn.velocity.x;
    bjorn.y += bjorn.velocity.y;
    world.resolveCollisionDetection(
      this.volume,
      bjorn.x,
      bjorn.y,
      bjorn.velocity
    );
  }
}

class BjornGraphicsComponent {
  constructor(physics) {
    this.spriteStand;
    this.spriteWalkLeft;
    this.spriteWalkRight;
    this.spriteJump;
    this.physics = physics;
  }
  update(bjorn, graphics) {
    if (this.physics.isOnGround()) {
      if (bjorn.velocity.x < 0) {
        graphics.draw(this.spriteWalkLeft, bjorn.x, bjorn.y);
      } else if (bjorn.velocity.x > 0) {
        graphics.draw(this.spriteWalkRight, bjorn.x, bjorn.y);
      } else {
        graphics.draw(this.spriteStand, bjorn.x, bjorn.y);
      }
    } else {
      graphics.draw(this.spriteJump, bjorn.x, bjorn.y);
    }
  }
}

function createBjorn() {
  const bjornPhysics = new BjornPhysicsComponent();
  return new GameObject(
    new PlayerInputComponent(),
    bjornPhysics,
    new BjornGraphicsComponent(bjornPhysics)
  );
}
