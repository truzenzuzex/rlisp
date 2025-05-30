// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::HashMap;

use hash::hash::RLHash;

use pars_symb::symbol::Symbol;
use pars_symb::token::Token;

pub struct EvalCompSymbs {
}

impl EvalCompSymbs {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                symbols: &mut HashMap<String, Token, RLHash>) {
        // special operators
        symbols.insert("eval-when".to_string(),
                       Token::Symb(Symbol::new("eval-when")));

        /*
        symbols.insert("load-time-value".to_string(),
                       Token::Symb(Symbol::new(
                           "load-time-value")));
        */

        symbols.insert("defmacro".to_string(),
                       Token::Symb(Symbol::new("defmacro")));

        symbols.insert("'".to_string(),
                       Token::Symb(Symbol::new("'")));

        symbols.insert("quote".to_string(),
                       Token::Symb(Symbol::new("quote")));

        symbols.insert("`".to_string(),
                      Token::Symb(Symbol::new("`")));

        /*
        symbols.insert("symbol-macrolet".to_string(),
                       Token::Symb(Symbol::new(
                           "symbol-macrolet")));
        */

        /*
        symbols.insert("locally".to_string(),
                       Token::Symb(Symbol::new("locally")));
        */

        /*
        symbols.insert("the".to_string(),
                       Token::Symb(Symbol::new("the".to_string())));
        */

        ////

        symbols.insert("lambda".to_string(),
                       Token::Symb(Symbol::new("lambda")));

        symbols.insert("eval".to_string(),
                       Token::Symb(Symbol::new("eval")));
    }
}
