// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::HashMap;

use hash::hash::RLHash;

use pars_symb::symbol::Symbol;
use pars_symb::token::Token;

pub struct DataControlSymbs {
}

impl DataControlSymbs {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                symbols: &mut HashMap<String, Token, RLHash>) {
        // special operators
        /*
        symbols.insert("flet".to_string(),
                       Token::Symb(Symbol::new("flet")));
        */

        /*
        symbols.insert("labels".to_string(),
                       Token::Symb(Symbol::new("labels")));
        */

        /*
        symbols.insert("macrolet".to_string(),
                       Token::Symb(Symbol::new("macrolet")));
        */

        symbols.insert("funcall".to_string(),
                       Token::Symb(Symbol::new("funcall")));

        symbols.insert("function".to_string(),
                       Token::Symb(Symbol::new("function")));

        /*
        symbols.insert("let".to_string(),
                       Token::Symb(Symbol::new("let")));
        */

        /*
        symbols.insert("let*".to_string(),
                       Token::Symb(Symbol::new("let*")));
        */

        symbols.insert("progv".to_string(),
                       Token::Symb(Symbol::new("progv")));

        /*
        symbols.insert("setq".to_string(),
                       Token::Symb(Symbol::new("setq")));
        */

        symbols.insert("block".to_string(),
                       Token::Symb(Symbol::new("block")));

        /*
        symbols.insert("catch".to_string(),
                       Token::Symb(Symbol::new("catch")));
        */

        /*
        symbols.insert("go".to_string(),
                       Token::Symb(Symbol::new("go")));
        */

        symbols.insert("return-from".to_string(),
                       Token::Symb(Symbol::new("return-from")));

        /*
        symbols.insert("tagbody".to_string(),
                       Token::Symb(Symbol::new("tagbody")));
        */

        /*
        symbols.insert("throw".to_string(),
                       Token::Symb(Symbol::new("throw")));
        */

        /*
        symbols.insert("unwind-protect".to_string(),
                       Token::Symb(Symbol::new("unwind-protect")));
        */

        /*
        symbols.insert("if".to_string(),
                       Token::Symb(Symbol::new("if")));
        */

        /*
        symbols.insert("multiple-value-call".to_string(),
                       Token::Symb(Symbol::new("multiple-value-call")));
        */

        /*
        symbols.insert("multiple-value-prog1".to_string(),
                       Token::Symb(Symbol::new("multiple-value-prog1")));
        */

        symbols.insert("progn".to_string(),
                       Token::Symb(Symbol::new("progn")));

        ////

        symbols.insert("defun".to_string(),
                       Token::Symb(Symbol::new("defun")));

        symbols.insert("defparameter".to_string(),
                       Token::Symb(Symbol::new("defparameter")));

        symbols.insert("defvar".to_string(),
                       Token::Symb(Symbol::new("defvar")));
    }
}
