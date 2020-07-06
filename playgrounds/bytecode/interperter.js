class Expression {
  constructor() {}

  evaluate() {
    return 0;
  }
}

class NumberExperssion extends Expression {
  constructor(value) {
    super();
    this.value = value;
  }

  evaluate() {
    return this.value;
  }
}

class AdditionExpression extends Expression {
  constructor(left, right) {
    super();
    this.left = left;
    this.right = right;
  }

  evaluate() {
    const left = this.left.evaluate();
    const right = this.right.evaluate();

    return left + right;
  }
}
