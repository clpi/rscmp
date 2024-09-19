use std::borrow::{Borrow, Cow};
use std::convert::Infallible;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Sub};
use std::string::ParseError;
use crossbeam::epoch::Pointable;
use pest::pratt_parser::{Assoc, Op as POp, PrattParser, PrattParserMap};
use strum::{AsRefStr, EnumIs, EnumIter, EnumString, EnumTryAs, IntoStaticStr, VariantArray, AsStaticRef, EnumCount};
use std::num::{ParseFloatError, ParseIntError};
use std::result;
use std::str::FromStr;
use pest_derive::Parser;
use lng::prelude::parse::grammar::{Grammar, Rule};
use pest::{Parser, ParseResult};
use pest::iterators::Pair;
use pest_meta::parser::parse;
use serde::{Deserialize, Serialize};
use strum::Display;
use once_cell::sync::Lazy;


fn main() -> Result<(), pest::error::Error<Rule>> {
    let src = r#"(2 + 2) * (9 / 3) / 2"#;
    let pairs = Grammar::parse(Rule::calculation, src)?;
    println!("{:?}\n", pairs.clone());
    let mut pp = PrattParser::new();
    pp
        .map_primary(|p| 
            if let Rule::value = p.as_rule() {
                parse_value(p)
            } else {
                Value::Nil
            }
        )
        .map_postfix(|lhs: Value, op: Pair<Rule>| {
            let lhs_value: Value = lhs;
            match op.as_rule() {
                Rule::op_pos_inc => {
                    if let Value::Num(l) = lhs_value {
                        if l.is_int() {
                            return Value::Num(ValueNum::Int(l.try_as_int().unwrap_or_default() + 1));
                        } else if l.is_float() {
                            return Value::Num(ValueNum::Float(l.try_as_float().unwrap_or_default() + 1.0));
                        } else {
                            return Value::Num(ValueNum::Uint(l.try_as_uint().unwrap_or_default() + 1));
                        }
                    } else {
                        return lhs_value;
                    }
                }
                Rule::op_pos_dec => {
                    if let Value::Num(l) = lhs_value {
                        if l.is_int() {
                            return Value::Num(ValueNum::Int(l.try_as_int().unwrap_or_default() - 1));
                        } else if l.is_float() {
                            return Value::Num(ValueNum::Float(l.try_as_float().unwrap_or_default() - 1.0));
                        } else {
                            return Value::Num(ValueNum::Uint(l.try_as_uint().unwrap_or_default() - 1));
                        }
                    } else {
                        lhs_value
                    }
                }
                _ => return lhs_value,
            }
        })
        .map_prefix(|op: Pair<Rule>, lhs: Value| {
            let lhs_value: Value = lhs;
            match op.as_rule() {
                Rule::op_pre_dec => {
                    if let Value::Num(l) = lhs_value {
                        if l.is_int() {
                            return Value::Num(ValueNum::Int(l.try_as_int().unwrap_or_default() - 1));
                        } else if l.is_float() {
                            return Value::Num(ValueNum::Float(l.try_as_float().unwrap_or_default() - 1.0));
                        } else {
                            return Value::Num(ValueNum::Uint(l.try_as_uint().unwrap_or_default() - 1));
                        }
                    } else {
                        lhs_value
                    }
                }
                Rule::op_pre_inc => {
                    if let Value::Num(l) = lhs_value {
                        if l.is_int() {
                            return Value::Num(ValueNum::Int(l.try_as_int().unwrap_or_default() + 1));
                        } else if l.is_float() {
                            return Value::Num(ValueNum::Float(l.try_as_float().unwrap_or_default() + 1.0));
                        } else {
                            return Value::Num(ValueNum::Uint(l.try_as_uint().unwrap_or_default() + 1));
                        }
                    } else {
                        return lhs_value;
                    }
                }
                Rule::op_pre_neg => {
                    if let Value::Num(lhs) = lhs_value {
                        return Value::Num(-lhs);
                    } else {
                        return lhs_value;
                    }
                }
                Rule::op_pre_not => {
                    if let Value::Bool(lhs) = lhs_value {
                        return Value::Bool(!lhs);
                    } else {
                        return lhs_value;
                    }
                }
                Rule::op_pre_ptr => {
                    if let Value::Num(lhs) = lhs_value {
                        return Value::Num(lhs);
                    } else {
                        return lhs_value;
                    }
                }
                Rule::op_pre_ref => {
                    if let Value::Num(lhs) = lhs_value {
                        return Value::Num(lhs);
                    } else {
                        return lhs_value;
                    }
                }
                Rule::op_pre_pipe => {
                    if let Value::Num(lhs) = lhs_value {
                        return Value::Num(lhs);
                    } else {
                        return lhs_value;
                    }
                }
                Rule::op_pre_dot => {
                    if let Value::Num(lhs) = lhs_value {
                        return Value::Num(lhs);
                    } else {
                        return lhs_value;
                    }
                }
                _ => return lhs_value,
            }
        })
        .map_infix(|lhs: Value, op: Pair<Rule>, rhs: Value| {
            // let lhs_value : Value= parse_value(lhs);
            // let rhs_value: Value = parse_value(rhs);
            let rhs_value: Value = rhs;
            let lhs_value: Value = lhs;
            let op_value: Op = parse_op(op);
            match op_value {
                Op::Nop => lhs_value,
                Op::Return(v) => v,
                Op::Assign(op) => {
                    match op {
                        _ => lhs_value.clone(),
                    }
       
                },
                Op::Math(op) => {
                    if let (Value::Num(lhs), Value::Num(rhs)) = (lhs_value.clone(), rhs_value.clone()) {
                        match op {
                            OpMath::Add => Value::Num(lhs.clone() + rhs.clone()),
                            OpMath::Sub => Value::Num(lhs.clone() - rhs.clone()),
                            OpMath::Mul => Value::Num(lhs.clone() * rhs.clone()),
                            OpMath::Div => Value::Num(lhs.clone() / rhs.clone()),
                            OpMath::Rem => Value::Num(lhs.clone() % rhs.clone()),
                            OpMath::Pow => {
                                let base = lhs.clone();
                                let exp = rhs.try_as_int().unwrap_or_default();
                                let mut result = ValueNum::Int(1);
                                for _ in 0..exp {
                                    result = result * base.clone();
                                }
                                Value::Num(result)
                            }
                            _ => lhs_value,
                        }
                    } else {
                        lhs_value
                    }
                }
                Op::Logic(op) => {
                    match op {  
                        OpLogic::Not => {
                            if let Value::Bool(lhs) = lhs_value.clone() {
                                Value::Bool(!lhs)
                            } else {
                                lhs_value.clone()
                            }
                        }
                        OpLogic::Or => {
                            if let (Value::Bool(lhs), Value::Bool(rhs)) = (lhs_value.clone(), rhs_value.clone()) {
                                Value::Bool(lhs || rhs)
                            } else {
                                lhs_value.clone()
                            }
                        }
                        OpLogic::And => {
                            if let (Value::Bool(lhs), Value::Bool(rhs)) = (lhs_value.clone(), rhs_value.clone()) {
                                Value::Bool(lhs && rhs)
                            } else {
                                lhs_value.clone()
                            }
                        }
                        OpLogic::Xor => {
                            if let (Value::Bool(lhs), Value::Bool(rhs)) = (lhs_value.clone(), rhs_value.clone()) {
                                Value::Bool(lhs ^ rhs)
                            } else {
                                lhs_value.clone()
                            }
                        },
                        _ => lhs_value.clone(),
                    }
                }
                Op::Cmp(op) => {
                    match op {
                        OpCmp::Eq => Value::Bool(lhs_value == rhs_value),
                        OpCmp::Ne => Value::Bool(lhs_value != rhs_value),
                        OpCmp::Eq3 => Value::Bool(lhs_value == rhs_value),
                        OpCmp::Ne3 => Value::Bool(lhs_value != rhs_value),
                        OpCmp::Le => Value::Bool(lhs_value <= rhs_value),
                        OpCmp::Gt => Value::Bool(lhs_value > rhs_value),
                        OpCmp::Ge => Value::Bool(lhs_value >= rhs_value),
                        OpCmp::Lt => Value::Bool(lhs_value < rhs_value),
                        _ => lhs_value,
                    }
                }
                Op::Nop => lhs_value,
                _ => lhs_value,
            }
        })
        .parse(pairs.clone());
    let ps = pairs.clone().into_iter();
    println!("{:?}", &ps.clone());
    for pair in ps.clone() {
        parse_calculation(pair);
    }
    // Ok(())
        // });
    // let src = r#"true and false"#;
    // for pair in ps {
    //     parse_calculation(pair);
    // }
    Ok(())
}


