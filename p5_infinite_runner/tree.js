class Tree {
    constructor(x, y, trunkWidth, trunkHeight, branchColor) {
        this.trunkLocation = createVector(x, y);
        this.trunkWidth = trunkWidth;
        this.trunkHeight = trunkHeight;
        this.branchColor = branchColor;
        this.branchesSize = trunkWidth * 4;
    }
    
    draw(trunkColor) {
        fill(trunkColor);
        noStroke();
        rect(
            this.trunkLocation.x, 
            this.trunkLocation.y,
            this.trunkWidth,
            this.trunkHeight
            );
        fill(this.branchColor);
        this.middleOfTrunk = this.trunkLocation.x + this.trunkWidth / 2;
        for(let count = 0; count < 5; count += 1) {
            triangle(
                this.middleOfTrunk, // x1
                this.trunkLocation.y - this.branchesSize + count * 20, // y1
                this.middleOfTrunk - this.branchesSize,  // x2
                this.trunkLocation.y + this.branchesSize + count * 20, // y2
                this.middleOfTrunk + this.branchesSize, // x3
                this.trunkLocation.y + this.branchesSize + count * 20 // y3
            );
        }
    }

    update(velocity) {
        this.trunkLocation.add(velocity);
    }

    isOffScreen() {
        return this.trunkLocation.x + this.trunkWidth + this.branchesSize < 0;
    }

    reset() {}
}