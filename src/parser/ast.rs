use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::Range;

use anyhow::{anyhow, Error};

use crate::parser::lexer::{Lexer, Token};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Exp {
    Val(Val),
    Exp {
        op: Op,
        left: Box<Exp>,
        right: Box<Exp>,
    },
}

impl Display for Exp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Exp::Val(val) => val.fmt(f),
            Exp::Exp { op, left, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
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

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Mod => '%',
            Op::Div => '/',
            Op::Row => '^',
        })
    }
}

impl Op {
    pub fn order(&self) -> u8 {
        match self {
            Op::Add => 3,
            Op::Sub => 3,
            Op::Mul => 2,
            Op::Mod => 2,
            Op::Div => 2,
            Op::Row => 1,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Val {
    Int(i128),
    Float(f64),
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Val::Int(val) => val.fmt(f),
            Val::Float(val) => val.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Sequence {
    Exp(Exp),
    Op(Op),
    Operand(Val),
}

impl Display for Sequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Sequence::Exp(exp) => exp.fmt(f),
            Sequence::Op(op) => op.fmt(f),
            Sequence::Operand(op) => op.fmt(f),
        }
    }
}

impl Sequence {
    pub fn op(&self) -> Option<Op> {
        match self {
            Sequence::Op(op) => Some(*op),
            _ => None,
        }
    }

    pub fn val(&self) -> Option<Val> {
        match self {
            Sequence::Operand(val) => Some(*val),
            _ => None,
        }
    }

    pub fn exp(self) -> Option<Exp> {
        match self {
            Sequence::Exp(exp) => Some(exp),
            Sequence::Op(_) => None,
            Sequence::Operand(val) => Some(Exp::Val(val)),
        }
    }
}


pub fn parse_exp(lexer: &mut Lexer) -> Result<Sequence, Error> {
    let mut last: Option<Token> = None;
    let mut seq = vec![];
    loop {
        match lexer.token() {
            Token::LParen => {
                if let Some(last) = &last {
                    if !last.is_sign() {
                        return Err(anyhow!("Unexpected '(' token. Position: {}", lexer.loc()));
                    }
                }

                lexer.advance()?;
                seq.push(Some(parse_exp(lexer)?));
                if lexer.token() != Token::RParen {
                    return Err(anyhow!(
                            "Unexpected end of input. Expected ')' token. Position: {}",
                            lexer.loc()
                        ));
                }
            }
            Token::EOF => {
                if let Some(last) = &last {
                    if !(last.is_number() || *last == Token::EOF || *last == Token::RParen) {
                        return Err(anyhow!(
                            "Unexpected end of input. Position: {}",
                            lexer.loc()
                        ));
                    }
                }
                break;
            }
            Token::RParen => {
                break;
            }
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

                seq.push(Some(Sequence::Operand(parse_number(false, lexer)?)));
            }
            Token::Plus => {
                if let Some(last) = &last {
                    if last.is_sign() {
                        return Err(anyhow!("Unexpected '+' token. Position: {}", lexer.loc()));
                    }
                }
                seq.push(Some(Sequence::Op(Op::Add)));
            }
            Token::Minus => {
                if let Some(last) = &last {
                    if last.is_sign() {
                        lexer.advance()?;
                        seq.push(Some(Sequence::Operand(parse_number(true, lexer)?)));
                    } else {
                        seq.push(Some(Sequence::Op(Op::Sub)));
                    }
                } else {
                    lexer.advance()?;
                    seq.push(Some(Sequence::Operand(parse_number(true, lexer)?)));
                }
            }
            Token::Star => {
                if let Some(last) = &last {
                    if last.is_sign() {
                        return Err(anyhow!("Unexpected '*' token. Position: {}", lexer.loc()));
                    }
                }
                seq.push(Some(Sequence::Op(Op::Mul)));
            }
            Token::Slash => {
                if let Some(last) = &last {
                    if last.is_sign() {
                        return Err(anyhow!("Unexpected '/' token. Position: {}", lexer.loc()));
                    }
                }
                seq.push(Some(Sequence::Op(Op::Div)));
            }
            Token::Caret => {
                if let Some(last) = &last {
                    if last.is_sign() {
                        return Err(anyhow!("Unexpected '^' token. Position: {}", lexer.loc()));
                    }
                }
                seq.push(Some(Sequence::Op(Op::Row)));
            }
            Token::Percent => {
                if let Some(last) = &last {
                    if last.is_sign() {
                        return Err(anyhow!("Unexpected '%' token. Position: {}", lexer.loc()));
                    }
                }
                seq.push(Some(Sequence::Op(Op::Mod)));
            }
        }

        last = Some(lexer.token());
        lexer.advance()?;
    }

