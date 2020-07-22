const MAX_CHILDREN = 5;

class Transform {
  constructor() {
    this.x = 0;
    this.y = 0;
    this.rotation = 0;
  }
  // I'm not sure what this origin function is really doing
  // {x: 0, y:0, rotation: 0}
  origin() {
    return new Transform();
  }

  combine(otherTransform) {
    // other is {x: 0, y:0, rotation: 0}
    this.x *= otherTransform.x;
    this.y *= otherTransform.y;
    this.rotation *= otherTransform.rotation;
  }
}

class GraphNode {
  constructor(mesh = null) {
    this.mesh = mesh;
    this.local = Transform.origin();
    this.children = new Array(MAX_CHILDREN);
    this.numChildren = 0;
    this.dirty = true;
    this.world = null;
  }

  render(parentWorld, dirty) {
    // first pass parentWorld is {x: 0, y:0, rotation: 0}, dirty is undefined
    // dirty = (dirty | _dirty) == 1; From Twitch@ootsby
    // dirty |= this.dirty;
    dirty = dirty || this.dirty;
    // dirty now is true;
    if (dirty) {
      // we are dirty at this point
      this.world = this.local.combine(parentWorld);
      this.dirty = false;
    }
    if (this.mesh) renderMesh(this.mesh, this.world);

    for (let index = 0; index < this.numChildren; index += 1) {
      this.children[index].render(this.world, dirty);
    }
  }

  setTransform(local) {
    this.local = local;
    this.dirty = true;
  }
}

let graph;
function setup() {
  graph = new GraphNode();
}

function draw() {
  graph.render(Transform.origin());
}

function renderMesh(mesh, transform) {}
