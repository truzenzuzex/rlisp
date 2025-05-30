// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::comma::RLComma;
// use crate::list::RLList;
// use crate::nil::RLNil;
use crate::qexpr::QExpr;
use crate::sexpr::SExpr;

#[derive(Debug, Clone)]
pub struct RLCons<T> {
    first: Box<T>,
    second: Box<T>,

    comma: Option<RLComma>,
}

impl RLCons<SExpr> {
    pub fn new_cons(first: SExpr,
                    second: SExpr) -> RLCons<SExpr> {
        let first = Box::new(first);

        let second = Box::new(second);

        let comma = None;

        RLCons {
            first,
            second,
            comma,
        }
    }

    pub fn new_with_comma(first: SExpr,
                          second: SExpr,
                          comma: Option<RLComma>) -> RLCons<SExpr> {
        let first = Box::new(first);

        let second = Box::new(second);

        let comma = comma;

        RLCons {
            first,
            second,
            comma,
        }
    }

    pub fn set_first(&mut self, sexpr: SExpr) {
        self.first = Box::new(sexpr);
    }

    pub fn set_second(&mut self, sexpr: SExpr) {
        self.second = Box::new(sexpr);
    }

    pub fn car(&self) -> SExpr {
        *self.first.clone()
    }

    pub fn cdr(&self) -> SExpr {
        *self.second.clone()
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

impl RLCons<QExpr> {
    pub fn new_cons(first: QExpr,
                    second: QExpr) -> RLCons<QExpr> {
        let first = Box::new(first);

        let second = Box::new(second);

        let comma = None;

        RLCons {
            first,
            second,
            comma,
        }
    }

    pub fn new_with_comma(first: QExpr,
                          second: QExpr,
                          comma: Option<RLComma>) -> RLCons<QExpr> {
        let first = Box::new(first);

        let second = Box::new(second);

        let comma = comma;

        RLCons {
            first,
            second,
            comma,
        }
    }

    pub fn set_first(&mut self, qexpr: QExpr) {
        self.first = Box::new(qexpr);
    }

    pub fn set_second(&mut self, qexpr: QExpr) {
        self.second = Box::new(qexpr);
    }

    pub fn car(&self) -> QExpr {
        *self.first.clone()
    }

    pub fn cdr(&self) -> QExpr {
        *self.second.clone()
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
