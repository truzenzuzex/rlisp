// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;

use crate::comma::RLComma;
use crate::string::RLString;

#[derive(Debug, Clone)]
pub struct RLAtom {
    name: String,

    comma: Option<RLComma>,
}

impl RLAtom {
    pub fn new(name: &str) -> RLAtom {
        let name = name.to_string();

        let comma = None;

        RLAtom {
            name,
            comma,
        }
    }

    pub fn get_atom_string(&self) -> String {
        self.name.clone()
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

    pub fn is_rlstring_atom(&self) -> bool {
        if self.name.starts_with("\"") &&
           self.name.ends_with("\"") {

           return true;
        } else {
            return false;
        }
    }

    pub fn get_atom_rlstring(&self) -> RLString {
        let string = self.name.clone();

        RLString::new(&string)
    }

    pub fn set_name_to_uppercase(&mut self) {
        self.name = self.name.to_uppercase();
    }
}

impl fmt::Display for RLAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // println!("in fmt for RLAtom");
        // println!("self: {:?}", self);

        match self {
            RLAtom { name: atom, .. } => write!(f, "{}", atom),
        }
    }
}
