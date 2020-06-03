class Monster {}

class Ghost extends Moster {
  constructor(health = 15, speed = 3) {
    this.health = health;
    this.speed = speed;
  }
}
class Demon extends Moster {}
class Sorcerer extends Moster {}

class Spawner {
  constructor(Spawn) {
    this.Spawn = Spawn;
    this.spawnMonster = () => new Spawn();
  }

  //   spawnMonster() {
  //     new this.Spawn();
  //   }
}

// class GhostSpawner extends Spawner {
//   spawnMonster() {
//     return new Ghost();
//   }
// }

// class DemonSpawner extends Spawner {
//   spawnMonster() {
//     return new Demon();
//   }
// }

// in setup

// function spawnGhost() {
//   return new Ghost(15, 3);
// }

// const ghostPrototype = new Ghost(15, 3);
const ghostSpawner = new Spawner(Ghost);

// in loop

ghosts.push(ghostSpawner.spawnMonster());

// prototypal version of Spawner
function Spawner(Spawn) {
  this.Spawn = Spawn;
}

Spawner.prototype.spawnMonster = function () {
  return new this.Spawn();
};

// Object.assign({}, JSON.parse(JSON.stringify(ghosts[0])));
