// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::hash_map::HashMap;

use env::env::RLEnvironment;
use env::env_trait::EnvTrait;
use env::named_lambda::RLNamedLambda;
use env::result::RLResult;
use env::symb::RLEnvSymbol;

use err::err::{RLError, UnboundVariableError, UndefinedFuncError};

use expr::atom::RLAtom;
use expr::sexpr::SExpr;

use hash::hash::RLHash;

pub struct SymbolFuncs {
}

impl SymbolFuncs {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                cl_pack_hash: &mut HashMap<String, RLEnvSymbol, RLHash>) {

        cl_pack_hash.insert("symbol-function".to_string(),
            RLEnvSymbol::new_named_lambda("SYMBOL-FUNCTION".to_string(),
                RLNamedLambda::new_func(
                    "SYMBOL-FUNCTION".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Return SYMBOL's current function definition. Settable with SETF."))),
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            SYMBOL_FUNCTION(&sexpr, env_ref)?) }))));

        cl_pack_hash.insert("symbol-package".to_string(),
            RLEnvSymbol::new_named_lambda("SYMBOL-PACKAGE".to_string(),
                RLNamedLambda::new_func(
                    "SYMBOL-PACKAGE".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Return SYMBOL's home package, or NIL if none."))),
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            SYMBOL_PACKAGE(&sexpr, env_ref)?) }))));

        cl_pack_hash.insert("symbol-value".to_string(),
            RLEnvSymbol::new_named_lambda("SYMBOL-VALUE".to_string(),
                RLNamedLambda::new_func(
                    "SYMBOL-VALUE".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Return SYMBOL's current bound value."))),
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            SYMBOL_VALUE(&sexpr, env_ref)?) }))));
    }
}

#[allow(non_snake_case)]
pub fn SYMBOL_FUNCTION(sexpr: &SExpr, env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from SYMBOL-FUNCTION");
    println!("");

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            match &*symb.name {
                "symbol-function" => {
                    if linked_list.is_empty() {
                        return Err(RLError::SimpleProgramError);
                    } else if linked_list.len() > 1 {
                        return Err(RLError::SimpleProgramError);
                    } else {
                        let mut symbol_name = "".to_string();

                        let symbol = linked_list.pop_front();

                        if let Some(SExpr::Symb(sym)) = symbol {
                            let symb_name = sym.get_symbol_name();

                            symbol_name = symb_name.to_lowercase();

                            println!("Symb - symbol_name: {:?}", symbol_name);
                            println!("");

                        } else if let Some(SExpr::Atom(atom)) = symbol {

                            let mut atom_string = atom.get_atom_string();

                            if env_ref.is_keyword(atom_string.clone()) {
                                let rest = atom_string.split_off(1);

                                symbol_name = rest;
                            } else {
                                symbol_name = atom_string.to_lowercase();
                            }
                        }

                        if let Some(rl_symbol) =
                            env_ref.get_symbol(&symbol_name) {

                            let mut named_lambda = rl_symbol
                                                       .named_lambda
                                                       .clone()
                                                       .unwrap();

                            named_lambda.package_formatter(false);

                            Ok(RLResult::FuncRes(named_lambda))
                        } else {
                            let err = UndefinedFuncError::new(&symbol_name);
                            return Err(RLError::UndefinedFuncError(err))
                        }
                    }
                }
                &_ => unreachable!()
            }
        }
        _ => unreachable!()
    }
}

#[allow(unused_assignments, non_snake_case)]
pub fn SYMBOL_PACKAGE(sexpr: &SExpr, env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from SYMBOL-PACKAGE");
    println!("");

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            match &*symb.name {
                "symbol-package" => {
                    if linked_list.is_empty() {
                        return Err(RLError::SimpleProgramError);
                    } else if linked_list.len() > 1 {
                        return Err(RLError::SimpleProgramError);
                    } else {
                        let mut symbol_name = "".to_string();

                        let symbol = linked_list.pop_front();

                        if let Some(SExpr::Symb(sym)) = symbol {
                            let symb_name = sym.get_symbol_name();

                            symbol_name = symb_name.to_lowercase();

                        } else if let Some(SExpr::Atom(atom)) = symbol {
                            let atom_string = atom.get_atom_string();

                            symbol_name = atom_string;
                        }

                        if let Some(rl_symbol) =
                            env_ref.get_symbol(&symbol_name) {

                            println!("rl_symbol: {:?}", rl_symbol);
                            println!("");

                            if let Some(mut named_lambda) =
                                rl_symbol.get_named_lambda() {

                                named_lambda.package_formatter(true);
                                Ok(RLResult::FuncRes(named_lambda))
                            } else if let Some(mut dyn_var) =
                                rl_symbol.get_dyn_var() {

                                dyn_var.package_formatter(true);
                                Ok(RLResult::VarRes(dyn_var))
                            } else {
                                Err(RLError::SimpleProgramError)
                            }

                        } else {
                            env_ref.package_formatter(true);
                            Ok(RLResult::EnvRes(env_ref.clone()))
                        }
                    }
                }
                &_ => unreachable!()
            }
        }
        _ => unreachable!()
    }
}

#[allow(non_snake_case)]
pub fn SYMBOL_VALUE(sexpr: &SExpr, env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from SYMBOL-VALUE");
    println!("");

    // let cl_pack = &mut env_ref.cl_package;
    // let cl_user_pack = &mut env_ref.cl_user_package;
    // let curr_pack: &mut RLPackage = env_ref.get_mut_current_package();

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            match &*symb.name {
                "symbol-value" => {
                   if linked_list.len() != 1 {
                        return Err(RLError::SimpleProgramError);
                    } else {
                         let symbol_name = match linked_list.pop_front() {
                            Some(SExpr::Atom(atom)) => {
                                let atom_string = atom.get_atom_string();

                                println!("atom_string: {:?}", atom_string);
                                println!("");

                                atom_string.to_uppercase()
                            }

                            Some(SExpr::Symb(symb)) => {
                                let symb_name = symb.get_symbol_name();

                                symb_name.to_uppercase()
                            }

                            _ => todo!(),
                        };

                        println!("symbol_name: {:?}", symbol_name);
                        println!("");

                        if let Some(symbol) = env_ref.get_symbol(
                            &symbol_name) {

                            if let Some(result_var) = symbol.dyn_var.clone() {
                                Ok(RLResult::VarRes(result_var))
                            } else {
                                let err = UnboundVariableError::new(&symbol_name);
                                Err(RLError::UnboundVariableError(err))
                            }
                        } else {
                            let err = UnboundVariableError::new(&symbol_name);
                            Err(RLError::UnboundVariableError(err))
                        }
                    }
                 }
                 &_ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

/*
System Class SYMBOL

Type KEYWORD

Function SYMBOLP

Function KEYWORDP

Function MAKE-SYMBOL

Function COPY-SYMBOL

Function GENSYM

Variable *GENSYM-COUNTER*

Function GENTEMP

Accessor SYMBOL-FUNCTION

Function SYMBOL-NAME

Function SYMBOL-PACKAGE

Accessor SYMBOL-PLIST

Accessor SYMBOL-VALUE

Accessor GET

Function REMPROP

Function BOUNDP

Function MAKUNBOUND

Function SET

Condition Type UNBOUND-VARIABLE
*/
