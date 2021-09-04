use super::tokenizer::Token;
use std::vec::Vec;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply
}

impl Operator {
    fn get_precedence(&self) -> u32 {
        use Operator::{*};
        match *self {
            Plus | Minus => 0,
            Divide | Multiply => 1
        }
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_precedence().cmp(&other.get_precedence()))
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.get_precedence() == other.get_precedence()
    }
}

#[derive(Debug)]
pub enum Node {
    Op(Operator, Box<Node>, Box<Node>),
    Num(u32),
}

#[derive(Debug)]
struct Stack {
    nodes: Vec<Node>,
    ops: Vec<Operator>
}

impl Stack {
    fn new() -> Stack {
        Stack {
            nodes: vec![],
            ops: vec![]
        }
    }

    fn merge_node_set(&mut self) {
        if self.nodes.len() < 2 {
            panic!("Node stack is too small");
        }

        if self.ops.len() < 1 {
            panic!("Ops stack is too small");
        }

        let right = self.nodes.pop().unwrap();
        let op = self.ops.pop().unwrap();
        let left = self.nodes.pop().unwrap();

        self.nodes.push(Node::Op(op, Box::new(left), Box::new(right)));
    }

    fn try_merge_node_set_if<F: Fn(Operator) -> bool>(&mut self, pred: F) {
        if self.ops.len() > 0 && pred(*self.ops.last().unwrap()) {
            self.merge_node_set();
        }
    }

    fn push_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
    
    fn push_op(&mut self, op: Operator) {
        self.ops.push(op);
    }

    fn resolve_all(&mut self) -> Node {
        if self.nodes.len() < 1 {
            panic!("Node stack is empty");
        }

        if  self.nodes.len() == 1 {
            return self.nodes.pop().unwrap()
        }

        while self.nodes.len() > 1 {
            self.merge_node_set()
        }
        self.nodes.pop().unwrap()
    }
}

struct Parser<'a, I: Iterator<Item = &'a Token>> {
    iter: I,
    stack: Stack
}

impl<'a, I: Iterator<Item = &'a Token>> Parser<'a, I> {
    fn new(iter: I) -> Parser<'a, I> {
        Parser {
            iter: iter,
            stack: Stack::new()
        }
    }

    fn validate_token(prev_token: Option<Token>, curr_token: Token) -> Result<(), String> {
        if let Some(prev) = prev_token {
            match prev {
                Token::Plus | Token::Minus | Token::Divide | Token::Multiply => {
                    match curr_token {
                        Token::Number(_) => {},
                        Token::OpenBracket => {},
                        _ => return Err("Expected Number or Open Bracket".to_owned())
                    }
                },
                Token::Number(_) => {
                    match curr_token {
                        Token::Plus | Token::Minus | Token::Divide | Token::Multiply | Token::CloseBracker => {},
                        _ => return Err("Expected Operator or Close Bracket".to_owned())
                    }
                },
                Token::OpenBracket => {
                    match curr_token {
                        Token::Number(_) => {},
                        _ => return Err("Expected Number".to_owned())
                    }
                },
                Token::CloseBracker => {
                    match curr_token {
                        Token::Plus | Token::Minus | Token::Divide | Token::Multiply => {},
                        _ => return Err("Expected Operator".to_owned())
                    }
                }
            }
        }
        return Ok(())
    }

    fn parse(&mut self, inside_brackets: bool) -> Result<Node, String> {
        let mut prev_token: Option<Token> = None;

        loop {
            let v = match self.iter.next() {
                None => None,
                Some(ref v) => Some(*v)
            };

            match v {
                Some(token) => {
                    Parser::<I>::validate_token(prev_token, *token)?;

                    prev_token = Some(*token);
                    match token {
                        Token::Plus => {
                            self.stack.try_merge_node_set_if(|top_op| { Operator::Plus <= top_op });
                            self.stack.push_op(Operator::Plus);
                        },
                        Token::Minus => {
                            self.stack.try_merge_node_set_if(|top_op| { Operator::Minus <= top_op });
                            self.stack.push_op(Operator::Minus);
                        },
                        Token::Divide => {
                            self.stack.try_merge_node_set_if(|top_op| { Operator::Divide <= top_op });
                            self.stack.push_op(Operator::Divide);
                        },
                        Token::Multiply => {
                            self.stack.try_merge_node_set_if(|top_op| { Operator::Multiply <= top_op });
                            self.stack.push_op(Operator::Multiply);
                        },
                        Token::Number(n) => self.stack.push_node(Node::Num(*n)),
                        Token::OpenBracket => {
                            let node = self.parse(true)?;
                            self.stack.push_node(node);
                            prev_token = Some(Token::CloseBracker);
                        },
                        Token::CloseBracker => {
                            if inside_brackets {
                                return Ok(self.stack.resolve_all());
                            }
                            else {
                                return Err("Unexpected close bracket".to_string())
                            }

                        }
                    }
                },
                None => {
                    if inside_brackets {
                        return Err("Unmatched bracket".to_string())
                    }
                    return Ok(self.stack.resolve_all())
                }
            }
        }
    }
}

pub fn parse(input: &[Token]) -> Result<Node, String> {
    let mut parser = Parser::new(input.iter());
    parser.parse(false)
}

#[cfg(test)]
mod test {
    #[test]
    fn do_test() {
        use super::parse;
        use super::tokenize_string;

        let input = tokenize_string("1+6*9/10").unwrap();
        let n = parse(&input).unwrap();
        println!("{:?}", n);
    }
}