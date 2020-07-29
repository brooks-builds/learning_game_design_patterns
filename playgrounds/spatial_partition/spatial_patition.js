class Unit {
  constructor(grid, x, y) {
    this.grid = grid;
    this.x = x; // 250.5px
    this.y = y; // 300.3px
    this.previous = null;
    this.next = null;
  }

  move(x, y) {
    this.grid.move(this, x, y);
  }
}

class Grid {
  constructor() {
    this.cellCount = 10;
    this.cellSize = 20;
    this.cells = new Array(this.cellCount);
    for (let index = 0; index < this.cells.length; index += 1) {
      this.cells[index] = new Array(this.cellCount);
    }

    for (let yIndex = 0; yIndex < this.cellCount; yIndex += 1) {
      for (let xIndex = 0; xIndex < this.cellCount; xIndex += 1) {
        this.cells[yIndex][xIndex] = null;
      }
    }
  }

  add(unit) {
    const xIndex = Math.floor(unit.x / this.cellSize);
    const yIndex = Math.floor(unit.y / this.cellSize);
    unit.previous = null;
    unit.next = this.cells[yIndex][xIndex];
    this.cells[yIndex][xIndex] = unit;

    if (unit.next != null) {
      unit.next.previous = unit;
    }
  }

  handleMelee() {
    for (let yIndex = 0; yIndex < this.cellCount; yIndex += 1) {
      for (let xIndex = 0; xIndex < this.cellCount; xIndex += 1) {
        this.handleCell(xIndex, yIndex);
      }
    }
  }

  handleCell(x, y) {
    const unit = this.cells[y][x];
    while (unit) {
      if (unit.next) {
        this.handleUnit(unit, unit.next);
      }
      unit = unit.next;
    }
  }

  handleUnit(unit, other) {
    while (other) {
      // handle collision
      const distance = utility.distance(unit, other);
      if (distance < ATTACK_DISTANCE) {
        this.handleAttack(unit, other);
      }
      other = other.next;
    }
  }

  handleAttack(firstUnit, secondUnit) {}

  move(unit, x, y) {
    const oldYIndex = Math.floor(unit.y / this.cellSize);
    const oldXIndex = Math.floor(unit.x / this.cellSize);
    const yIndex = Math.floor(x / this.cellSize);
    const xIndex = Math.floor(y / this.cellSize);
    unit.x = x;
    unit.y = y;

    if (oldXIndex == xIndex && oldYIndex == yIndex) {
      return;
    }

    if (unit.previous) {
      unit.previous.next = unit.next;
    }

    if (unit.next) {
      unit.next.previous = unit.previous;
    }

    if (this.cells[oldYIndex][oldXIndex] == unit) {
      this.cells[oldYIndex][oldXIndex] = unit.next;
    }

    this.add(unit);
  }
}
