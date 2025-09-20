use std::{cmp::Ordering, collections::HashMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MathExpression(pub String);

impl MathExpression {
    pub fn eval(&self, vars: &HashMap<String, f64>) -> Result<f64, MathExpressionParseError> {
        let raw = parse::tokenize_raw(&self.0)?;
        let raw = parse::process_raw_tokens(&raw)?;
        let raw = parse::tokens_to_expr(&raw)?;
        let raw = parse::add_implicit_multi(raw);
        let raw = parse::expr_to_postfix(&raw);
        let term = parse::postfix_expr_to_term(&raw)?;

        term.eval(vars)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

impl Op {
    const fn precedence(self) -> u8 {
        match self {
            Self::Add | Self::Subtract => 1,
            Self::Multiply | Self::Divide => 2,
            Self::Power => 3,
        }
    }

    const fn is_left_associative(self) -> bool {
        !matches!(self, Self::Power)
    }

    const fn should_shunt(self, other: Self) -> bool {
        (other.precedence() > self.precedence())
            || (other.precedence() == self.precedence() && other.is_left_associative())
    }
}

#[derive(Debug, Clone)]
enum Func {
    Abs(Term),
    Log2(Term),
    Sign(Term),
    Max(Vec<Term>),
    Min(Vec<Term>),
}

impl Func {
    fn eval(&self, vars: &HashMap<String, f64>) -> Result<f64, MathExpressionParseError> {
        match self {
            Self::Abs(term) => Ok(term.eval(vars)?.abs()),
            Self::Log2(term) => Ok(term.eval(vars)?.log2()),
            Self::Sign(term) => {
                let val = term.eval(vars)?;
                if val.abs() < f64::EPSILON {
                    Ok(0.0)
                } else {
                    Ok(val.signum())
                }
            }
            Self::Max(terms) => {
                let mut max = f64::MIN;
                for term in terms {
                    let val = term.eval(vars)?;
                    if val.total_cmp(&max) == Ordering::Greater {
                        max = val;
                    }
                }

                Ok(max)
            }
            Self::Min(terms) => {
                let mut min = f64::MAX;
                for term in terms {
                    let val = term.eval(vars)?;
                    if val.total_cmp(&min) == Ordering::Less {
                        min = val;
                    }
                }

                Ok(min)
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Term {
    Num(f64),
    Var(String),
    Func(Box<Func>),
    Op(Box<Self>, Op, Box<Self>),
}

impl Term {
    fn eval(&self, vars: &HashMap<String, f64>) -> Result<f64, MathExpressionParseError> {
        match self {
            Self::Func(func) => func.eval(vars),
            Self::Num(n) => Ok(*n),
            Self::Var(v) => vars
                .get(v)
                .copied()
                .ok_or_else(|| MathExpressionParseError::UnknownVariable(v.clone())),
            Self::Op(left, op, right) => {
                let left = left.eval(vars)?;
                let right = right.eval(vars)?;

                let res = match op {
                    Op::Add => left + right,
                    Op::Subtract => left - right,
                    Op::Multiply => left * right,
                    Op::Divide => left / right,
                    Op::Power => left.powf(right),
                };

                Ok(res)
            }
        }
    }
}

pub use parse::ParseError as MathExpressionParseError;
mod parse {
    use regex::Regex;

    use super::{Func, Op, Term};

    #[derive(Debug, Clone)]
    pub(super) enum Expr {
        Op(Op),
        Num(f64),
        Var(String),
        Sub(Vec<Self>),
        Func(String, Vec<Vec<Self>>),
    }

    impl Expr {
        const fn is_operand(&self) -> bool {
            use self::Expr::{Func, Num, Op, Sub, Var};
            match *self {
                Op(_) => false,
                Num(_) | Var(_) | Sub(_) | Func(_, _) => true,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub(super) enum Token {
        Op(Op),
        Num(f64),
        Name(String, Sign),
        Sub(Vec<Self>),
        Comma,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(super) enum Sign {
        Pos,
        Neg,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(super) enum Paren {
        Open,
        Close,
    }

    #[derive(Debug, Clone)]
    pub(super) enum RawToken {
        Op(Op),
        Num(f64),
        Sign(Sign),
        Name(String),
        Paren(Paren),
        Comma,
    }

    #[derive(Debug, Clone, thiserror::Error)]
    pub enum ParseError {
        #[error("unexpected token: {0}")]
        UnexpectedToken(String),
        #[error("mismatched parentheses")]
        MismatchedParens,
        #[error("unknown function {0}")]
        UnknownFunction(String),
        #[error("invalid function arguments for {0}")]
        InvalidFunctionArgs(String),
        #[error("expected {0} not found")]
        Expected(String),
        #[error("unknown variable {0}")]
        UnknownVariable(String),
    }

    type TokenizerFn = fn(&str) -> Option<(RawToken, &str)>;

    fn next_paren(raw: &str) -> Option<(RawToken, &str)> {
        let paren = match raw.chars().next()? {
            '(' => Paren::Open,
            ')' => Paren::Close,
            _ => return None,
        };

        Some((RawToken::Paren(paren), &raw[1..]))
    }

    fn next_op(raw: &str) -> Option<(RawToken, &str)> {
        let op = match raw.chars().next()? {
            '+' => Op::Add,
            '-' => Op::Subtract,
            '*' => Op::Multiply,
            '/' => Op::Divide,
            '^' => Op::Power,
            _ => return None,
        };

        Some((RawToken::Op(op), &raw[1..]))
    }

    fn next_comma(raw: &str) -> Option<(RawToken, &str)> {
        if raw.chars().next()? != ',' {
            return None;
        }

        Some((RawToken::Comma, &raw[1..]))
    }

    fn next_sign(raw: &str) -> Option<(RawToken, &str)> {
        match raw.chars().next()? {
            '+' => Some((RawToken::Sign(Sign::Pos), &raw[1..])),
            '-' => Some((RawToken::Sign(Sign::Neg), &raw[1..])),
            _ => None,
        }
    }

    fn next_num(raw: &str) -> Option<(RawToken, &str)> {
        #[allow(clippy::unwrap_used)]
        let matcher =
            Regex::new(r"^(?:(?:0x([0-9a-f]+))|((?:[0-9]+\.?[0-9]*|\.[0-9]+)(?:e-?[0-9]+)?))")
                .unwrap();

        let cap = matcher.captures(raw)?;
        let val = if let Some(hex) = cap.get(1) {
            u64::from_str_radix(hex.as_str(), 16).ok()? as f64
        } else if let Some(float) = cap.get(2) {
            float.as_str().parse().ok()?
        } else {
            return None;
        };

        let num_len = cap[0].len();
        Some((RawToken::Num(val), &raw[num_len..]))
    }

    fn next_name(raw: &str) -> Option<(RawToken, &str)> {
        let mut name = "";

        let first = raw.chars().next()?;
        if !first.is_alphabetic() && first != '_' {
            return None;
        }

        #[allow(clippy::range_plus_one)]
        for c in raw.chars() {
            if c.is_alphanumeric() || c == '_' {
                name = &raw[..name.len() + 1];
            } else {
                break;
            }
        }

        if name.is_empty() {
            return None;
        }

        Some((RawToken::Name(name.to_string()), &raw[name.len()..]))
    }

    fn next_parser_order(previous: Option<&RawToken>) -> &[TokenizerFn] {
        #[allow(clippy::match_same_arms)]
        match previous {
            Some(&RawToken::Paren(Paren::Open)) => &[next_paren, next_name, next_num, next_sign],
            Some(&RawToken::Paren(Paren::Close)) => &[
                next_paren, next_comma, next_op, next_name, next_num, next_sign,
            ],
            Some(&RawToken::Op(_)) => &[next_paren, next_name, next_num, next_sign],
            Some(&RawToken::Num(_)) => &[next_paren, next_comma, next_op, next_name],
            Some(&RawToken::Sign(_)) => &[next_paren, next_name, next_num, next_sign],
            Some(&RawToken::Name(_)) => &[
                next_paren, next_comma, next_op, next_name, next_num, next_sign,
            ],
            Some(&RawToken::Comma) => &[next_paren, next_name, next_num, next_sign],
            None => &[next_paren, next_name, next_num, next_sign],
        }
    }

    fn next_token<'a>(
        raw: &'a str,
        previous: Option<&RawToken>,
    ) -> Result<(RawToken, &'a str), ParseError> {
        let order = next_parser_order(previous);

        // skip whitespace
        let mut token_start = 0;
        for c in raw.chars() {
            if c.is_whitespace() {
                token_start += 1;
            } else {
                break;
            }
        }

        let raw = &raw[token_start..];
        for &parser in order {
            if let Some(next) = parser(raw) {
                return Ok(next);
            }
        }

        Err(ParseError::UnexpectedToken(
            raw.chars().next().unwrap_or_default().to_string(),
        ))
    }

    pub(super) fn tokenize_raw(mut raw: &str) -> Result<Vec<RawToken>, ParseError> {
        let mut tokens = Vec::new();
        while !raw.is_empty() {
            let (next, rest) = next_token(raw, tokens.last())?;
            tokens.push(next);
            raw = rest;
        }

        Ok(tokens)
    }

    pub(super) fn process_raw_tokens(raw: &[RawToken]) -> Result<Vec<Token>, ParseError> {
        fn inner(raw: &[RawToken]) -> Result<Vec<Token>, ParseError> {
            let mut res = Vec::new();

            let mut paren_count = 0;
            let mut paren_start = 0;
            let mut counting_parens = false;

            let mut sign = None;

            for (i, token) in raw.iter().enumerate() {
                match *token {
                    RawToken::Paren(Paren::Open) => {
                        if !counting_parens {
                            counting_parens = true;
                            paren_start = i;
                        }

                        paren_count += 1;
                    }
                    RawToken::Paren(Paren::Close) => {
                        paren_count -= 1;
                        if paren_count < 0 {
                            return Err(ParseError::MismatchedParens);
                        }

                        if paren_count == 0 {
                            counting_parens = false;
                            let sub = inner(&raw[paren_start + 1..i])?;

                            if sign.take() == Some(Sign::Neg) {
                                res.push(Token::Sub(vec![
                                    Token::Num(-1.0),
                                    Token::Op(Op::Multiply),
                                    Token::Sub(sub),
                                ]));
                            } else {
                                res.push(Token::Sub(sub));
                            }
                        }
                    }
                    RawToken::Op(op) => {
                        if !counting_parens {
                            res.push(Token::Op(op));
                        }
                    }
                    RawToken::Num(val) => {
                        if !counting_parens {
                            let val = if sign.take() == Some(Sign::Neg) {
                                -val
                            } else {
                                val
                            };

                            res.push(Token::Num(val));
                        }
                    }
                    RawToken::Sign(Sign::Pos) => {
                        // noop
                    }
                    RawToken::Sign(Sign::Neg) => {
                        if !counting_parens {
                            if sign.take() == Some(Sign::Neg) {
                                continue;
                            }

                            sign = Some(Sign::Neg);
                        }
                    }
                    RawToken::Name(ref name) => {
                        if !counting_parens {
                            res.push(Token::Name(name.clone(), sign.take().unwrap_or(Sign::Pos)));
                        }
                    }
                    RawToken::Comma => {
                        if !counting_parens {
                            res.push(Token::Comma);
                        }
                    }
                }
            }

            Ok(res)
        }

        inner(raw)
    }

    pub(super) fn tokens_to_expr(raw: &[Token]) -> Result<Vec<Expr>, ParseError> {
        static FUNC_NAMES: [&str; 5] = ["abs", "log2", "sign", "max", "min"];

        fn push_pending_name(pending_name: &mut Option<(String, Sign)>, res: &mut Vec<Expr>) {
            if let Some((pending, sign)) = pending_name.take() {
                if sign == Sign::Pos {
                    res.push(Expr::Var(pending));
                } else {
                    let sub = vec![Expr::Num(-1.0), Expr::Op(Op::Multiply), Expr::Var(pending)];
                    res.push(Expr::Sub(sub));
                }
            }
        }

        let mut res = Vec::new();
        let mut pending_name = None;

        for t in raw {
            match t {
                Token::Num(val) => {
                    // name before number -> not a function
                    push_pending_name(&mut pending_name, &mut res);

                    res.push(Expr::Num(*val));
                }
                Token::Op(op) => {
                    // name before op -> not a function
                    push_pending_name(&mut pending_name, &mut res);

                    res.push(Expr::Op(*op));
                }
                Token::Sub(sub) => {
                    if let Some((pending, sign)) = pending_name.take() {
                        if FUNC_NAMES.contains(&pending.as_str()) {
                            let args = tokens_to_args(sub)?;

                            if sign == Sign::Pos {
                                res.push(Expr::Func(pending, args));
                            } else {
                                let sub = vec![
                                    Expr::Num(-1.0),
                                    Expr::Op(Op::Multiply),
                                    Expr::Func(pending, args),
                                ];
                                res.push(Expr::Sub(sub));
                            }
                        } else {
                            if sign == Sign::Pos {
                                res.push(Expr::Var(pending));
                            } else {
                                let sub = vec![
                                    Expr::Num(-1.0),
                                    Expr::Op(Op::Multiply),
                                    Expr::Var(pending),
                                ];
                                res.push(Expr::Sub(sub));
                            }
                            res.push(Expr::Sub(tokens_to_expr(sub)?));
                        }
                    } else {
                        res.push(Expr::Sub(tokens_to_expr(sub)?));
                    }
                }
                Token::Name(name, sign) => {
                    // name followed by name -> not a function
                    push_pending_name(&mut pending_name, &mut res);

                    pending_name = Some((name.clone(), *sign));
                }
                Token::Comma => {
                    // commas should only appear in function argument lists
                    // which get handled in `tokens_to_args`
                    return Err(ParseError::UnexpectedToken(",".to_string()));
                }
            }
        }

        push_pending_name(&mut pending_name, &mut res);

        Ok(res)
    }

    pub(super) fn tokens_to_args(raw: &[Token]) -> Result<Vec<Vec<Expr>>, ParseError> {
        let args = raw.split(|t| matches!(t, Token::Comma)).collect::<Vec<_>>();

        let mut res = Vec::new();
        for arg in args {
            if arg.is_empty() {
                continue;
            }

            res.push(tokens_to_expr(arg)?);
        }

        Ok(res)
    }

    pub(super) fn add_implicit_multi(mut raw: Vec<Expr>) -> Vec<Expr> {
        if raw.is_empty() {
            return raw;
        }

        let mut i = 0;
        while i < raw.len() - 1 {
            if raw[i].is_operand() && raw[i + 1].is_operand() {
                raw.insert(i + 1, Expr::Op(Op::Multiply));
            }

            i += 1;
        }

        let mut res = Vec::new();
        for expr in raw {
            match expr {
                Expr::Sub(sub) => res.push(Expr::Sub(add_implicit_multi(sub))),
                Expr::Func(name, args) => res.push(Expr::Func(
                    name,
                    args.into_iter().map(add_implicit_multi).collect(),
                )),
                e => res.push(e),
            }
        }

        res
    }

    pub(super) fn expr_to_postfix(raw: &[Expr]) -> Vec<Expr> {
        // shunting-yard algorithm
        fn inner(raw: &[Expr]) -> Vec<Expr> {
            let mut stack = Vec::new();
            let mut ops = Vec::new();

            for expr in raw {
                match *expr {
                    Expr::Num(num) => stack.push(Expr::Num(num)),
                    Expr::Var(ref name) => stack.push(Expr::Var(name.clone())),
                    Expr::Sub(ref sub) => stack.push(Expr::Sub(inner(sub))),
                    Expr::Func(ref name, ref args) => stack.push(Expr::Func(name.clone(), {
                        let mut new_args = Vec::new();
                        for arg in args {
                            new_args.push(inner(arg));
                        }
                        new_args
                    })),
                    Expr::Op(ref op) => {
                        while let Some(top_op) = ops.pop() {
                            if op.should_shunt(top_op) {
                                stack.push(Expr::Op(top_op));
                            } else {
                                ops.push(top_op);
                                break;
                            }
                        }

                        ops.push(*op);
                    }
                }
            }

            stack.extend(ops.into_iter().rev().map(Expr::Op));

            stack
        }

        inner(raw)
    }

    pub(super) fn postfix_expr_to_term(raw: &[Expr]) -> Result<Term, ParseError> {
        let mut stack = Vec::new();
        for expr in raw {
            match expr {
                Expr::Num(num) => stack.push(Term::Num(*num)),
                Expr::Sub(sub) => stack.push(postfix_expr_to_term(sub)?),
                Expr::Var(name) => stack.push(Term::Var(name.clone())),
                Expr::Func(name, args) => match name.as_str() {
                    "abs" => {
                        if args.len() != 1 {
                            return Err(ParseError::InvalidFunctionArgs(name.clone()));
                        }

                        let arg = postfix_expr_to_term(&args[0])?;
                        stack.push(Term::Func(Box::new(Func::Abs(arg))));
                    }
                    "log2" => {
                        if args.len() != 1 {
                            return Err(ParseError::InvalidFunctionArgs(name.clone()));
                        }

                        let arg = postfix_expr_to_term(&args[0])?;
                        stack.push(Term::Func(Box::new(Func::Log2(arg))));
                    }
                    "sign" => {
                        if args.len() != 1 {
                            return Err(ParseError::InvalidFunctionArgs(name.clone()));
                        }

                        let arg = postfix_expr_to_term(&args[0])?;
                        stack.push(Term::Func(Box::new(Func::Sign(arg))));
                    }
                    "max" => {
                        if args.len() < 2 || args.len() > 255 {
                            return Err(ParseError::InvalidFunctionArgs(name.clone()));
                        }

                        let args = args
                            .iter()
                            .map(|a| postfix_expr_to_term(a))
                            .collect::<Result<_, _>>()?;
                        stack.push(Term::Func(Box::new(Func::Max(args))));
                    }
                    "min" => {
                        if args.len() < 2 || args.len() > 255 {
                            return Err(ParseError::InvalidFunctionArgs(name.clone()));
                        }

                        let args = args
                            .iter()
                            .map(|a| postfix_expr_to_term(a))
                            .collect::<Result<_, _>>()?;
                        stack.push(Term::Func(Box::new(Func::Min(args))));
                    }
                    _ => return Err(ParseError::UnknownFunction(name.clone())),
                },
                Expr::Op(op) => {
                    let Some(right) = stack.pop() else {
                        return Err(ParseError::Expected("expression".to_string()));
                    };
                    let Some(left) = stack.pop() else {
                        return Err(ParseError::Expected("expression".to_string()));
                    };

                    stack.push(Term::Op(Box::new(left), *op, Box::new(right)));
                }
            }
        }

        // only a single term should exist
        if stack.len() > 1 {
            return Err(ParseError::Expected("operator".to_string()));
        }

        stack
            .pop()
            .ok_or_else(|| ParseError::Expected("expression".to_string()))
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]

    use std::collections::HashMap;

    use crate::MathExpression;

    fn eval(terms: &[(&str, f64)], vars: &HashMap<String, f64>) {
        for (term, expected) in terms {
            let expr = MathExpression((*term).to_string());
            let res = expr
                .eval(vars)
                .unwrap_or_else(|e| panic!("'{term}' should be valid. {e}"));

            assert!(
                (res - expected).abs() < f64::EPSILON,
                "{term} = {res}, expected {expected}"
            );
        }
    }

    #[test]
    fn evaluate_simple() {
        const TERM_TO_RES: &[(&str, f64)] = &[
            ("1337.42", 1337.42),
            ("-.025e2", -2.5),
            ("5 + 4 * 5", 25.0),
            ("5 + 4 * -5", -15.0),
            ("8 - 20.5", -12.5),
            ("-0xf + 15", 0.0),
            ("(3+1)^(5-2)", 64.0),
            ("64^0.5", 8.0),
            ("11 * (3.33 - 1.11 * 3) + 25.8 / -4", -6.45),
        ];

        let no_vars = HashMap::with_capacity(0);
        eval(TERM_TO_RES, &no_vars);
    }

    #[test]
    fn evaluate_functions() {
        const TERM_TO_RES: &[(&str, f64)] = &[
            ("abs(-42)", 42.0),
            ("-abs(3-9.99)", -6.99),
            ("log2(0x01)", 0.0),
            ("log2(256)", 8.0),
            ("sign(0)", 0.0),
            ("sign(-0)", 0.0),
            ("sign(-123.4)", -1.0),
            ("sign(567.8)", 1.0),
            ("max(-55, -12)", -12.0),
            ("max(-123, 0 + 5e2, 5^5, 420.69 / 25e-2)", 3125.0),
            ("min(27, -5 * (8 + 2.5))", -52.5),
            ("min(432, 11.0, -3/1e5, 0x00)", -0.000_03),
        ];

        let no_vars = HashMap::with_capacity(0);
        eval(TERM_TO_RES, &no_vars);
    }

    #[test]
    fn evaluate_vars() {
        const TERM_TO_RES: &[(&str, f64)] = &[
            ("a", 42.0),
            ("b", -13.5),
            ("c", 0.25),
            ("-k", -1000.0),
            ("47.5k", 47_500.0),
            ("1e-1 k k / 2", 50_000.0),
            ("a + b - c", 28.25),
            ("(abs(a) + abs(b)) * c", 13.875),
        ];

        let mut vars = HashMap::new();
        vars.insert("a".to_string(), 42.0);
        vars.insert("b".to_string(), -13.5);
        vars.insert("c".to_string(), 0.25);
        vars.insert("k".to_string(), 1000.0);

        eval(TERM_TO_RES, &vars);
    }
}