#[derive(Debug, EnumIs, Clone, EnumIter, EnumTryAs, AsRefStr, IntoStaticStr, Default, Serialize, Display, Deserialize)]
pub enum Expr {
    #[default]
    Nop,
    Single {
        lhs: Box<Expr>,
        op: Op,
    },
    Bin {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: Op,
    }
}

#[derive(Debug, EnumIs, Clone, EnumIter, EnumTryAs, AsRefStr, IntoStaticStr, Default, Serialize, Display, Deserialize)]
pub enum Parse {
    Expr,
    Term,
    Value,
    Op,
    Comment,
    #[default]
    EOI,
}
#[derive(PartialEq, Clone, PartialOrd, Debug, EnumIs, EnumIter, EnumTryAs, AsRefStr, IntoStaticStr, Default, Serialize, Display, Deserialize)]
pub enum Value {
    Num(ValueNum),
    Bool(bool),
    Str(String),
    Chr(char),
    #[default]
    Nil,
}

impl From<ValueNum> for Value {
    fn from(num: ValueNum) -> Self {
        Value::Num(num)
    }
}
impl From<Value> for ValueNum {
    fn from(value: Value) -> Self {
        match value {
            Value::Num(num) => num,
            _ => ValueNum::default(),
        }
    }
}
impl From<Pair<'_, Rule>> for Value {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair.as_rule() {
            Rule::num => Value::Num(ValueNum::parse(pair)),
            Rule::int => Value::Num(ValueNum::Int(pair.as_str().parse().unwrap())),
            Rule::float => Value::Num(ValueNum::Float(pair.as_str().parse().unwrap())),
            Rule::uint => Value::Num(ValueNum::Uint(pair.as_str().parse().unwrap())),
            Rule::bool => Value::Bool(pair.as_str().parse().unwrap()),
            Rule::string => Value::Str(pair.as_str().to_string()),
            Rule::character => Value::Chr(pair.as_str().chars().next().unwrap()),
            Rule::nil => Value::Nil,
            _ => Value::Nil,
        }
    }
}
impl Not for Value {
    type Output = bool;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(b) => !b,
            Value::Num(ValueNum::Int(i)) => i == 0,
            Value::Num(ValueNum::Uint(u)) => u == 0,
            Value::Num(ValueNum::Float(f)) => f == 0.0, 
            Value::Str(s) => s.is_empty(),
            Value::Chr(c) => c == '\0',
            Value::Nil => true,
            _ => false,
        }
    }
}
impl Add for ValueNum {
    type Output = ValueNum;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ValueNum::Int(lhs), ValueNum::Int(rhs)) => ValueNum::Int(lhs + rhs),
            (ValueNum::Float(lhs), ValueNum::Float(rhs)) => ValueNum::Float(lhs + rhs),
            (ValueNum::Uint(lhs), ValueNum::Uint(rhs)) => ValueNum::Uint(lhs + rhs),
            _ => panic!("Cannot add values of different types"),
        }
    }
}
impl BitAnd for Value {
    type Output = Value;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Bool(lhs), Value::Bool(rhs)) => Value::Bool(lhs &&rhs),
            (Value::Num(ValueNum::Int(lhs)), Value::Num(ValueNum::Int(rhs))) => Value::Bool(lhs > 0 && rhs > 0),
            (Value::Num(ValueNum::Uint(lhs)), Value::Num(ValueNum::Uint(rhs))) => Value::Bool(lhs > 0 && rhs > 0),
            (Value::Num(ValueNum::Int(lhs)), Value::Num(ValueNum::Uint(rhs))) => Value::Bool(lhs > 0 && rhs > 0),
            (Value::Num(lhs), Value::Num(rhs)) => Value::Bool(lhs.try_as_int().unwrap_or_default().gt(&0) & rhs.try_as_int().unwrap_or_default().gt(&0) ),
            _ => panic!("Cannot bitwise AND values of different types"),
        }
    }
}
impl BitOr for Value {
    type Output = Value;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(lhs), Value::Num(rhs)) => Value::Bool(lhs.try_as_int().unwrap_or_default().gt(&0) | rhs.try_as_int().unwrap_or_default().gt(&0) ),
            (Value::Num(ValueNum::Int(lhs)), Value::Num(ValueNum::Uint(rhs))) => Value::Bool(lhs.gt(&0) | rhs.gt(&0) ),
            (Value::Num(lhs), Value::Num(rhs)) => Value::Bool(lhs.try_as_int().unwrap_or_default().gt(&0) | rhs.try_as_int().unwrap_or_default().gt(&0) ),
            (Value::Num(ValueNum::Uint(lhs)), Value::Num(ValueNum::Int(rhs))) => Value::Bool(lhs.gt(&0) | rhs.gt(&0) ),
            (Value::Num(lhs), Value::Num(rhs)) => Value::Bool(lhs.try_as_int().unwrap_or_default().gt(&0) | rhs.try_as_int().unwrap_or_default().gt(&0) ),
            _ => panic!("Cannot bitwise OR values of different types"),
        }
    }
}
impl BitXor for Value {
    type Output = Value;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(ValueNum::Int(i1)), Value::Num(ValueNum::Int(i2))) => Value::Num(ValueNum::Int(i1 ^ i2)),
            (Value::Num(ValueNum::Uint(u1)), Value::Num(ValueNum::Uint(u2))) => Value::Num(ValueNum::Uint(u1 ^ u2)),
            (Value::Bool(b1), Value::Bool(b2)) => Value::Bool(b1 ^ b2),
            (Value::Num(lhs), Value::Num(rhs)) => Value::Num(ValueNum::Int(lhs.try_as_int().unwrap_or_default() ^ rhs.try_as_int().unwrap_or_default())),
            _ => panic!("Cannot bitwise XOR values of different types"),
        }
    }
}
impl Shl for Value {
    type Output = Value;

