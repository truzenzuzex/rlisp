// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;
use std::collections::HashMap;

use crate::symbol::Symbol;

use hash::hash::RLHash;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Atom(String),
    Symb(Symbol),
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Atom(s)   => write!(f, "{}", s),
            Token::Symb(s)    => write!(f, "{}", s.name),
            Token::Eof       => write!(f, "Eof"),
        }
    }
}

pub fn make_token_hash_map() -> HashMap<String, Token, RLHash> {
    HashMap::with_hasher(RLHash { })
}
