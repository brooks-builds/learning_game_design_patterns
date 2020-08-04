class DrawFloor {
  constructor() {
    this.color = color("brown");
  }

  draw(location, width, height) {
    fill(this.color);
    rect(location.x, location.y, width, height);
  }
}

class DrawStart {
  constructor() {
    this.color = color("yellow");
  }

  draw(location, width, height) {
    fill(this.color);
    rect(location.x, location.y, width, height);
  }
}

class DrawSpike {
  constructor() {
    this.color = color("grey");
  }

  draw(location, width, height) {
    fill(this.color);
    triangle(
      location.x + width / 2,
      location.y,
      location.x + width,
      location.y + height,
      location.x,
      location.y + height
    );
  }
}

class DrawEnd {
  constructor() {
    this.color = color("green");
  }

  draw(location, width, height) {
    fill(this.color);
    rect(location.x, location.y, width, height);
  }
}
