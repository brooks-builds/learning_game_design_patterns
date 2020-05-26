class World {
    constructor() {
        this.trees = [];
        this.treeVelocity = createVector(treeData.speedX, treeData.speedY);
        this.treeTrunkColor = color(treeData.trunkRed, treeData.trunkGreen, treeData.trunkBlue);
        this.chanceToCreateTree = 0.02;
    }

    drawTrees() {
        this.trees.forEach(tree => tree.draw(this.treeTrunkColor));
    }

    run() {
        this.drawTrees();
        this.removeTreesOffScreen();
        console.log(this.trees.length);
        this.trees.forEach(tree => tree.update(this.treeVelocity));

        if(random() < this.chanceToCreateTree) this.createTree();
    }

    createTree() {
        const green = random(50, 150);
        const treeAlpha = random(1, 10);
        const treeColor = color(0, green, 0, alpha);
        const trunkHeight = 250;
        const trunkWidth = 20;
        this.trees.push(
            new Tree(width + trunkWidth * treeData.branchSize, height - trunkHeight, trunkWidth, trunkHeight, treeColor, treeAlpha),
        )
    }

    removeTreesOffScreen() {
        for(let treeIndex = this.trees.length - 1; treeIndex >=0; treeIndex -= 1) {
            const tree = this.trees[treeIndex];

            if(tree.isOffScreen()) this.trees.splice(treeIndex, 1);
        }
    }
}