    fn shl(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(lhs), Value::Num(rhs)) => Value::Num(ValueNum::Int(lhs.try_as_int().unwrap_or_default() << rhs.try_as_int().unwrap_or_default())),
            _ => panic!("Cannot shift left values of different types"),
        }
    }
}
impl Sub for ValueNum {
    type Output = ValueNum;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ValueNum::Int(lhs), ValueNum::Int(rhs)) => ValueNum::Int(lhs - rhs),
            (ValueNum::Float(lhs), ValueNum::Float(rhs)) => ValueNum::Float(lhs - rhs),
            (ValueNum::Uint(lhs), ValueNum::Uint(rhs)) => ValueNum::Uint(lhs - rhs),
            _ => panic!("Cannot subtract values of different types"),
        }
    }   
}
impl Mul for ValueNum {
    type Output = ValueNum;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ValueNum::Int(lhs), ValueNum::Int(rhs)) => ValueNum::Int(lhs * rhs),
            (ValueNum::Float(lhs), ValueNum::Float(rhs)) => ValueNum::Float(lhs * rhs),
            (ValueNum::Uint(lhs), ValueNum::Uint(rhs)) => ValueNum::Uint(lhs * rhs),
            _ => panic!("Cannot multiply values of different types"),
        }
    }   
}
impl Div for ValueNum {
    type Output = ValueNum;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ValueNum::Int(lhs), ValueNum::Int(rhs)) => ValueNum::Int(lhs / rhs),
            (ValueNum::Float(lhs), ValueNum::Float(rhs)) => ValueNum::Float(lhs / rhs),
            (ValueNum::Uint(lhs), ValueNum::Uint(rhs)) => ValueNum::Uint(lhs / rhs),
            _ => panic!("Cannot divide values of different types"),
        }
    }   
}
impl Rem for ValueNum {
    type Output = ValueNum;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ValueNum::Int(lhs), ValueNum::Int(rhs)) => ValueNum::Int(lhs % rhs),
            (ValueNum::Float(lhs), ValueNum::Float(rhs)) => ValueNum::Float(lhs % rhs),
            (ValueNum::Uint(lhs), ValueNum::Uint(rhs)) => ValueNum::Uint(lhs % rhs),
            _ => panic!("Cannot modulo values of different types"),
        }   
    }
}
impl std::ops::Neg for ValueNum {
    type Output = ValueNum;

