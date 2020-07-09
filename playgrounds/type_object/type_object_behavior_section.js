class Monster {
  constructor() {
    this.health = 0;
  }

  getAttack() {
    return "I am attacking you";
  }

  Monster(startingHealth) {
    this.health = startingHealth;
  }
}

class Dragon extends Monster {
  constructor() {
    super();
    this.Monster(230);
  }

  getAttack() {
    return "The Dragon breathes fire!";
  }
}

class Troll extends Monster {
  constructor() {
    super();
    this.Monster(48);
  }

  getAttack() {
    return "The Troll clubs you!";
  }
}
