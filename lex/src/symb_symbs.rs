// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::HashMap;

use hash::hash::RLHash;

use pars_symb::symbol::Symbol;
use pars_symb::token::Token;

pub struct SymbSymbs {
}

impl SymbSymbs {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                symbols: &mut HashMap<String, Token, RLHash>) {
        symbols.insert("symbol-function".to_string(),
                       Token::Symb(Symbol::new("symbol-function")));

        symbols.insert("symbol-package".to_string(),
                       Token::Symb(Symbol::new("symbol-package")));

        symbols.insert("symbol-value".to_string(),
                       Token::Symb(Symbol::new("symbol-value")));
    }
}