    fn neg(self) -> Self::Output {
        match self {
            ValueNum::Int(i) => ValueNum::Int(-i),
            ValueNum::Float(f) => ValueNum::Float(-f),
            ValueNum::Uint(u) => ValueNum::Uint(u),
            _ => panic!("Cannot negate values of different types"),
        }
    }
}
impl FromStr for Value {
    type Err = either::Either<ParseIntError, ParseFloatError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Value::Num(ValueNum::from_str(s)?))
    }
}

impl From<Pair<'_, Rule>> for Op {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair.as_rule() {
            Rule::assign => Op::Assign(OpAssign::from_str(pair.as_str()).unwrap_or_default()),
            Rule::math => Op::Math(OpMath::from_str(pair.as_str()).unwrap_or_default()),
            Rule::logic => Op::Logic(OpLogic::from_str(pair.as_str()).unwrap_or_default()),
            Rule::cmp => Op::Cmp(OpCmp::from_str(pair.as_str()).unwrap_or_default()),
            Rule::nop => Op::Nop,
            Rule::EOI => Op::Nop,
            Rule::ln_comment => Op::Nop,
            Rule::WHITESPACE => Op::Nop,
            Rule::expr => Op::Nop,
            Rule::calculation => Op::Nop,
            Rule::term => Op::Nop,
            Rule::value => Op::Return(Value::parse(pair)),
            _ => Op::Nop,
        }
    }
}
impl Value {
    pub fn parse(pair: Pair<Rule>) -> Value {
        match pair.as_rule() {
            Rule::int => Value::Num(ValueNum::Int(pair.as_str().parse().unwrap())),
            Rule::float => Value::Num(ValueNum::Float(pair.as_str().parse().unwrap())),
            Rule::uint => Value::Num(ValueNum::Uint(pair.as_str().parse().unwrap())),
            _ => panic!("Invalid rule"),
        }
    }
}

