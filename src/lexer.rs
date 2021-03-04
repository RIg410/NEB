use anyhow::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    EOF,
    IntNumber,
    FloatNumber,
    LParen,
    RParen,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Percent,
}

pub struct Lexer<'input> {
    text: &'input str,
    prev_end: usize,
    cur_start: usize,
    cur_end: usize,
    token: Token,
}

impl<'input> Lexer<'input> {
    pub fn new(text: &'input str) -> Lexer<'input> {
        Lexer {
            text,
            prev_end: 0,
            cur_start: 0,
            cur_end: 0,
            token: Token::EOF,
        }
    }

    pub fn token(&self) -> Token {
        self.token
    }

    pub fn content(&self) -> &str {
        &self.text[self.cur_start..self.cur_end]
    }

    pub fn start_loc(&self) -> usize {
        self.cur_start
    }

    pub fn previous_end_loc(&self) -> usize {
        self.prev_end
    }

    pub fn advance(&mut self) -> Result<(), Error> {
        self.prev_end = self.cur_end;
        let text = self.text[self.cur_end..].trim_start();
        self.cur_start = self.text.len() - text.len();
        let (token, len) = Self::find_token(text)?;
        self.cur_end = self.cur_start + len;
        self.token = token;
        Ok(())
    }

    fn find_token(text: &str) -> Result<(Token, usize), Error> {
        let c: char = match text.chars().next() {
            Some(next_char) => next_char,
            None => {
                return Ok((Token::EOF, 0));
            }
        };

        Ok(match c {
            '0'..='9' => {
                let len = text
                    .chars()
                    .position(|c| match c {
                        '0'..='9' | '.' | ',' => false,
                        _ => true,
                    })
                    .unwrap_or_else(|| text.len());
                let tkn = &text[..len];
                if tkn.contains('.') || tkn.contains(',') {
                    (Token::FloatNumber, len)
                } else {
                    (Token::IntNumber, len)
                }
            }
            '%' => (Token::Percent, 1),
            '(' => (Token::LParen, 1),
            ')' => (Token::RParen, 1),
            '*' => (Token::Star, 1),
            '+' => (Token::Plus, 1),
            '-' => (Token::Minus, 1),
            '/' => (Token::Slash, 1),
            '^' => (Token::Caret, 1),
            _ => return Err(Error::msg(format!("Invalid character: '{}'", c))),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::{Lexer, Token};

    fn perform(tokens: &[(Token, &str)], input: &str) {
        let mut lexer = Lexer::new(input);

        let expected = tokens.iter()
            .map(|(tkn, ct)| (*tkn, ct.to_string()))
            .collect::<Vec<_>>();

        let mut actual = vec![];
        loop {
            lexer.advance().unwrap();

            actual.push((lexer.token(), lexer.content().to_owned()));
            if lexer.token() == Token::EOF {
                break;
            }
        }
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_empty_input() {
        perform(&[(Token::EOF, "")], "");
    }

    #[test]
    #[should_panic(expected = "Invalid character: 'd'")]
    pub fn test_invalid_input() {
        perform(&[(Token::IntNumber, "10"), (Token::Star, "*"), (Token::EOF, "")], "10 * d");
    }

    #[test]
    pub fn test_number() {
        perform(&[(Token::IntNumber, "123123"), (Token::EOF, "")], "123123");
        perform(&[(Token::IntNumber, "0"), (Token::EOF, "")], "0");
        perform(
            &[(Token::FloatNumber, "123.123"), (Token::EOF, "")],
            "123.123",
        );
        perform(&[(Token::FloatNumber, "0,1"), (Token::EOF, "")], "0,1");
    }

    #[test]
    pub fn test_sign() {
        perform(
            &[
                (Token::Plus, "+"),
                (Token::Minus, "-"),
                (Token::Slash, "/"),
                (Token::Star, "*"),
                (Token::Caret, "^"),
                (Token::Percent, "%"),
                (Token::LParen, "("),
                (Token::RParen, ")"),
                (Token::EOF, ""),
            ],
            "+-/*^%()",
        );
    }

    #[test]
    pub fn test_expiration() {
        perform(
            &[
                (Token::LParen, "("),
                (Token::IntNumber, "20"),
                (Token::Plus, "+"),
                (Token::IntNumber, "3"),
                (Token::RParen, ")"),
                (Token::Slash, "/"),
                (Token::IntNumber, "13"),
                (Token::Percent, "%"),
                (Token::LParen, "("),
                (Token::FloatNumber, "10.2"),
                (Token::Star, "*"),
                (Token::FloatNumber, "0,1"),
                (Token::RParen, ")"),
                (Token::Star, "*"),
                (Token::LParen, "("),
                (Token::IntNumber, "10"),
                (Token::Caret, "^"),
                (Token::IntNumber, "1"),
                (Token::RParen, ")"),
                (Token::EOF, ""),
            ],
            "(20 + 3) / 13 % (10.2 * 0,1) * (10 ^ 1)",
        );
    }
}
