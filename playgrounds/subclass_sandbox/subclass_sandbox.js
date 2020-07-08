class Superpower {
  //   constructor(soundEngine) {
  //     this.soundEngine = soundEngine;
  //   }

  activate() {
    throw new Error("this has not been implented");
  }

  move(x, y, z) {
    // move the player
  }

  playSound(soundId, volume) {
    // play the sound
    this.soundEngine.play(soundId, volume);
  }

  spawnParticles(particleType, count) {
    // display the particles
  }

  getHeroX() {
    // get the x position of the hero
  }

  getHeroY() {
    // get the y position of the hero
  }

  getHeroZ() {
    // get the z position of the hero
  }

  getSoundPlayer() {
    return state.soundPlayer;
  }
}

class SkyLaunch extends Superpower {
  constructor() {
    super();
  }

  activate() {
    if (this.getHeroY() == 0) {
      // standing on the ground
      this.playSound("sproing", 1);
      this.spawnParticles("dust", 200);
      this.move(0, 20, 0);
    } else if (this.getHeroY() < 10) {
      // in the air, but not high up
      this.playSound("swoop", 1);
      this.spawnParticles("cloud", 50);
      this.move(0, this.getHeroY() - 10, 0);
    } else {
      // we are high in the air
      this.playSound("dive", 1);
      this.move(0, 0, 0);
    }
  }
}

const skylaunch = new SkyLaunch();
skylaunch.activate();
