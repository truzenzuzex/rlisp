// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;

use crate::comma::RLComma;

#[derive(Debug, Clone)]
pub struct RLNil {
    name: String,

    comma: Option<RLComma>,
}

impl RLNil {
    pub fn new() -> RLNil {
        let name = "NIL".to_string();

        let comma = None;

        RLNil {
            name,
            comma,
        }
    }

    pub fn get_nil_name(&self) -> String {
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
}

impl fmt::Display for RLNil {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            RLNil { name: nil, .. } => write!(f, "{}", nil),
        }
    }
}