#[derive(PartialEq, Clone, PartialOrd, Debug, EnumIs, EnumIter, EnumTryAs, AsRefStr, IntoStaticStr, Serialize, Display, Deserialize)]
pub enum ValueNum {
    Int(i64),
    Float(f64),
    Uint(u64),
}
impl From<Pair<'_, Rule>> for ValueNum {
    fn from(pair: Pair<'_, Rule>) -> Self {
        ValueNum::parse(pair)
    }
}

impl Default for ValueNum {
    fn default() -> Self {
        ValueNum::Int(0)
    }
}

impl FromStr for ValueNum {
    type Err = either::Either<ParseIntError, ParseFloatError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i64>() {
            Ok(i) => ValueNum::Int(i),
            Err(_) => match s.parse::<f64>() {
                Ok(f) => ValueNum::Float(f),
                Err(_) => s.parse::<u64>().map(ValueNum::Uint)
                    .unwrap_or_default(),
            },
        })
    }
}

impl ValueNum {
    pub fn parse(pair: Pair<Rule>) -> ValueNum {
        let p = pair.as_str();
        match pair.as_rule() {
            Rule::int => ValueNum::Int(p.parse().unwrap()),
            Rule::float => ValueNum::Float(p.parse().unwrap()),
            Rule::uint => ValueNum::Uint(p.parse().unwrap()),
            _ => ValueNum::default(),
        }
    }
}

#[derive(Debug, Clone, EnumIs, AsRefStr, IntoStaticStr, EnumTryAs, Default, EnumIter, EnumCount, Serialize, Display, Deserialize)]
pub enum Op {
    Assign(OpAssign),
    Math(OpMath),
    Logic(OpLogic),
    Cmp(OpCmp),
    Return(Value),
    #[default]
    Nop,
}

#[derive(Default, Clone, Debug, EnumIs, AsRefStr, IntoStaticStr, EnumTryAs, EnumIter, EnumCount, Serialize, Display, Deserialize)]
pub enum OpAssign {
    #[default]
    Equal,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
    Define,
    Question,
}
#[derive(Default, Clone, Debug, EnumIs, AsRefStr, IntoStaticStr, EnumTryAs, EnumIter, EnumCount, Serialize, Display, Deserialize)]
pub enum OpMath {
    #[default]
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
}
#[derive(Default, Clone, Debug, EnumIs, AsRefStr, IntoStaticStr, EnumTryAs, EnumIter, EnumCount, Serialize, Display, Deserialize)]
pub enum OpLogic {
    #[default]
    And,
    Or,
    Not,
    Xor,
}
#[derive(Default, Debug, Clone, EnumIs, AsRefStr, IntoStaticStr, EnumTryAs, EnumIter, EnumCount, Serialize, Display, Deserialize)]
pub enum OpCmp {
    #[default]
    Eq,
    Ne,
    Eq3,
    Ne3,
    Le,
    Gt,
    Ge,
    Lt,
}

