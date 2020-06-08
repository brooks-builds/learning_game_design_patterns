// class FileSystem {
//   constructor() {
//     this.stored_instance = null;
//   }

//   instance() {
//     // Lazy initialize.
//     if (this.stored_instance == null) this.stored_instance = new FileSystem();
//     return this.stored_instance;
//   }
// }

const PLATFORM = process.env.PLATFORM; // ps3 || wii || ps2
const PLAYSTATION_3 = "ps3";
const WII = "wii";

class FileSystem {
  constructor() {
    this.instance = null;
  }

  readFile(path) {}
  writeFile(path, contents) {}

  instance() {
    if (this.instance) return this.instance;

    if (PLATFORM === PLAYSTATION_3) {
      this.instance = new PS3FileSystem();
    } else if (PLATFORM === WII) {
      // set instance to Wii File System
    }

    return this.instance;
  }
}

class PS3FileSystem extends FileSystem {
  readFile(path) {
    // does the read file stuff
  }

  writeFile(path, contents) {
    // writes to a file
  }
}

// before we refactor away from singleton

class Bullet {
  constructor() {
    this.x;
    this.y;
  }

  getX() {
    return this.x;
  }

  getY() {
    return this.y;
  }

  setX(x) {
    this.x = x;
  }

  setY(y) {
    this.y = y;
  }
}

class BulletManager {
  constructor() {}

  create(x, y) {
    const bullet = new Bullet();
    bullet.setX(x);
    bullet.setY(y);
    return bullet;
  }

  isOnScreen(bullet) {
    // calculate if bullet is on screen, return boolean
  }

  move(bullet) {
    bullet.setX(bullet.getX() + 5);
  }
}

// after refactoring

class Bullet {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }

  isOnScreen() {
    // return true / false if on screen or not
  }

  move() {
    this.x += 5;
  }
}
