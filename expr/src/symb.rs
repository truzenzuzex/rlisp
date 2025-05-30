// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;

use crate::comma::RLComma;

use pars_symb::symbol::Symbol;

#[derive(Debug, Clone)]
pub struct RLSymbol {
    symbol: Symbol,

    comma: Option<RLComma>,
}

impl RLSymbol {
    pub fn new_with_symb(symbol: &Symbol) -> RLSymbol {
        let symbol = symbol.clone();

        let comma = None;

        RLSymbol {
            symbol,
            comma,
        }
    }

    pub fn new_with_str(str: &str) -> RLSymbol {
        let symbol = Symbol::new(str);

        let comma = None;

        RLSymbol {
            symbol,
            comma,
        }
    }

    pub fn get_symbol_name(&self) -> String {
        self.symbol.name.clone()
    }

    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }

    pub fn set_comma(&mut self, comma: &RLComma) {
        self.comma = Some(comma.clone());
    }

    pub fn get_comma(&self) -> Option<RLComma> {
        self.comma.clone()
    }

    pub fn has_comma(&self) -> bool {
        if let Some(_comma) = &self.comma {
            return true
        } else {
            return false;
        }
    }
}



impl fmt::Display for RLSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // println!("in fmt for RLSymbol");
        // println!("self: {:?}", self);

        match self {
            RLSymbol { symbol: name, .. } => write!(f, "{}", name),
        }
    }
}
