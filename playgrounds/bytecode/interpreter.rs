trait Expression {
    fn evaluate(&self) -> i32;
}

struct NumberExpression {
    value: i32
}

impl Expression for NumberExpression {
    fn evaluate(&self) -> i32 {
        self.value
    }
}

struct AdditionExpression {
    left: NumberExpression,
    right: NumberExpression,
}

impl AdditionExpression {
    pub fn new(left: NumberExpression, right: NumberExpression) -> AdditionExpression {
        AdditionExpression {
            left: left,
            right: right,
        }
    }
}

impl Expression for AdditionExpression {
    fn evaluate(&self) -> i32 {
        let left = self.left.evaluate();
        let right = self.right.evaluate();

        left + right
    }
}