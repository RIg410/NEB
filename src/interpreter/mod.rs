pub mod exec;

use crate::parser::ast::Val;

pub trait Execution {
    fn exec(&self) -> Val;
}

#[cfg(test)]
mod test {
    use crate::interpreter::Execution;
    use crate::parser::ast::{parse_exp, Val};
    use crate::parser::lexer::Lexer;

    fn perform(input: &str, result: Val) {
        let mut lexer = Lexer::new(input);
        lexer.advance().unwrap();
        let exp = parse_exp(&mut lexer).unwrap().exp().unwrap();
        assert_eq!(exp.exec(), result);
    }

    #[test]
    fn test_interpreter() {
        perform("13", Val::Int(13));
        perform("-1", Val::Int(-1));
        perform("-1.0", Val::Float(-1.0));
        perform("13 + 13", Val::Int(13 + 13));
        perform(" 2 * 13 + 13", Val::Int(2 * 13 + 13));
        perform(" 2 * (13 + 13)", Val::Int(2 * (13 + 13)));
        perform(" 2 * (13 + 13) / 2", Val::Int(2 * (13 + 13) / 2));
        perform("(2 + 2) * 10 ^ 2", Val::Int((2 + 2) * 10i64.pow(2)));
        perform("(2 + 2) * 10.0", Val::Float((2 + 2) as f64 * 10.0));
    }
}
