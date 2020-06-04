class World {
  constructor() {
    this.trees = [];
    this.chanceToCreateTree = 0.02;
  }

  drawTrees() {
    this.trees.forEach((tree) => tree.draw(this.treeTrunkColor));
  }

  run() {
    this.drawTrees();
    this.removeTreesOffScreen();
    this.trees.forEach((tree) => tree.update(this.treeVelocity));

    if (random() < this.chanceToCreateTree) this.createTree();
  }

  createTree() {
    const treeTypes = ["tree", "tallTree"];
    const treeData = createTreeData(random(treeTypes));
    const green = random(50, 150);
    const treeAlpha = random(1, 10);
    const treeColor = color(0, green, 0, alpha);
    const newTree = new Tree(
      width + treeData.trunkWidth + treeData.branchSize,
      height - treeData.trunkHeight,
      treeData,
      treeColor,
      treeAlpha
    );
    if (treeData.type == "tree") {
      this.trees.push(newTree);
    } else {
      this.trees.unshift(newTree);
    }
  }

  removeTreesOffScreen() {
    for (
      let treeIndex = this.trees.length - 1;
      treeIndex >= 0;
      treeIndex -= 1
    ) {
      const tree = this.trees[treeIndex];

      if (tree.isOffScreen()) this.trees.splice(treeIndex, 1);
    }
  }
}
