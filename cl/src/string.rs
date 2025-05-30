// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::hash_map::HashMap;
// use std::ops::Not;

// use env::env::RLEnvironment;
// use env::env_trait::EnvTrait;
use env::named_lambda::RLNamedLambda;
use env::result::RLResult;
use env::symb::RLEnvSymbol;
// use env::var::RLVar;

use err::err::{RLError};

use expr::nil::RLNil;
use expr::sexpr::SExpr;

use hash::hash::RLHash;

pub struct StrFuncs {
}

impl StrFuncs {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                cl_pack_hash: &mut HashMap<String, RLEnvSymbol, RLHash>) {
                           // func_hash: &mut HashMap<String,
        // Box<dyn FnMut(SExpr) -> Result<RLResult, RLError>>>) {

        // cl_pack_hash
        cl_pack_hash.insert("dummy".to_string(),
            RLEnvSymbol::new_named_lambda("DUMMY".to_string(),
                RLNamedLambda::new_func(
                    "DUMMY".to_string(),
                    None,
                    "COMMON-LISP".to_string(),
                    Some(|sexpr| {
                        Ok::<RLResult, RLError>(
                            DUMMY(&sexpr)?) }),
                    None)));
    }
}

#[allow(non_snake_case)]
pub fn DUMMY(sexpr: &SExpr) ->
    Result<RLResult, RLError> {

    /*
    Syntax:

    */

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let _linked_list = ll.clone();

            match &*symb.name {
                "dummy" => {

                    return Ok(RLResult::NilRes(RLNil::new()));
                }
                &_ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
