// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::HashMap;

use hash::hash::RLHash;

use pars_symb::symbol::Symbol;
use pars_symb::token::Token;

pub struct ListSymbs {
}

impl ListSymbs {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                symbols: &mut HashMap<String, Token, RLHash>) {
        // list operators
        symbols.insert("car".to_string(),
                       Token::Symb(Symbol::new("car")));

        symbols.insert("cdr".to_string(),
                       Token::Symb(Symbol::new("cdr")));

        symbols.insert("concatenate".to_string(),
                       Token::Symb(Symbol::new("concatenate")));

        symbols.insert("cons".to_string(),
                       Token::Symb(Symbol::new("cons")));

        symbols.insert("list".to_string(),
                       Token::Symb(Symbol::new("list")));
    }
}
