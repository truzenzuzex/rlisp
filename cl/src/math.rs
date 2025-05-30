// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::hash_map::HashMap;

use env::named_lambda::RLNamedLambda;
use env::result::RLResult;
use env::symb::RLEnvSymbol;

use err::err::{RLError,
               CustomParseFloatError,
               TypeError};

use expr::atom::RLAtom;
use expr::sexpr::SExpr;

use hash::hash::RLHash;

pub struct MathFuncs {
}

impl MathFuncs {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                cl_pack_hash: &mut HashMap<String, RLEnvSymbol, RLHash>) {

        // cl_pack_env_hash
        cl_pack_hash.insert("+".to_string(),
            RLEnvSymbol::new_named_lambda("+".to_string(),
                RLNamedLambda::new_func(
                    "ADD".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Return the sum of its arguments. With no args, returns 0."))),
                    "COMMON-LISP".to_string(),
                    Some(|sexpr| {
                        Ok(RLResult::NumRes(ADD(&sexpr)?)) }),
                    None)));

        cl_pack_hash.insert("-".to_string(),
            RLEnvSymbol::new_named_lambda("-".to_string(),
                RLNamedLambda::new_func(
                    "MINUS".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Subtract the second and all subsequent arguments from the first;
  or with one argument, negate the first argument."))),
                    "COMMON-LISP".to_string(),
                    Some(|sexpr| {
                        Ok(RLResult::NumRes(MINUS(&sexpr)?)) }),
                    None)));

        cl_pack_hash.insert("*".to_string(),
            RLEnvSymbol::new_named_lambda("*".to_string(),
                RLNamedLambda::new_func(
                    "MUL".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Return the product of its arguments. With no args, returns 1."))),
                    "COMMON-LISP".to_string(),
                    Some(|sexpr| {
                        Ok(RLResult::NumRes(MUL(&sexpr)?)) }),
                    None)));

        cl_pack_hash.insert("/".to_string(),
            RLEnvSymbol::new_named_lambda("/".to_string(),
                RLNamedLambda::new_func(
                    "DIV".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Divide the first argument by each of the following arguments, in turn.
  With one argument, return reciprocal."))),
                    "COMMON-LISP".to_string(),
                    Some(|sexpr| {
                        Ok(RLResult::NumRes(DIV(&sexpr)?)) }),
                    None)));
    }
}

#[allow(non_snake_case)]
pub fn ADD(sexpr: &SExpr) -> Result<f64, RLError> {
    match sexpr {
        SExpr::Atom(atom) => {
            let it = atom.get_atom_string();

            let mut number = 0.0;

            if let Ok(f64_atom) = it.parse::<f64>() {
                number = f64_atom;
            }

            if let Err(err) = it.parse::<f64>() {
                let err2 = CustomParseFloatError::new(&it, "NUMBER", err);
                    return Err(RLError::ParseFloatError(err2))
            }
            Ok(number)
        }

        SExpr::Cons(symb, v) => {
            match &*symb.name {
                "+" => {
                    let mut iter = v.iter().peekable();

                    if let None = iter.peek() {
                        return Err(RLError::SimpleProgramError);
                    }

                    let mut add0;

                    match ADD(iter.next().unwrap()) {
                        Ok(num)  => add0 = num,
                        Err(err) => return Err(err),
                    }

                    while let Some(elem) = iter.next() {
                        let summand;

                        match ADD(elem) {
                            Ok(num)  => summand = num,
                            Err(err) => return Err(err),
                        }

                        add0 = add0 + summand
                    }
                    Ok(add0)
                }

                &_ => unreachable!(),
            }
        }

        SExpr::Nil(_nil) => {
            let err = TypeError::new("NIL", "NUMBER");
                return Err(RLError::TypeError(err));
        }

        _ => Ok(0.0)
    }
}

