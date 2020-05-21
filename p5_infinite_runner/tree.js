class Tree {
    constructor(x, y, width, height, color) {
        this.trunkLocation = createVector(x, y);
        this.width = width;
        this.height = height;
        this.color = color;
    }

    draw() {
        fill(this.color);
        rect(
            this.trunkLocation.x, 
            this.trunkLocation.y,
            this.width,
            this.height
            );
    }

    update() {}

    isOffScreen() {}

    reset() {}
}