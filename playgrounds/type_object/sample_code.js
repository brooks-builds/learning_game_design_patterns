const breedData = require("./breeds.json");

const LOW_HEALTH = 10;

class Breed {
  constructor(parent = null, health = null, attackString = null) {
    if (health || !parent) {
      this.health = health;
    } else {
      this.health = parent.health;
    }

    if (attackString || !parent) {
      this.attackString = attackString;
    } else {
      this.attackString = parent.attackString;
    }
  }

  newMonster() {
    return new Monster(this);
  }

  get health() {
    return this.health;
  }

  get attackString() {
    return this.attackString;
  }
}

class Monster {
  constructor(breed) {
    this.health = breed.health;
    this.breed = breed;
  }

  get attackString() {
    if (this.health < LOW_HEALTH) {
      return "The monster flails weakly";
    }
    return this.breed.attackString;
  }

  get breed() {
    return this.breed;
  }

  set breed(breed) {
    this.breed = breed;
  }
}

// setup
const dragon = new Breed(248, "The Dragon breathes fire on you!");
const zombieDragon = new Breed(100, "The Zombie Dragon lurches at you");
const sallyDragon = dragon.newMonster();
sallyDragon.breed = zombieDragon;
