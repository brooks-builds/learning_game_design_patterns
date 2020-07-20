let gameOver = false;

class GameEntity {
  constructor(ai, physics, render) {
    this.ai = ai;
    this.physics = physics;
    this.render = render;
  }

  ai() {
    return this.ai;
  }

  physics() {
    return this.physics;
  }

  render() {
    return this.render;
  }
}

class AiComponent {
  constructor(
    animation,
    energy,
    goalPosition,
    loot,
    minDrops,
    maxDrops,
    chanceOfDrop
  ) {
    this.animation = animation;
    this.energy = energy;
    this.goalPosition = goalPosition;
    this.loot = new LootDrop(loot, minDrops, maxDrops, chanceOfDrop);
  }
  update() {}
}

class LootDrop {
  constructor(loot, minDrops, maxDrops, chanceOfDrop) {
    this.loot = loot;
    this.minDrops = minDrops;
    this.maxDrops = maxDrops;
    this.chanceOfDrop = chanceOfDrop;
  }
}

class PhysicsComponent {
  update() {}
}

class RenderComponent {
  update() {}
}

class ParticleSystem {
  constructor() {
    this.numParticles = 0;
    this.maxParticles = 100000;
    this.particles = new Array(this.maxParticles);
    this.numActive = 0;
  }

  update() {
    for (let index = 0; index < this.numActive; index += 1) {
      this.particles[index].update();
    }
  }

  activateParticle(index) {
    if (index < this.numActive)
      throw new Error("Attempting to activate active particle");

    const temp = this.particles[this.numActive];
    this.particles[this.numActive] = this.particles[index];
    this.particles[index] = temp;
    this.numActive += 1;
  }

  deactivateParticle(index) {
    if (index >= this.numActive) {
      throw new Error("Attempting to deactivate a deactivated particle");
    }

    this.numActive -= 1;
    const temp = this.particles[this.numActive];
    this.particles[this.numActive] = this.particles[index];
    this.particles[index] = temp;
  }
}

// setup
// const entities = [
//   new GameEntity(
//     new AiComponent(),
//     new PhysicsComponent(),
//     new RenderComponent()
//   ),
//   new GameEntity(
//     new AiComponent(),
//     new PhysicsComponent(),
//     new RenderComponent()
//   ),
//   new GameEntity(
//     new AiComponent(),
//     new PhysicsComponent(),
//     new RenderComponent()
//   ),
// ];
const aiComponents = [new AiComponent(), new AiComponent(), new AiComponent()];
const physicsComponents = [
  new PhysicsComponent(),
  new PhysicsComponent(),
  new PhysicsComponent(),
];
const renderComponents = [
  new RenderComponent(),
  new RenderComponent(),
  new RenderComponent(),
];

if (
  aiComponents.length !== physicsComponents.length &&
  aiComponents.length !== renderComponents.length
) {
  throw new Error("We must have the same number of components");
}

// update loop
while (!gameOver) {
  for (let index = 0; index < aiComponents.length; index += 1) {
    // entities[index].ai().update();
    aiComponents[index].update();
  }

  for (let index = 0; index < aiComponents.length; index += 1) {
    // entities[index].physics().update();
    physicsComponents[index].update();
  }

  for (let index = 0; index < aiComponents.length; index += 1) {
    // entities[index].render().update();
    renderComponents[index].update();
  }
}

// player index 0
// ai index 2
const locations = [120, 15, 300, 25];
