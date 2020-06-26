class Entity {
  constructor(type, drawPlane) {
    this.type = type;
    this.drawPlane = drawPlane;
  }
}

const types = {
  player: "player",
  obstacle: "obstacle",
  tree: "tree",
};