impl OpAssign {
    pub fn parse(pair: Pair<Rule>) -> OpAssign {
        match pair.as_rule() {
            Rule::assign => OpAssign::parse(pair),
            _ => panic!("Invalid rule"),
        }
    }
}
impl FromStr for OpLogic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "and" => Ok(OpLogic::And),
            "or" => Ok(OpLogic::Or),
            "not" => Ok(OpLogic::Not),
            "xor" => Ok(OpLogic::Xor),
            _ => Err(()),
        }
    }
}
impl FromStr for OpCmp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "==" => Ok(OpCmp::Eq),
            "!=" => Ok(OpCmp::Ne),
            "===" => Ok(OpCmp::Eq3),
            "!==" => Ok(OpCmp::Ne3),
            "<=" => Ok(OpCmp::Le),
            ">=" => Ok(OpCmp::Ge),
            "<" => Ok(OpCmp::Lt),
            ">" => Ok(OpCmp::Gt),
            _ => Err(()),
        }
    }
}
impl FromStr for OpMath {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(OpMath::Add),
            "-" => Ok(OpMath::Sub),
            "*" => Ok(OpMath::Mul),
            "/" => Ok(OpMath::Div),
            "%" => Ok(OpMath::Rem),
            "^" => Ok(OpMath::Pow),
            _ => Err(()),
        }
    }
}
impl FromStr for OpAssign {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "=" => Ok(OpAssign::Equal),
            "+=" => Ok(OpAssign::Add),
            "-=" => Ok(OpAssign::Sub),
            "*=" => Ok(OpAssign::Mul),
            "/=" => Ok(OpAssign::Div),
            "%=" => Ok(OpAssign::Rem),
            "^=" => Ok(OpAssign::Pow),
            ":=" => Ok(OpAssign::Define),
            "?=" => Ok(OpAssign::Question),
            _ => Err(()),
        }
    }
}
pub fn parse_comment(pair: Pair<Rule>) {
    println!("comment {:#?}\t", pair.as_rule());
}

pub fn parse_expr(pair: Pair<Rule>) {
    println!("expr {:#?}\t", pair.as_rule());
}
pub fn parse_calculation(pair: Pair<Rule>) {
    for pair in pair.clone().into_inner() {
        let r = pair.as_rule();
        match r {
            Rule::ln_comment => {
                parse_comment(pair);
            }
            Rule::expr => {
                parse_expr(pair);
            }
            Rule::value => {
                parse_value(pair);
            }
            Rule::op => {
                parse_op(pair);
            }
            Rule::EOI => {
                println!("EOI");
            }
            _ => (),
        }
    }
}


pub fn parse_value(pair: Pair<Rule>) -> Value {
            // println!("value {:#?}\t", r);
        // println!("{:?}", pair);
        match pair.as_rule() {
            Rule::num => parse_num(pair),
            Rule::bool => parse_bool(pair),
            Rule::nil => Value::Nil,
            Rule::string => parse_string(pair),
            Rule::character => parse_character(pair),
            /// Quote // Code type?
            _ => Value::Nil,
    }
}

pub fn parse_bool(pair: Pair<Rule>) -> Value {
    println!("bool {:#?}\t", pair.as_rule());
    return Value::Bool(pair.as_str().parse().unwrap());
}

pub fn parse_nil(pair: Pair<Rule>) -> Value {
    println!("nil {:#?}\t", pair.as_rule());
    return Value::Nil;
}

pub fn parse_string(pair: Pair<Rule>) -> Value {
    println!("string {:#?}\t", pair.as_rule());
    let s = pair.as_str().to_string();
    return Value::Str(s.into());
}

pub fn parse_character(pair: Pair<Rule>) -> Value {
    println!("character {:#?}\t", pair.as_rule());
    return Value::Chr(pair.as_str().chars().next().unwrap());
}

pub fn parse_num(pair: Pair<Rule>) -> Value {
    let re = match pair.as_rule() {
        
        Rule::int => {
            println!("int {:#?}\t", pair.as_str());
            Value::Num(ValueNum::Int(pair.as_str().parse().unwrap()))
        }
        Rule::float => {
            println!("float {:#?}\t", pair.as_str());
            Value::Num(ValueNum::Float(pair.as_str().parse().unwrap()))
        }
        Rule::uint => {
            println!("uint {:#?}\t", pair.as_str());
            Value::Num(ValueNum::Uint(pair.as_str().parse().unwrap()))
        }
        _ => Value::Num(ValueNum::default()),
        /// Complex?
    };
    return re;
}

