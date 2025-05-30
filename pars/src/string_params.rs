// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::HashMap;
use std::collections::LinkedList;

use crate::param_generator::RLParamGenerator;

use err::err::{ // ParseError,
               RLError};

use expr::sexpr::SExpr;

use hash::hash::RLHash;

use lex::lexer::RLLexer;

// use pars_symb::symbol::Symbol;
use pars_symb::token::Token;

pub struct StringParams {
}

impl StringParams {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                param_gens: &mut HashMap<String, RLParamGenerator, RLHash>) {
        param_gens.insert("dummy".to_string(),
            RLParamGenerator::new(|param_gen, lexer| {
                Ok::<LinkedList<SExpr>, RLError>(
                    dummy(param_gen, lexer)?) }));
    }
}

pub fn dummy(_param_gen: &mut RLParamGenerator, lexer: &mut RLLexer) ->
    Result<LinkedList<SExpr>, RLError> {

    /*

    */

    println!("Hello from DUMMY");
    println!("");

    let list = LinkedList::<SExpr>::new();

    loop {
        match lexer.next() {
            Token::Symb(_symb) => {}
            Token::Atom(_atom) => {}
            Token::Eof => { break; }
        }
    }

    /*
    param_gen.set_param_order(vec!["type_ident".to_string(),
                                   "string_list".to_string()]);
    */

    Ok(list)
}
