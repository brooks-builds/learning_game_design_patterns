let counter = 0;

console.log(counter++);
console.log(counter);
console.log(++counter);
console.log(counter);

const newEntities = [];
const entities = [];

// main loop
while (true) {
  entities.forEach((entity) => {
    entity.update();
    newEntities.push(new Entity());
  });

  entity = entities.concat(newEntities);
}

for (let index = entities.length - 1; index >= 0; index -= 1) {
  const needsToDie = entities[index].update();
  if (needsToDie) {
    entities.splice(index, 1);
    index -= 1;
  }
}

class Entity {
  constructor() {
    this.x = 0;
    this.y = 0;
  }

  update() {}

  get x() {
    return this.x;
  }

  get y() {
    return this.y;
  }

  set x(x) {
    this.x = x;
  }

  set y(y) {
    this.y = y;
  }
}

class World {
  constructor() {
    this.numberOfEntities = 0;
    this.entities = [];
  }

  gameLoop() {
    while (true) {
      this.entities.forEach((entity) => entity.update);
    }
  }
}

class Skeleton extends Entity {
  constructor() {
    super();
    this.patrollingLeft = false;
  }

  update(elapsed) {
    if (this.patrollingLeft) {
      this.x = this.x - elapsed;
      if (this.x == 0) this.patrollingLeft = false;
    } else {
      this.x = this.x + elapsed;
      if (this.x == 100) this.patrollingLeft = true;
    }
  }
}

class Statue extends Entity {
  constructor(delay = 100) {
    super();
    this.frames = 0;
    this.delay = delay;
  }

  update() {
    this.frames += 1;
    if (this.frames == this.delay) {
      this.shootLightning();
      this.frames = 0;
    }
  }

  shootLightning() {}
}
