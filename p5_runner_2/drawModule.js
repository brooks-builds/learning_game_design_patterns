class DrawFloor {
  constructor() {
    this.color = color("brown");
  }

  draw(location, width, height) {
    fill(this.color);
    rect(location.x, location.y, width, height);
  }
}
