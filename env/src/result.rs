// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;

use crate::block::RLBlock;
use crate::env::RLEnvironment;
use crate::lambda::RLLambda;
use crate::named_lambda::RLNamedLambda;
use crate::symb::RLEnvSymbol;
use crate::dyn_var::RLDynVar;

use err::err::{RLError, TypeError};

use expr::atom::RLAtom;
use expr::bool::RLBool;
use expr::nil::RLNil;
// use expr::QuoteTrait;
use expr::expr::Expr;
use expr::qexpr::QExpr;
use expr::sexpr::SExpr;
use expr::string::RLString;
use expr::symb::RLSymbol;


use pars_symb::symbol::Symbol;

#[derive(Debug)]
pub enum RLResult {
    BlockRes(RLBlock),
    ReturnFromRes(RLBlock),

    ExprRes(Expr),
    QExprRes(QExpr),
    SExprRes(SExpr),

    NilRes(RLNil),
    BoolRes(RLBool),

    NumRes(f64),
    StrRes(String),
    StringRes(RLString),
    SymbolRes(RLEnvSymbol),
    SymbRes(Symbol),
    // SymbRes(RLSymbol),
    EnvRes(RLEnvironment),
    FuncRes(RLNamedLambda),
    LambdaRes(RLLambda),
    MacroRes(RLNamedLambda),
    NamedDsBindRes(RLBlock),
    VarRes(RLDynVar),
}

impl RLResult {
    pub fn get_sexpr_from_res(res: RLResult) -> Result<SExpr, RLError> {
        match res {
            RLResult::QExprRes(qexpr) => {
                let sexpr = expr::expr::qexpr_to_sexpr(qexpr);
                return Ok(sexpr);
            }

            RLResult::SExprRes(sexpr) => {
                return Ok(sexpr);
            }

            RLResult::NilRes(nil)     => {
               return Ok(SExpr::Nil(nil));
            }

            /*
            RLResult::BoolRes(bool)   => {}
            */

            RLResult::NumRes(num)     => {
                return Ok(SExpr::Atom(RLAtom::new(&num.to_string())));
            }

            RLResult::StrRes(string)  => {
                return Ok(SExpr::Atom(RLAtom::new(&string)));
            }

            RLResult::StringRes(rlstring) => {
                let string = rlstring.to_string();

                return Ok(SExpr::Atom(RLAtom::new(&string)));
            }

            RLResult::SymbRes(symb)   => {
                return Ok(SExpr::Symb(RLSymbol::new_with_symb(&symb)));
            }

            _ => unreachable!(),
        }
    }

    pub fn get_qexpr_res(res: RLResult) -> Result<QExpr, RLError> {
        if let RLResult::QExprRes(qexpr) = res {
            Ok(qexpr)
        } else {
              let err = TypeError::new("NIL", "NUMBER");
              return Err(RLError::TypeError(err));
        }
    }

    /*
    pub fn get_block_res(res: BlockRes)     -> {}
    pub fn get_return_from_res(res: ReturnFromRes) -> {}
    pub fn get_sexpr_res(res: SExprRes)     -> {}
    pub fn get_nil_res(res: NilRes)         -> {}
    pub fn get_bool_res(res: BoolRes)       -> {}
    pub fn get_num_res(res: f64)            -> {}
    pub fn get str_res(res: String)         -> {}
    pub fn get_string_res(res: RLString)    -> {}
    pub fn get_symbol_res(res: RLSymbol)    -> {}
    pub fn get_env_res(res: RLEnvironment)  -> {}
    pub fn get_func_res(res: RLNamedLambda) -> {}
    pub fn get_lambda_res(res: RLLambda)    -> {}
    pub fn get_macro_res(res: RLNamedLambda) -> {}
    pub fn get_var_res(res: RLDynVar)       -> {}
    */
}

impl fmt::Display for RLResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // println!("in fmt for SimpleResult");
        // println!("self: {:?}", self);
        match self {
            RLResult::ExprRes(expr) => {
                match expr {
                    Expr::QExpr(qexpr) => write!(f, "{}", qexpr),
                    Expr::SExpr(sexpr) => write!(f, "{}", sexpr),
                }
            }
            RLResult::BlockRes(block)      => write!(f, "{}", block),
            RLResult::ReturnFromRes(block) => write!(f, "{}", block),
            RLResult::QExprRes(qexpr)      => write!(f, "{}", qexpr),
            RLResult::SExprRes(sexpr)      => write!(f, "{}", sexpr),
            RLResult::NilRes(nil)          => write!(f, "{}", nil),
            RLResult::BoolRes(rlbool)      => write!(f, "{}", rlbool),
            RLResult::NumRes(f64)          => write!(f, "{}", f64),
            RLResult::StrRes(str)          => write!(f, "{}", str),
            RLResult::StringRes(rlstr)     => write!(f, "{}", rlstr),
            RLResult::SymbolRes(symb)      =>
                write!(f, "{}", symb.get_name().to_uppercase()),
            RLResult::SymbRes(symb)        =>
                write!(f, "{}", symb.name.to_uppercase()),
            RLResult::EnvRes(env)          => write!(f, "{}", env),
            RLResult::FuncRes(func)        => write!(f, "{}", func),
            RLResult::LambdaRes(lambda)    => write!(f, "{}", lambda),
            RLResult::NamedDsBindRes(block) => write!(f, "{}", block),
            RLResult::MacroRes(func)       => write!(f, "{}", func),
            RLResult::VarRes(var)          => write!(f, "{}", var),
        }
    }
}
