use std::vec::Vec;

#[derive(Debug, Copy, Clone)]
pub enum Token
{
    Plus,
    Minus,
    Divide,
    Multiply,

    OpenBracket,
    CloseBracker,

    Number(u32),
}

fn remove_whitespaces(input: &mut String) {
    input.retain(|c| !c.is_whitespace());
}

pub fn tokenize_string(input: &str) -> Result<Vec<Token>, String> {
    let mut input = input.to_string();
    remove_whitespaces(&mut input);

    enum TokenizerState {
        None,
        Number(String)
    }

    let mut state = TokenizerState::None;
    let mut result = Vec::<Token>::new();

    for c in input.chars() {
        if c.is_digit(10) {
            match state {
                TokenizerState::None => state = TokenizerState::Number(c.to_string()),
                TokenizerState::Number(ref mut s) => s.push(c)
            }
        }
        else {
            if let TokenizerState::Number(number_str) = state {
                result.push(Token::Number(number_str.parse().unwrap()));
                state = TokenizerState::None;
            }

            result.push(match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '/' => Token::Divide,
                '*' => Token::Multiply,

                '(' => Token::OpenBracket,
                ')' => Token::CloseBracker,
                _ => return Err("Invalid input".to_string())
            });
        }
    }

    if let TokenizerState::Number(ref mut s) = state {
        result.push(Token::Number(s.parse().unwrap()));
    }

    Ok(result)
}

#[cfg(Test)]
mod Test {
    #[test]
    fn parse()
    {
        use TokenizerState::{*};
        assert_eq!(tokenize_string(" 1 +2   * 4 / 7"), vec![Number(1), Plus, Number(2), Multiply, Number(4), Divide, Number(7)]);
        assert_eq!(tokenize_string("5   -   ( 6 /   9   )"), vec![Number(5), Minus, OpenBracket, Number(6), Divide, Number(9), CloseBracker]);
    }
}