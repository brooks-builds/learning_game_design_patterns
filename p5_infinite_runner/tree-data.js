const tree = {
  speedX: -1,
  speedY: 0,
  branchSize: 75,
  trunkRed: 166,
  trunkGreen: 137,
  trunkBlue: 124,
  trunkHeight: 250,
  trunkWidth: 20,
  type: "tree",
};

// // we can have different trees defined each with a prototype pointing towards treeData
const tallTree = {
  prototype: "tree",
  trunkHeight: 400,
  speedX: -0.5,
  type: "tallTree",
};

const treeTypes = {
  tallTree,
  tree,
};

function createTreeData(type) {
  // const newTree = Object.assign({}, treeTypes[type]);
  const treeData = Object.assign(
    {},
    treeTypes[treeTypes[type].prototype],
    treeTypes[type]
  );
  return treeData;
}
