class Tree {
  constructor(x, y, data, branchColor) {
    this.trunkColor = color(data.trunkRed, data.trunkGreen, data.trunkBlue);
    this.trunkLocation = createVector(x, y);
    this.trunkWidth = data.trunkWidth;
    this.trunkHeight = data.trunkHeight;
    this.branchColor = branchColor;
    this.branchesSize = data.branchSize;
    this.velocity = createVector(data.speedX, data.speedY);
    console.log(this);
  }

  draw() {
    fill(this.trunkColor);
    noStroke();
    rect(
      this.trunkLocation.x,
      this.trunkLocation.y,
      this.trunkWidth,
      this.trunkHeight
    );
    fill(this.branchColor);
    this.middleOfTrunk = this.trunkLocation.x + this.trunkWidth / 2;
    for (let count = 0; count < 5; count += 1) {
      triangle(
        this.middleOfTrunk, // x1
        this.trunkLocation.y - this.branchesSize + count * 20, // y1
        this.middleOfTrunk - this.branchesSize, // x2
        this.trunkLocation.y + this.branchesSize + count * 20, // y2
        this.middleOfTrunk + this.branchesSize, // x3
        this.trunkLocation.y + this.branchesSize + count * 20 // y3
      );
    }
  }

  update() {
    this.trunkLocation.add(this.velocity);
  }

  isOffScreen() {
    return this.trunkLocation.x + this.trunkWidth + this.branchesSize < 0;
  }

  reset() {}
}
