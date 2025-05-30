// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::HashMap;

use hash::hash::RLHash;

use pars_symb::symbol::Symbol;
use pars_symb::token::Token;

pub struct CLSymbs {
}

impl CLSymbs {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                symbols: &mut HashMap<String, Token, RLHash>) {
        // lisp parenthesis
        symbols.insert("(".to_string(),
                       Token::Symb(Symbol::new("(")));

        symbols.insert(")".to_string(),
                       Token::Symb(Symbol::new(")")));

        // true
        symbols.insert("t".to_string(),
                       Token::Symb(Symbol::new("t")));

        // backquote
        symbols.insert("backquote".to_string(),
                       Token::Symb(Symbol::new("backquote")));

        // comma
        symbols.insert(",".to_string(),
                       Token::Symb(Symbol::new(",")));

        // comma_at
        symbols.insert(",@".to_string(),
                      Token::Symb(Symbol::new(",@")));

        // lambda-list keywords
        symbols.insert("&allow-other-keys".to_string(),
                       Token::Symb(Symbol::new("&allow-other-keys")));

        symbols.insert("&aux".to_string(),
                       Token::Symb(Symbol::new("&aux")));

        symbols.insert("&key".to_string(),
                       Token::Symb(Symbol::new("&key")));

        symbols.insert("&optional".to_string(),
                       Token::Symb(Symbol::new("&optional")));

        symbols.insert("&rest".to_string(),
                       Token::Symb(Symbol::new("&rest")));
    }
}