    if seq.is_empty() {
        return Err(anyhow!("Empty expression"));
    }

    if seq.len() == 1 {
        Ok(seq.remove(0).ok_or_else(|| anyhow!("Empty expression"))?)
    } else {
        Ok(Sequence::Exp(make_exp(seq)?))
    }
}

fn parse_number(negative: bool, lexer: &mut Lexer) -> Result<Val, Error> {
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

fn make_exp(mut seq: Vec<Option<Sequence>>) -> Result<Exp, Error> {
    let mut operator_order = seq.iter()
        .flatten()
        .enumerate()
        .filter_map(|(i, sq)| {
            if let Some(op) = sq.op() {
                Some((op.order(), i, op))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    operator_order.sort_by(|(l_order, l_index, _), (r_order, r_index, _)| {
        (*l_order, *l_index).cmp(&(*r_order, *r_index)
        )
    });

    let mut buffer: Vec<(Range<usize>, Exp)> = Vec::new();

    fn find_in_buffer(buffer: &mut Vec<(Range<usize>, Exp)>, index: usize) -> Option<(Range<usize>, Exp)> {
        if let Some(index) = buffer.iter().enumerate()
            .find_map(|(i, (range, _))| {
                if range.start <= index && range.end >= index {
                    Some(i)
                } else {
                    None
                }
            }) {
            Some(buffer.remove(index))
        } else {
            None
        }
    }

    fn find_exp(buffer: &mut Vec<(Range<usize>, Exp)>, seq: &mut Vec<Option<Sequence>>, index: usize) -> Result<(Range<usize>, Exp), Error> {
        find_in_buffer(buffer, index)
            .or_else(||
                seq[index].take()
                    .and_then(|sq| sq.exp())
                    .map(|ex| (((index..index), ex)))
            ).ok_or_else(|| anyhow!("Invalid expiration"))
    }

    for (_, index, operator) in operator_order {
        // index may not be 0.
        let (l_range, l_exp) = find_exp(&mut buffer, &mut seq, index - 1)?;
        let (r_range, r_exp) = find_exp(&mut buffer, &mut seq, index + 1)?;

        let exp = Exp::Exp {
            op: operator,
            left: Box::new(l_exp),
            right: Box::new(r_exp),
        };

        buffer.push(((l_range.start..r_range.end), exp))
    }

    if buffer.len() != 1 {
        return Err(anyhow!("Invalid expiration"));
    }

    Ok(buffer.remove(0).1)
}

#[cfg(test)]
mod test {
    use crate::parser::ast::parse_exp;
    use crate::parser::lexer::Lexer;

    fn perform_test(input: &str, ir_foot_print: &str) {
        let mut lexer = Lexer::new(input);
        lexer.advance().unwrap();
        assert_eq!(parse_exp(&mut lexer).unwrap().to_string(), ir_foot_print);
    }

    #[test]
    fn test_exp() {
        perform_test("10 * 1 + -2 / 10,1 % 0.1 ^ 1", "((10 * 1) + ((-2 / 10.1) % (0.1 ^ 1)))");
        perform_test("13", "13");
        perform_test("10 - 1", "(10 - 1)");
        perform_test("-10 +  -1", "(-10 + -1)");
        perform_test("10 - (1 * 2)", "(10 - (1 * 2))");
        perform_test("(1 * 2) + 10", "((1 * 2) + 10)");
        perform_test("10", "10");
        perform_test("(1 + 2) * 10 * (3 - 10^2) ", "(((1 + 2) * 10) * (3 - (10 ^ 2)))");
    }
}
