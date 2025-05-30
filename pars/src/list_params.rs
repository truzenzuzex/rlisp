// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::HashMap;
// use std::collections::LinkedList;

use crate::param_generator::RLParamGenerator;

// use err::err::{RLError, ParseError};

// use expr::sexpr::SExpr;

use hash::hash::RLHash;

// use lex::lexer::RLLexer;

// use pars_symb::symbol::Symbol;
// use pars_symb::token::Token;


pub struct ListParams {
}

impl ListParams {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                _param_gens: &mut HashMap<String, RLParamGenerator, RLHash>) {

        /*
        param_gens.insert("concatenate".to_string(),
            RLParamGenerator::new(|param_gen, lexer| {
                Ok::<LinkedList<SExpr>, RLError>(
                    concatenate(param_gen, lexer)?) }));
        */
    }
}

/*
pub fn concatenate(param_gen: &mut RLParamGenerator, lexer: &mut RLLexer) ->
    Result<LinkedList<SExpr>, RLError> {

    /*
    Syntax:
    */

    println!("Hello from CONCATENATE-PARAMS");
    println!("");

    let mut list = LinkedList::<SExpr>::new();

    // match function name or lambda expr
    match lexer.next() {
        Token::Atom(atom) => {

        }

        Token::Symb(symb) => {

        }

        Token::Eof => {

        }
    }
    Ok(list)
}
*/
