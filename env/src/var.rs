// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;
use std::collections::HashMap;

// use crate::lambda::RLLambda;
// use crate::named_lambda::RLNamedLambda;

use expr::bool::RLBool;
use expr::nil::RLNil;
use expr::qexpr::QExpr;
use expr::sexpr::SExpr;

use hash::hash::RLHash;

    /* QExpr types
    Atom(String),
    Nil(RLNil),
    // RLBool(bool),
    Symb(Symbol),
    QList(LinkedList<QExpr>),
    Cons(Box<QExpr>, Box<QExpr>),
    */

    /* SExpr types
    Atom(RLAtom),
    QCons(Box<QExpr>, Box<QExpr>),

    // type necessary for zipper replacements?!
    QList(LinkedList<QExpr>),

    SList(RLList),
    Symb(RLSymbol),
    Cons(Symbol, LinkedList<SExpr>),
    Nil(RLNil),
    // Bool(bool),
    Dummy,
    */

#[derive(Debug, Clone)]
pub enum RLVar {
    // BlockVar(RLBlock),
    QListVar(QExpr),
    SListVar(SExpr),
    // SConsVar(SExpr),
    // SExprVar(SExpr),
    SAtomVar(SExpr),
    NilVar(RLNil),
    BoolVar(RLBool),

    // NamedLambdaVar(RLNamedLambda),
    // LambdaVar(RLLambda),

    // NumVar(f64),
    // StrVar(String),
    // FuncVar(RLNamedLambda),
}

impl RLVar {
    pub fn var_to_sexpr(&mut self) -> SExpr {
        let result: SExpr;

        match self {
            RLVar::SAtomVar(SExpr::Atom(atom)) => {
                result = SExpr::Atom(atom.clone());
            }

            RLVar::NilVar(nil) => {
                result = SExpr::Nil(nil.clone());
            }

            RLVar::SListVar(SExpr::SList(slist)) => {
                result = SExpr::SList(slist.clone());
            }

            _ => todo!(),
        }
        result
    }
}

pub fn make_rlvar_hash_map() -> HashMap<String, RLVar, RLHash> {
    HashMap::with_hasher(RLHash { })
}

/*
impl Clone for RLVar {
}
*/

impl fmt::Display for RLVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // println!("in fmt for SimpleVar");
        // println!("self: {:?}", self);
        match self {
            // RLVar::BlockVar(block) => write!(f, "{}", block),
            RLVar::QListVar(qexpr) => write!(f, "{}", qexpr),
            RLVar::SListVar(sexpr) => write!(f, "{}", sexpr),
            // RLVar::SConsVar(sexpr) => write!(f, "{}", sexpr),
            // RLVar::SExprVar(sexpr) => write!(f, "{}", sexpr),

            RLVar::NilVar(nil)     => write!(f, "{}", nil),
            RLVar::BoolVar(rlbool) => write!(f, "{}", rlbool),

            // RLVar::NamedLambdaVar(_named_lambda) => todo!(),
            // RLVar::LambdaVar(_lambda) => todo!(),

            RLVar::SAtomVar(atom)  => write!(f, "{}", atom),
            // RLVar::NumVar(num)  => write!(f, "{}", num),
            // RLVar::StrVar(str)  => write!(f, "{}", str),
            // RLVar::FuncVar(n_l)    => write!(f, "{}", n_l),
        }
    }
}
