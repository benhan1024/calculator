use super::parser::{Node, Operator};

fn traverse(node: &Node) -> f32 {
    use Node::{*};

    let (op, left, right) = match node {
        Op(op, left, right) => (op, left, right),
        Num(n) => return *n as f32
    };

    use Operator::{*};

    let left = traverse(left);
    let right = traverse(right);

    match op {
        Plus => left + right,
        Minus => left - right,
        Divide => left / right,
        Multiply => left * right
    }
}

pub fn calculate(node: &Node) -> f32 {
    traverse(node)
}