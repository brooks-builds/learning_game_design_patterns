class World {
    constructor(trees) {
        this.trees = trees;
        this.treeVelocity = createVector(-1, 0);
    }


    drawTrees() {
        this.trees.forEach(tree => tree.draw());
    }
}