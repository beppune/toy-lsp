use std::collections::VecDeque;
use std::mem::discriminant;

#[derive(Debug,Clone,PartialEq)]
enum Token {
    WS(char),
    Sign(char),
    Digit(char),
    Alpha(char),
    Dot,
}

fn tokenize(s:&str) -> VecDeque<Token> {

    let mut tokens:VecDeque<Token> = VecDeque::new();

    let it = s.chars();

    for c in it {
        match c {
            '\u{0020}' | '\u{00A}' | '\u{000D}' | '\u{0009}'
            => {
                tokens.push_back( Token::WS(c) );
            },
            '+' | '-' => {
                tokens.push_back( Token::Sign(c) );
            },
            'A'..='Z' | 'a'..='z' => {
                tokens.push_back( Token::Alpha(c) );
            },
            '0'..='9' => {
                tokens.push_back( Token::Digit(c) );
            },
            '.' => {
                tokens.push_back( Token::Dot );
            },
            _ => {}
        }
    }

    tokens
}

struct Parser {
    tokens: VecDeque<Token>
}

impl Parser {

    fn new(tokens:VecDeque<Token>) -> Self {
        Parser {
            tokens
        }
    }

    pub fn matcht(&mut self, t:Token) -> bool {
        if self.tokens.len() == 0 {
            return false;
        }

        let front = self.tokens.front().unwrap();

        if discriminant(front) == discriminant(&t) {
            self.tokens.pop_front();
            return true;
        }

        return false;

    }
}

#[cfg(test)]
mod rpc {
    use super::*;

    #[test]
    fn atest() {
        let tokens = tokenize(" \t\r\n+--+aZ.0");

        for i in &tokens {
            println!("{:?}", i);
        }

        assert!( tokens.len() == 12 );
    }

    #[test]
    fn mat_token_test() {
        let mut parser = Parser::new( tokenize("0") );

        let b = parser.matcht( Token::Digit('9') );
        assert!(b);
    }

}