#[allow(non_snake_case)]
pub fn MINUS(sexpr: &SExpr) -> Result<f64, RLError> {
    match sexpr {
        SExpr::Atom(atom) => {
            let it = atom.get_atom_string();

            let mut number = 0.0;

            if let Ok(f64_atom) = it.parse::<f64>() {
                number = f64_atom;
            }

            if let Err(err) = it.parse::<f64>() {
                let err2 = CustomParseFloatError::new(&it, "NUMBER", err);
                    return Err(RLError::ParseFloatError(err2))
            }
            Ok(number)
        }

        SExpr::Cons(symb, v) => {
            match &*symb.name {
                "-" => {
                    let mut iter = v.iter().peekable();

                    // Are there any arguments?!
                    if let None = iter.peek() {
                        return Err(RLError::SimpleProgramError);
                    }

                    // Define minuend first
                    let mut sub0;

                    // Check for possible type errors
                    match MINUS(iter.next().unwrap()) {
                        Ok(num)  => sub0 = num,
                        Err(err) => return Err(err),
                    }

                    // If there is no subtrahend, change sign
                    if let None = iter.peek() {
                        sub0 = -1.0 * sub0;
                    }

                    // There exist subtrahends
                    while let Some(elem) = iter.next() {
                        let subtrahend;

                        // Check for possible type errors
                        match MINUS(elem) {
                            Ok(num)  => subtrahend = num,
                            Err(err) => return Err(err),
                        }

                        sub0 = sub0 -subtrahend
                    }
                    Ok(sub0)
                }

                &_ => unreachable!(),
            }
        }

        SExpr::Nil(_nil) => {
            let err = TypeError::new("NIL", "NUMBER");
                return Err(RLError::TypeError(err));
        }

        _ => Ok(0.0)
    }
}

#[allow(non_snake_case)]
pub fn MUL(sexpr: &SExpr) -> Result<f64, RLError> {
    match sexpr {
        SExpr::Atom(atom) => {
            let it = atom.get_atom_string();

            let mut number = 0.0;

            if let Ok(f64_atom) = it.parse::<f64>() {
                number = f64_atom;
            }

            if let Err(err) = it.parse::<f64>() {
                let err2 = CustomParseFloatError::new(&it, "NUMBER", err);
                    return Err(RLError::ParseFloatError(err2))
            }
            Ok(number)
        }

        SExpr::Cons(symb, v) => {
            match &*symb.name {
                "*" => {
                    let mut iter = v.iter().peekable();

                    if let None = iter.peek() {
                        return Err(RLError::SimpleProgramError);
                    }

                    let mut mul0;

                    match MUL(iter.next().unwrap()) {
                        Ok(num)  => mul0 = num,
                        Err(err) => return Err(err),
                    }

                    while let Some(elem) = iter.next() {
                        let factor;

                        match MUL(elem) {
                            Ok(num)  => factor = num,
                            Err(err) => return Err(err),
                        }

                        mul0 = mul0 * factor
                    }
                    Ok(mul0)
                }

                &_ => unreachable!(),
            }
        }

        SExpr::Nil(_nil) => {
            let err = TypeError::new("NIL", "NUMBER");
                return Err(RLError::TypeError(err));
        }

        _ => Ok(0.0)
    }
}

#[allow(non_snake_case)]
pub fn DIV(sexpr: &SExpr) -> Result<f64, RLError> {
    match sexpr {
        SExpr::Atom(atom) => {
            let it = atom.get_atom_string();

            let mut number = 0.0;

            if let Ok(f64_atom) = it.parse::<f64>() {
                number = f64_atom;
            }

            if let Err(err) = it.parse::<f64>() {
                let err2 = CustomParseFloatError::new(&it, "NUMBER", err);
                    return Err(RLError::ParseFloatError(err2))
            }
            Ok(number)
        }

        SExpr::Cons(symb, v) => {
            match &*symb.name {
                "/" => {
                    let mut iter = v.iter().peekable();

                    if let None = iter.peek() {
                        return Err(RLError::SimpleProgramError);
                    }

                    let mut div0;

                    match DIV(iter.next().unwrap()) {
                        Ok(num)  => div0 = num,
                        Err(err) => return Err(err),
                    }

                    if let None = iter.peek() {
                        div0 = 1.0 / div0;
                    }

                    while let Some(elem) = iter.next() {
                        let divisor;

                        match DIV(elem) {
                            Ok(num)  => divisor = num,
                            Err(err) => return Err(err),
                        }

                        if divisor == 0.0 {
                            return Err(RLError::DivisionByZero);
                        } else {
                            div0 = div0 / divisor;
                        }
                    }
                    Ok(div0)
                }

                &_ => unreachable!(),
            }
        }

        SExpr::Nil(_nil) => {
            let err = TypeError::new("NIL", "NUMBER");
                return Err(RLError::TypeError(err));
        }

        _ => Ok(0.0)
    }
}