pub fn parse_op(pair: Pair<Rule>) -> Op {
        match pair.clone().as_rule() {
            Rule::nop => {
                println!("nop {:#?}\t", pair.as_rule());
                return Op::Nop;
            }
            Rule::assign => {
                return Op::Assign(parse_assign(pair));
            }
            Rule::math => {
                return Op::Math(parse_math(pair));
            }
            Rule::logic => {
                return Op::Logic(parse_logic(pair));
            }
            Rule::cmp => {
                return Op::Cmp(parse_cmp(pair));
            }
            _ => Op::default(),
    }
}

pub fn parse_assign(pair: Pair<Rule>) -> OpAssign {
    match pair.as_rule() {
        Rule::eeq => {
            println!("= {:#?}\t", pair.as_rule());
            return OpAssign::Equal;
        }
        Rule::edef => {
            println!(":={:#?}\t", pair.as_rule());
            return OpAssign::Define;
        }
        Rule::eque => {
            println!("?= {:#?}\t", pair.as_rule());
            return OpAssign::Question;
        }
        Rule::eadd => {
            println!("+= {:#?}\t", pair.as_rule());
            return OpAssign::Add;
        }
        Rule::esub => {
            println!("-= {:#?}\t", pair.as_rule());
            return OpAssign::Sub;
        }
        Rule::emul => {
            println!("*= {:#?}\t", pair.as_rule());
            return OpAssign::Mul;
        }
        Rule::ediv => {
            println!("/= {:#?}\t", pair.as_rule());
            return OpAssign::Div;
        }
        Rule::epow => {
            println!("^= {:#?}\t", pair.as_rule());
            return OpAssign::Pow;
        }
        Rule::erem => {
            println!("%= {:#?}\t", pair.as_rule());
            return OpAssign::Rem;
        }
        _ => OpAssign::default(),
    }
}

pub fn parse_math(pair: Pair<Rule>) -> OpMath {
    match pair.as_rule() {
        Rule::add => {
            println!("+ {:#?}\t", pair.as_rule());
            return OpMath::Add;
        }
        Rule::sub => {
            println!("- {:#?}\t", pair.as_rule());
            return OpMath::Sub;
        }
        Rule::mul => {
            println!("* {:#?}\t", pair.as_rule());
            return OpMath::Mul;
        }
        Rule::div => {
            println!("/ {:#?}\t", pair.as_rule());
            return OpMath::Div;
        }
        Rule::rem => {
            println!("% {:#?}\t", pair.as_rule());
            return OpMath::Rem;
        }
        _ => OpMath::default(), 
    }
}

pub fn parse_logic(pair: Pair<Rule>) -> OpLogic {
        match pair.as_rule() {
            Rule::and => {
                println!("&& {:#?}\t", pair.as_rule());
                return OpLogic::And;
            }
            Rule::or => {
                println!("|| {:#?}\t", pair.as_rule());
                return OpLogic::Or;
            }
            Rule::not => {
                println!("! {:#?}\t", pair.as_rule());
                return OpLogic::Not;
            }
            Rule::xor => {
                println!("! {:#?}\t", pair.as_rule());
                return OpLogic::Xor;
            }
            _ => OpLogic::default(),
        }
}

pub fn parse_cmp(pair: Pair<Rule>) -> OpCmp {
        match pair.as_rule() {
            Rule::cmpeq => {
                println!("== {:#?}\t", pair.as_rule());
                return OpCmp::Eq;
            }
            Rule::cmpne => {
                println!("!= {:#?}\t", pair.as_rule());
                return OpCmp::Ne;
            }
            Rule::cmpeq3 => {
                println!("=== {:#?}\t", pair.as_rule());
                return OpCmp::Eq3;
            }
            Rule::cmpne3 => {
                println!("!= {:#?}\t", pair.as_rule());
                return OpCmp::Ne3;
            }
            Rule::cmple => {
                println!("<= {:#?}\t", pair.as_rule());
                return OpCmp::Le;
            }
            Rule::cmpgt => {
                println!("> {:#?}\t", pair.as_rule());
                return OpCmp::Gt;
            }
            Rule::cmpge => {
                println!(">= {:#?}\t", pair.as_rule());
                return OpCmp::Ge;
            }
            Rule::cmplt => {
                println!("< {:#?}\t", pair.as_rule());
                return OpCmp::Lt;
            }
            _ => OpCmp::default(),
    }
}
