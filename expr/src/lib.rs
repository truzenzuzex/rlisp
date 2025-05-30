// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::sexpr::SExpr;

use err::err::RLError;

pub mod atom;
pub mod bool;
pub mod comma;
pub mod cons;
pub mod expr;
pub mod list;
pub mod nil;
pub mod qexpr;
pub mod sexpr;
pub mod symb;
pub mod string;
pub mod t;

pub trait QuoteTrait {
    fn quote(sexpr: SExpr) -> Result<Self, RLError> where Self: Sized;

    // fn backquote(sexpr: SExpr) -> Result<SExpr, RLError>;
}





/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
