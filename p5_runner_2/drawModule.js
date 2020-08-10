class DrawElement {
  draw(location, width, height, editMode = false) {
    if (editMode) {
      stroke("white");
    }
    fill(this.color);
    rect(location.x, location.y, width, height);
  }
}

class DrawFloor extends DrawElement {
  constructor(editingMode = false) {
    super();
    this.color = color("brown");
  }
}

class DrawSpace {
  draw() {}
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

class DrawPlayer {
  constructor() {
    this.color = color("white");
  }

  draw(location, width, height) {
    fill(this.color);
    ellipse(
      location.x + width / 2,
      location.y - gameData.player.headSize / 2,
      gameData.player.headSize
    );
    rect(location.x, location.y, width, height);
  }
}
