use anyhow::{anyhow, Error};

use crate::parser::lexer::{Lexer, Token};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Exp {
    Value(bool, Val),
    BinaryOp(Op, Box<Exp>, Box<Exp>),
    CastToInt(Box<Exp>),
    CastToFloat(Box<Exp>),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Op {
    // +
    Add,
    // -
    Sub,
    // *
    Mul,
    // %
    Mod,
    // /
    Div,
    // ^
    Row,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Val {
    Int(i128),
    Float(f64),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Sequence {
    Seq(Vec<Sequence>),
    Sign(Op),
    Operand(Val),
}

pub fn parse_exp(lexer: &mut Lexer) -> Result<Sequence, Error> {
    let first = lexer.token();

    if first == Token::LParen {
        lexer.advance()?;
    }
    let mut last: Option<Token> = None;

    let mut seq = vec![];
    loop {
        match lexer.token() {
            Token::IntNumber | Token::FloatNumber => {
                if let Some(last) = &last {
                    if !last.is_sign() {
                        return Err(anyhow!(
                            "Unexpected number '{}' token. Position: {}",
                            lexer.content(),
                            lexer.loc()
                        ));
                    }
                }

                seq.push(Sequence::Operand(parse_number(false, lexer)?));
            }
            Token::LParen => {
                if let Some(last) = &last {
                    if !last.is_sign() {
                        return Err(anyhow!("Unexpected '(' token. Position: {}", lexer.loc()));
                    }
                }
                seq.push(parse_exp(lexer)?);
            }
            Token::Plus => {
                if let Some(last) = &last {
                    if last.is_sign() {
                        return Err(anyhow!("Unexpected '+' token. Position: {}", lexer.loc()));
                    }
                }
                seq.push(Sequence::Sign(Op::Add));
            }
            Token::Minus => {
                if let Some(last) = &last {
                    if last.is_sign() {
                        lexer.advance()?;
                        seq.push(Sequence::Operand(parse_number(true, lexer)?));
                    } else {
                        seq.push(Sequence::Sign(Op::Sub));
                    }
                } else {
                    seq.push(Sequence::Sign(Op::Sub));
                }
            }
            Token::Star => {
                if let Some(last) = &last {
                    if last.is_sign() {
                        return Err(anyhow!("Unexpected '*' token. Position: {}", lexer.loc()));
                    }
                }
                seq.push(Sequence::Sign(Op::Mul));
            }
            Token::Slash => {
                if let Some(last) = &last {
                    if last.is_sign() {
                        return Err(anyhow!("Unexpected '/' token. Position: {}", lexer.loc()));
                    }
                }
                seq.push(Sequence::Sign(Op::Div));
            }
            Token::Caret => {
                if let Some(last) = &last {
                    if last.is_sign() {
                        return Err(anyhow!("Unexpected '^' token. Position: {}", lexer.loc()));
                    }
                }
                seq.push(Sequence::Sign(Op::Row));
            }
            Token::Percent => {
                if let Some(last) = &last {
                    if last.is_sign() {
                        return Err(anyhow!("Unexpected '%' token. Position: {}", lexer.loc()));
                    }
                }
                seq.push(Sequence::Sign(Op::Mod));
            }
            Token::EOF => {
                if first == Token::LParen {
                    return Err(anyhow!(
                        "Unexpected end of input. Expected ')' token. Position: {}",
                        lexer.loc()
                    ));
                } else {
                    break;
                }
            }
            Token::RParen => {
                if first == Token::LParen {
                    lexer.advance()?;
                    break;
                } else {
                    return Err(anyhow!("Unexpected ')' token. Position: {}", lexer.loc()));
                }
            }
        }

        last = Some(lexer.token());
        lexer.advance()?;
    }

    Ok(Sequence::Seq(seq))
}

pub fn parse_number(negative: bool, lexer: &mut Lexer) -> Result<Val, Error> {
    match lexer.token() {
        Token::IntNumber => {
            let mut val = lexer.content().parse().map_err(|err| {
                anyhow!("{:?}. '{}' Position: {}", err, lexer.content(), lexer.loc())
            })?;
            if negative {
                val *= -1;
            }
            Ok(Val::Int(val))
        }
        Token::FloatNumber => {
            let mut val = lexer.content().replace(",", ".").parse().map_err(|err| {
                anyhow!("{:?}. '{}' Position: {}", err, lexer.content(), lexer.loc())
            })?;
            if negative {
                val *= -1.0;
            }
            Ok(Val::Float(val))
        }
        _ => Err(anyhow!(
            "Invalid number. '{}' Position: {}",
            lexer.content(),
            lexer.loc()
        )),
    }
}

#[cfg(test)]
mod test {
    use crate::parser::ast::parse_exp;
    use crate::parser::lexer::Lexer;

    #[test]
    fn test_exp() {
        let mut lexer = Lexer::new("1 + -2 / (10,1 % 0.1)");
        lexer.advance().unwrap();
        dbg!(parse_exp(&mut lexer).unwrap());
    }
}