/*
#[allow(non_snake_case)]
pub fn SIMPLEMATH(sexpr: &SExpr) -> Result<f64, RLError> {
        match sexpr {
            SExpr::Atom(it) => {
                let mut number = 0.0;

                if let Ok(f64_atom) = it.parse::<f64>() {
                    number = f64_atom;
                }

                if let Err(err) = it.parse::<f64>() {
                    let err2 = CustomParseFloatError::new(it, "NUMBER", err);
                    return Err(RLError::ParseFloatError(err2))
                }
                Ok(number)
            }

            SExpr::Cons(symb, v) => {
                match &*symb.name {
                    "+" => {
                        let mut iter = v.iter().peekable();

                        if let None = iter.peek() {
                            return Err(RLError::SimpleProgramError);
                        }

                        let mut add0;

                        match SIMPLEMATH(iter.next().unwrap()) {
                            Ok(num)  => add0 = num,
                            Err(err) => return Err(err),
                        }

                        while let Some(elem) = iter.next() {
                            let summand;

                            match SIMPLEMATH(elem) {
                                Ok(num)  => summand = num,
                                Err(err) => return Err(err),
                            }

                            add0 = add0 + summand
                        }
                        Ok(add0)
                    }

                    "-" => {
                        let mut iter = v.iter().peekable();

                        // Are there any arguments?!
                        if let None = iter.peek() {
                            return Err(RLError::SimpleProgramError);
                        }

                        // Define minuend first
                        let mut sub0;

                        // Check for possible type errors
                        match SIMPLEMATH(iter.next().unwrap()) {
                            Ok(num)  => sub0 = num,
                            Err(err) => return Err(err),
                        }

                        // If there is no subtrahend, change sign
                        if let None = iter.peek() {
                            sub0 = -1.0 * sub0;
                        }

                        // There exist subtrahends
                        while let Some(elem) = iter.next() {
                            let subtrahend;

                            // Check for possible type errors
                            match SIMPLEMATH(elem) {
                                Ok(num)  => subtrahend = num,
                                Err(err) => return Err(err),
                            }

                            sub0 = sub0 -subtrahend
                        }
                        Ok(sub0)
                    }

                    "*" => {
                        let mut iter = v.iter().peekable();

                        if let None = iter.peek() {
                            return Err(RLError::SimpleProgramError);
                        }

                        let mut mul0;

                        match SIMPLEMATH(iter.next().unwrap()) {
                            Ok(num)  => mul0 = num,
                            Err(err) => return Err(err),
                        }

                        while let Some(elem) = iter.next() {
                            let factor;

                            match SIMPLEMATH(elem) {
                                Ok(num)  => factor = num,
                                Err(err) => return Err(err),
                            }

                            mul0 = mul0 * factor
                        }
                    Ok(mul0)
                    }

                    "/" => {
                        let mut iter = v.iter().peekable();

                        if let None = iter.peek() {
                            return Err(RLError::SimpleProgramError);
                        }

                        let mut div0;

                        match SIMPLEMATH(iter.next().unwrap()) {
                            Ok(num)  => div0 = num,
                            Err(err) => return Err(err),
                        }

                        if let None = iter.peek() {
                            div0 = 1.0 / div0;
                        }

                        while let Some(elem) = iter.next() {
                            let divisor;

                            match SIMPLEMATH(elem) {
                                Ok(num)  => divisor = num,
                                Err(err) => return Err(err),
                            }

                            if divisor == 0.0 {
                                return Err(RLError::DivisionByZero);
                            } else {
                                div0 = div0 / divisor;
                        }
                    }
                    Ok(div0)
                    }
                    &_ => todo!(),
                }
            }

            SExpr::Nil => {
                let err = TypeError::new("NIL", "NUMBER");
                return Err(RLError::TypeError(err));
            }

            _ => Ok(0.0)
        }
    }
*/
