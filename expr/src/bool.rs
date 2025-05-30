// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;

use crate::t::RLT;
use crate::nil::RLNil;

// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(Debug, Clone)]
pub enum RLBool {
    T(RLT),
    Nil(RLNil),
}

/*
impl RLBool {
}
*/

impl fmt::Display for RLBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RLBool::T(t) => write!(f, "{}", t),
            RLBool::Nil(nil) => write!(f, "{}", nil),
        }
    }
}
