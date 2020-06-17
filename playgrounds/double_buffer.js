const WHITE = color(255, 255, 255, 255);
const BLACK = color(0, 0, 0, 255);

class Framebuffer {
  constructor() {
    this.width = 1024;
    this.height = 768;
    this.pixels = new Array(this.width * this.height);
    this.clear();
  }

  clear() {
    for (let index in this.pixels) {
      this.pixels[index] = WHITE;
    }
  }

  draw(x, y) {
    this.pixels[this.width * y + x] = BLACK;
  }

  getPixels() {
    return this.pixels;
  }
}

class Scene {
  constructor() {
    this.buffers = [new Framebuffer(), new Framebuffer()];
    this.current = this.buffers[0];
    this.next = this.buffers[1];
  }

  draw() {
    this.next.clear();
    this.next.draw(1, 1);
    this.next.draw(4, 1);
    this.next.draw(1, 3);
    this.next.draw(2, 4);
    this.next.draw(3, 4);
    this.next.draw(4, 3);
    this.swap();
  }

  getBuffer() {
    return this.current;
  }

  swap() {
    const temp = this.current;
    this.current = this.next;
    this.next = temp;
  }
}

class Actor {
  constructor() {
    this.slapped = [false, false];
  }

  init() {
    this.current = 0;
  }

  update() {}

  slap() {
    this.slapped[this.next()] = true;
  }

  wasSlapped() {
    return this.slapped[this.current];
  }

  swap() {
    this.current = this.next();
  }

  next() {
    return 1 - this.current;
  }
}

class Stage {
  constructor() {
    this.number_of_actors = 3;
    this.actors = new Array(this.number_of_actors);
  }
  add(actor, index) {
    this.actors[index] = actor;
  }

  update() {
    this.actors.forEach((actor) => {
      actor.update();
    });

    this.actors.forEach((actor) => {
      actor.swap();
    });
  }
}

class Comedian extends Actor {
  constructor() {
    super();
    this.facing = null;
  }

  face(actor) {
    this.facing = actor;
  }

  update() {
    if (this.wasSlapped()) this.facing.slap();
  }
}

// in our main loop
const stage = new Stage();

const harry = new Comedian();
const baldy = new Comedian();
const chump = new Comedian();

harry.face(baldy);
baldy.face(chump);
chump.face(harry);

stage.add(harry, 2);
stage.add(baldy, 1);
stage.add(chump, 0);

harry.slap();
stage.update();
