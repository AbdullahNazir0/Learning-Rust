#![allow(dead_code)]

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Calculator {
    operand1: f64,
    operand2: f64,
    operator: Operation,
}

impl Calculator {
    fn new(oper1: f64, oper2: f64, operator: Operation) -> Calculator {
        Calculator {
            operand1: oper1,
            operand2: oper2,
            operator: operator,
        }
    }
    fn perform_operation(&self) {
        match self.operator {
            Operation::Add => println!(
                "{0} + {1} = {2}",
                self.operand1,
                self.operand2,
                self.operand1 + self.operand2
            ),
            Operation::Subtract => println!(
                "{0} - {1} = {2}",
                self.operand1,
                self.operand2,
                self.operand1 - self.operand2
            ),
            Operation::Multiply => println!(
                "{0} * {1} = {2}",
                self.operand1,
                self.operand2,
                self.operand1 * self.operand2
            ),
            Operation::Divide => println!(
                "{0} / {1} = {2}",
                self.operand1,
                self.operand2,
                self.operand1 / self.operand2
            ),
        }
    }
}

fn main() {
    let calculator1 = Calculator {
        operand1: 2.5,
        operand2: 4.0,
        operator: Operation::Add,
    };

    calculator1.perform_operation();

    let calculator2 = Calculator::new(3.0, 5.0, Operation::Multiply);

    calculator2.perform_operation();
}
