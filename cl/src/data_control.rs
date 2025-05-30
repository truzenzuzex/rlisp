// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::LinkedList;
use std::collections::hash_map::HashMap;
use std::ops::Not;

use env::block::RLBlock;
use env::dyn_var::RLDynVar;
use env::env::RLEnvironment;
use env::env_trait::EnvTrait;
use env::named_lambda::RLNamedLambda;
use env::pack::RLPackage;
use env::result::RLResult;
use env::symb::RLEnvSymbol;
use env::var::RLVar;

use err::err::RLError;
use err::err::{BlockError,
               ReturnFromError,
               UndefinedFuncError};

use expr::atom::RLAtom;
use expr::nil::RLNil;
use expr::sexpr::SExpr;
use expr::string::RLString;

use hash::hash::RLHash;

use pars_symb::symbol::Symbol;

pub struct DataControlFuncs {
}

impl DataControlFuncs {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                cl_pack_hash: &mut HashMap<String, RLEnvSymbol, RLHash>) {

        cl_pack_hash.insert("funcall".to_string(),
            RLEnvSymbol::new_named_lambda("FUNCALL".to_string(),
                RLNamedLambda::new_func(
                    "FUNCALL".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Call FUNCTION with the given ARGUMENTS."))),
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            FUNCALL(&sexpr, env_ref)?) }))));

        cl_pack_hash.insert("function".to_string(),
            RLEnvSymbol::new_named_lambda("FUNCTION".to_string(),
                RLNamedLambda::new_func(
                    "FUNCTION".to_string(),
                    Some(SExpr::Atom(RLAtom::new("FUNCTION name

Return the lexically apparent definition of the function NAME. NAME may also
be a lambda expression."))),
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            FUNCTION(&sexpr, env_ref)?) }))));

        cl_pack_hash.insert("defun".to_string(),
            RLEnvSymbol::new_named_lambda("DEFUN".to_string(),
                RLNamedLambda::new_func(
                    "DEFUN".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Define a function at top level."))),
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            DEFUN(&sexpr, env_ref)?) }))));

        cl_pack_hash.insert("defparameter".to_string(),
            RLEnvSymbol::new_named_lambda("DEFPARAMETER".to_string(),
                RLNamedLambda::new_func(
                    "DEFPARAMETER".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Define a parameter that is not normally changed by the program,
  but that may be changed without causing an error. Declare the
  variable special and sets its value to VAL, overwriting any
  previous value. The third argument is an optional documentation
  string for the parameter."))),
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            DEFPARAMETER(&sexpr, env_ref)?) }))));

        cl_pack_hash.insert("defvar".to_string(),
            RLEnvSymbol::new_named_lambda("DEFVAR".to_string(),
                RLNamedLambda::new_func(
                    "DEFVAR".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Define a special variable at top level. Declare the variable
  SPECIAL and, optionally, initialize it. If the variable already has a
  value, the old value is not clobbered. The third argument is an optional
  documentation string for the variable."))),
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            DEFVAR(&sexpr, env_ref)?) }))));

        cl_pack_hash.insert("progv".to_string(),
            RLEnvSymbol::new_named_lambda("PROGV".to_string(),
                RLNamedLambda::new_func(
                    "PROGV".to_string(),
                    None,
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            PROGV(&sexpr, env_ref)?) }))));

        cl_pack_hash.insert("setq".to_string(),
            RLEnvSymbol::new_named_lambda("SETQ".to_string(),
                RLNamedLambda::new_func(
                    "SETQ".to_string(),
                    None,
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            SETQ(&sexpr, env_ref)?) }))));

        cl_pack_hash.insert("block".to_string(),
            RLEnvSymbol::new_named_lambda("BLOCK".to_string(),
                RLNamedLambda::new_func(
                    "BLOCK".to_string(),
                    Some(SExpr::Atom(RLAtom::new("BLOCK name form*

Evaluate the FORMS as a PROGN. Within the lexical scope of the body,
RETURN-FROM can be used to exit the form."))),
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            BLOCK(&sexpr, env_ref)?) }))));

        cl_pack_hash.insert("return-from".to_string(),
            RLEnvSymbol::new_named_lambda("RETURN_FROM".to_string(),
                RLNamedLambda::new_func(
                    "RETURN_FROM".to_string(),
                    Some(SExpr::Atom(RLAtom::new("RETURN-FROM name value

Evaluate the VALUE, returning its values from the lexically enclosing
block NAME. This is constrained to be used only within the dynamic
extent of the block."))),
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            RETURN_FROM(&sexpr, env_ref)?) }))));

        cl_pack_hash.insert("progn".to_string(),
            RLEnvSymbol::new_named_lambda("PROGN".to_string(),
                RLNamedLambda::new_func(
                    "PROGN".to_string(),
                    Some(SExpr::Atom(RLAtom::new("PROGN form*

Evaluates each FORM in order, returning the values of the last form. With no
forms, returns NIL."))),
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            PROGN(&sexpr, env_ref)?) }))));
    }
}

/*
Function APPLY
*/

#[allow(non_snake_case)]
pub fn FUNCALL(sexpr: &SExpr, env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from FUNCALL");
    println!("");

    let curr_pack = env_ref.get_mut_current_package();

    let curr_pack_name = curr_pack.get_name();

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            println!("linked_list: {:?}", linked_list);
            println!("");

            match &*symb.name {
                "funcall" => {

                    /*
                    for _n in 0..linked_list.len() {
                        let item = linked_list.pop_front().expect("REASON");

                        println!("funcall - item: {:?}", item);
                        println!("");
                    }
                    */


                    let first_param = linked_list.pop_front();

                    if let Some(SExpr::Atom(ref atom)) = first_param {
                        // let symbol: Option<RLSymbol>;

                        let atom_string = atom.get_atom_string();

                        match &*atom_string.as_str() {
                            "ADD" => {
                                let symb = Symbol::new("+");

                                return Ok(RLResult::SExprRes(
                                    SExpr::Cons(symb, linked_list)));
                            }

                            "MINUS" => {
                                let symb = Symbol::new("-");

                                return Ok(RLResult::SExprRes(
                                    SExpr::Cons(symb, linked_list)));
                            }

                            "MUL" => {
                                let symb = Symbol::new("*");

                                return Ok(RLResult::SExprRes(
                                    SExpr::Cons(symb, linked_list)));
                            }

                            "DIV" => {
                                let symb = Symbol::new("/");

                                return Ok(RLResult::SExprRes(
                                    SExpr::Cons(symb, linked_list)));
                            }

                            _ => {
                                let err_description = format!(
                                    "The function {}::{} is undefined.",
                                        curr_pack_name,
                                        atom_string.to_uppercase());

                                let err = UndefinedFuncError::new(
                                    &err_description);

                                return Err(RLError::UndefinedFuncError(err))
                            }
                        }
                    }

                    if let Some(SExpr::Lambda(atom)) = first_param {
                        let atom_string = atom.get_atom_string();

                        println!("atom_string: {:?}", atom_string);
                        println!("");

                        if let Some(lambda) =
                            curr_pack.get_lambda(&atom_string) {

                            println!("lambda: {:?}", lambda);
                            println!("");

                            if let Some(_name) =
                                lambda.get_named_lambda_name() {

                                let mut l_params =
                                    lambda.get_cleaned_req_params();

                                println!("l_params: {:?}", l_params);
                                println!("");

                                for _n in 0..l_params.len() {

                                    let item =
                                        l_params.pop_front().unwrap();

                                    if let Some(value) =
                                        linked_list.pop_front() {

                                        match item {
                                            SExpr::Atom(param) => {
                                                let param_string =
                                                    param.get_atom_string();

                                                let var =
                                                    RLVar::SAtomVar(value);

                                                lambda.add_dyn_env_var(
                                                    param_string, var);
                                            }

                                            _ => todo!(),
                                        }
                                    } else {
                                        return Err(
                                            RLError::SimpleProgramError);
                                    }
                                }
                            } else {
                                let params = lambda.get_parameters();

                                if let SExpr::SList(slist) =
                                    params.get_required_params() {

                                    let list = slist.get_linked_list();

                                    println!("parameter list: {:?}", list);
                                    println!("");

                                    for item in list {
                                        if let SExpr::Atom(param) = item {

                                            let var = RLVar::SAtomVar(
                                                linked_list.pop_front()
                                                           .expect("REASON"));

                                            lambda.add_dyn_env_var(
                                                param.to_string(), var);
                                        }
                                    }
                                }
                            }

                            let l_hash_map = lambda.get_ref_dyn_env_lambda();

                            println!("l_hash_map: {:?}", l_hash_map);
                            println!("");

                            return Ok(RLResult::LambdaRes(lambda.clone()));
                        } else {
                            return Err(RLError::SimpleProgramError);
                        }
                    }

                    if let Some(SExpr::Symb(symb)) = first_param {

                        let symbol_string = symb.get_symbol_name();

                        if let Some(pack_symbol) =
                            env_ref.get_symbol(&symbol_string) {

                            println!("pack_symbol: {:?}", pack_symbol);
                            println!("");

                            if pack_symbol.get_is_macro() == true {
                                let err_description = format!(
                                    "{}::{} is a macro, not a function",
                                        curr_pack_name,
                                        symbol_string.to_uppercase());

                                let err = UndefinedFuncError::new(
                                    &err_description);

                                return Err(RLError::UndefinedFuncError(err))
                            } else {

                                let pars_symbol = symb.get_symbol();

                                println!("SYMB - FIRSTPARAM");
                                println!("");

                                return Ok(RLResult::SExprRes(
                                    SExpr::Cons(pars_symbol, linked_list)))
                            }
                        } else {
                            return Err(RLError::SimpleProgramError);
                        }
                    } else {
                            return Err(RLError::SimpleProgramError);
                    }
                }
                &_ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

// special operator function
#[allow(non_snake_case)]
pub fn FUNCTION(sexpr: &SExpr, env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    /*
    (lambda lambda-list [[declaration* | documentation]] form*)
  ==  (function (lambda lambda-list [[declaration* | documentation]] form*))
  ==  #'(lambda lambda-list [[declaration* | documentation]] form*)
    */

    println!("Hello from FUNCTION");
    println!("");

    let curr_pack = env_ref.get_mut_current_package();

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            match &*symb.name {
                "function" => {
                    if linked_list.is_empty() {
                        return Err(RLError::SimpleProgramError);
                    } else {

                        let item = linked_list.pop_front();

                        println!("function - next item: {:?}", item);
                        println!("");

                        match item {
                            Some(SExpr::Lambda(atom)) => {
                                let atom_string = atom.get_atom_string();

                                if let Some(lambda) =
                                    curr_pack.get_lambda(&atom_string) {

                                    println!("lambda: {:?}", lambda);
                                    println!("");

                                    return Ok(RLResult::LambdaRes(
                                        lambda.clone()));
                                } else {
                                    return Err(RLError::SimpleProgramError);
                                }
                            }

                            /*
                            Some(SExpr::Atom(atom)) => {
                                let atom_string = atom.get_atom_string();

                                if let Some(&mut ref lambda) =
                                    curr_pack.get_lambda(&atom_string) {

                                    println!("lambda: {:?}", lambda);
                                    println!("");

                                    return Ok(RLResult::LambdaRes(
                                        lambda.clone()));
                                }

                                if let Some(&mut ref mut symbol) =
                                    env_ref.get_symbol(&atom_string) {

                                    if let Some(named_lambda) =
                                        symbol.get_named_lambda() {

                                        return Ok(RLResult::FuncRes(
                                            named_lambda.clone()));
                                    } else {
                                        return Err(
                                            RLError::SimpleProgramError);
                                    }
                                } else {
                                    return Err(RLError::SimpleProgramError);
                                }
                            }
                            */

                            Some(SExpr::Symb(symb)) => {
                                let symb_name = symb.get_symbol_name();

                                if let Some(&mut ref mut symbol) =
                                    env_ref.get_symbol(&symb_name) {

                                    if let Some(named_lambda) =
                                        symbol.get_named_lambda() {

                                        println!("named_lambda: {:?}",
                                            named_lambda);
                                        println!("");

                                        return Ok(RLResult::FuncRes(
                                            named_lambda.clone()));
                                    } else {
                                        return Err(
                                            RLError::SimpleProgramError);
                                    }
                                } else {
                                    return Err(RLError::SimpleProgramError);
                                }
                            }

                            None => {
                                return Err(RLError::SimpleProgramError);
                            }

                            _ => unreachable!(),
                        }
                    }
                }
                &_ => unreachable!(),
            }
        }
        _ => unreachable!()
    }
}

// Macro DEFUN
#[allow(non_snake_case, unused_assignments)]
pub fn DEFUN(sexpr: &SExpr, env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    /*
    Syntax:
    defun function-name lambda-list [[declaration* | documentation]] form*
    */

    println!("Hello from DEFUN");
    println!("");

    let curr_pack = &mut env_ref.get_mut_current_package();

    let curr_pack_name = curr_pack.get_name();

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            println!("linked_list: {:?}", linked_list);

            match &*symb.name {
                "defun" => {
                    if linked_list.is_empty() {
                        return Err(RLError::SimpleProgramError);
                    } else {
                        // let mut name = RLString::new("dummy");
                        let mut name = "dummy".to_string();

                        if let Some(SExpr::Symb(symb)) =
                            linked_list.pop_front() {

                            let symb_name = symb.get_symbol_name();

                                // name.set(&atom);
                                name = symb_name;

                                println!("{}", name);
                        } else {
                            // Which RLError?!
                            return Err(RLError::SimpleProgramError);
                        }

                        let mut parameters = SExpr::Nil(RLNil::new());

                        if let Some(SExpr::SList(mut list)) =
                            linked_list.pop_front() {
                                list.pop_front();
                                list.pop_back();

                                parameters = SExpr::SList(list);

                                println!("parameters: {:?}", parameters);
                                println!("");
                        } else {
                            // which RLError?!
                            return Err(RLError::SimpleProgramError);
                        }

                        let opt_docstring: Option<SExpr>;

                        let mut lambda_string = "".to_string();

                        let next_param = linked_list.pop_front();

                        match next_param {
                            Some(SExpr::Nil(_nil)) => {
                                // opt_docstring = Some(SExpr::Nil(nil));
                                opt_docstring = None;
                            }

                            Some(SExpr::Atom(atom)) => {
                                opt_docstring = Some(SExpr::Atom(atom));
                            }

                            Some(SExpr::Lambda(atom)) => {
                                opt_docstring = None;

                                    lambda_string =
                                        atom.get_atom_string();

                                    println!("lambda_string: {:?}",
                                        lambda_string);
                                    println!("")
                            }

                            Some(SExpr::SForm(form)) => {
                                opt_docstring = None;

                                linked_list.push_front(SExpr::SForm(form));
                            }

                            Some(SExpr::SToken(token)) => {
                                opt_docstring = None;

                                linked_list.push_front(
                                    SExpr::SToken(token));
                            }

                            Some(SExpr::Cons(symb, ll)) => {
                                opt_docstring = None;

                                linked_list.push_front(
                                    SExpr::Cons(symb, ll));
                            }

                            _ => unreachable!(),
                        }

                        let mut named_lambda = RLNamedLambda::new_func(
                            name.clone().to_uppercase(),
                            opt_docstring.clone(),
                            curr_pack_name,
                            None,
                            Some(|sexpr, env_ref| {
                                Ok::<RLResult, RLError>(
                                    RLFUNC(&sexpr, env_ref)?) }));

                        if lambda_string.eq("").not() {
                            println!("in lambda_string.eq.(\"\").not()");
                            println!("");

                            if let Some(lambda) =
                                curr_pack.get_lambda(&lambda_string) {

                                println!("lambda: {:?}", lambda);
                                println!("");

                                named_lambda.set_lambda(lambda);

                                curr_pack.delete_lambda(lambda_string);
                             }

                        } else {

                            let mut block = RLBlock::new(name.clone()
                                                             .to_uppercase());

                            block.set_progn_ll(&linked_list);

                            named_lambda.set_block(&block);
                        }

                        let _ = named_lambda.init_parameters(parameters);

                        let symbol = RLEnvSymbol::new_named_lambda(
                            name.clone().to_uppercase(),
                            named_lambda.clone());

                        curr_pack.add_symbol(name.clone(), symbol);

                        println!("last of defun");
                        println!("named_lambda: {:?}", named_lambda);
                        println!("");

                        Ok(RLResult::StrRes(name.clone().to_uppercase()))
                    }
                }

                &_ => unreachable!(),
            }
        }
        _ => unreachable!()
    }
}

#[allow(non_snake_case)]
pub fn RLFUNC(sexpr: &SExpr, env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from RLFUNC");
    println!("");

    let curr_pack = &mut env_ref.get_mut_current_package();

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            let symb_string = &*symb.name;

            println!("{:?}", symb_string);

            let mut symbol = curr_pack.get_symbol(
                &symb_string.to_string()).unwrap()
                                         .clone();

            let mut named_lambda = symbol.named_lambda.clone().unwrap();

            println!("symbol: {:?}", symbol);
            println!("");

            let mut parameters = named_lambda.get_parameters();

            if let Some(SExpr::SList(list)) =
                parameters.get_req_params_as_option() {

                let ll = list.get_linked_list();

                println!("required_params: {:?}", ll);
                println!("");

                for item in ll {
                    if let SExpr::Atom(atom) = item {
                        let var = RLVar::SAtomVar(linked_list.pop_front()
                                                             .expect("REASON"));
                        let param = atom.get_atom_string();

                        named_lambda.add_dyn_env_var(param, var);
                    }
                }

                let hash_map = named_lambda.get_ref_dyn_env();

                println!("hash_map: {:?}", hash_map);
                println!("");

                symbol.set_named_lambda(&named_lambda);

                println!("symbol: {:?}", symbol);
                println!("");

                if let Some(lambda) = named_lambda.get_lambda() {
                    let lambda_id = lambda.get_id();

                    println!("lambda_id: {:?}", lambda_id);
                    println!("");

                    curr_pack.add_lambda(lambda.get_id(),
                                         lambda.clone());

                    Ok(RLResult::LambdaRes(lambda))
                } else{
                    Ok(RLResult::FuncRes(named_lambda))
                }
            } else {
                // which RLError?!
                return Err(RLError::SimpleProgramError)
            }
        }
        _ => unreachable!(),
    }
}

#[allow(non_snake_case)]
pub fn DEFPARAMETER(sexpr: &SExpr, env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from DEFPARAMETER");
    println!("");

    let curr_pack: &mut RLPackage = env_ref.get_mut_current_package();
    let pack_name = curr_pack.get_name();

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            match &*symb.name {
                "defparameter" => {
                    println!("linked_list.len: {}", linked_list.len());

                    if linked_list.len() < 2 {
                        return Err(RLError::SimpleProgramError);
                    } else if linked_list.len() > 3  {
                        return Err(RLError::SimpleProgramError);
                    } else {
                        let mut var_name = "dummy".to_string();

                        if let Some(SExpr::Atom(atom)) =
                            linked_list.pop_front() {

                            let atom_name = atom.get_atom_string();

                            var_name = atom_name.to_uppercase();

                            println!("var_name: {:?}", var_name);
                            println!("");
                        }

                        let mut var_value = match linked_list.pop_front() {
                            /*
                            Some(SExpr::Cons(sym, list)) => {
                                println!("cons: {:?}", SExpr::Cons(
                                    sym.clone(), list.clone()));
                                println!("");

                                RLDynVar::new(None,
                                              pack_name,
                                              RLVar::SConsVar(
                                                  SExpr::Cons(sym, list)))
                            }
                            */

                            Some(SExpr::Atom(mut atom)) => {

                                let atom_string = atom.get_atom_string();

                                println!("atom_string: {:?}", atom_string);
                                atom.set_name_to_uppercase();

                                RLDynVar::new(None,
                                              pack_name,
                                              RLVar::SAtomVar(
                                                  SExpr::Atom(atom)))
                            }

                            Some(SExpr::Lambda(atom)) => {
                                RLDynVar::new(None,
                                              pack_name,
                                              RLVar::SAtomVar(
                                                  SExpr::Lambda(atom)))
                            }

                            Some(SExpr::Nil(nil)) => {
                                RLDynVar::new(None,
                                              pack_name,
                                              RLVar::NilVar(nil))
                            }

                            Some(SExpr::SList(slist)) => {
                                RLDynVar::new(None,
                                              pack_name,
                                              RLVar::SListVar(SExpr::SList(slist)))
                            }

                            _ => todo!(),
                        };

                        println!("var_value: {:?}", var_value);

                        if let Some(SExpr::Atom(atom)) =
                            linked_list.pop_front() {

                            let atom_doc_str = atom.get_atom_string();

                            var_value.set_doc_string(
                                Some(RLString::new(&atom_doc_str)));
                        }

                        let symbol =
                            RLEnvSymbol::new_var(var_name.clone(), var_value);

                        if let Some(_varsymb) =
                            curr_pack.get_symbol(&var_name) {

                            curr_pack.delete_symbol(var_name.clone());

                            curr_pack.add_symbol(var_name,
                                                 symbol.clone());
                            println!("Done");
                        } else {
                            curr_pack.add_symbol(var_name,
                                                 symbol.clone());
                            println!("Done2");
                        }

                        // let result_var_symbol =
                        // curr_pack.get_symbol(&var_name.clone()).unwrap();
                        /*
                        let bla = &"BLA".to_string();
                        let hash = curr_pack.get_symbols_hash_map();
                        let clony = clone_hash_map(&hash);
                        assert_eq!(hash.contains_key(bla), true);
                        assert_eq!(clony.contains_key(bla), true);

                        for val in hash.values() {
                            println!("hash values: {val}");
                        }

                        for val in clony.values() {
                            println!("clony values: {val}");
                        }
                        */

                        return Ok(RLResult::SymbolRes(symbol.clone()))
                    }
                }
                &_ => unreachable!()
            }
        }
        _ => unreachable!(),
    }
}

#[allow(non_snake_case)]
pub fn DEFVAR(_sexpr: &SExpr, _env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from DEFVAR");
    println!("");

    Ok(RLResult::StrRes("DEFVAR".to_string()))
}

// Special Operator PROGV
#[allow(non_snake_case, unused_assignments)]
pub fn PROGV(sexpr: &SExpr, _env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from PROGV");
    println!("");

match sexpr {
        SExpr::Cons(_symb, ll) => {
            let mut linked_list = ll.clone();

            println!("linked_list: {:?}", linked_list);
            println!("");

            let mut param_symbols_ll = LinkedList::<SExpr>::new();

            if let Some(SExpr::SList(params)) =
                linked_list.pop_front() {

                param_symbols_ll = params.get_linked_list();

                // get rid of "(" and ")"
                param_symbols_ll.pop_front();
                param_symbols_ll.pop_back();

                println!("param_symbols_ll: {:?}", param_symbols_ll);
                println!("");

                // param_symbols = SExpr::SList(params);
            } else {
                // throw an error
            }

            let mut param_values_ll = LinkedList::<SExpr>::new();

            if let Some(SExpr::SList(values)) =
                linked_list.pop_front() {

                param_values_ll = values.get_linked_list();

                // get rid of "(" and ")"
                param_values_ll.pop_front();
                param_values_ll.pop_back();

                println!("param_values_ll: {:?}", param_values_ll);
                println!("");

                // param_values = SExpr::SList(values);
            } else {
                // throw an error
            }

            let mut block = RLBlock::new("(PROGV)-START".to_string());

            while let Some(symbol) = param_symbols_ll.pop_front() {

                if let Some(value) = param_values_ll.pop_front() {
                    let var = RLVar::SAtomVar(value);
                    block.add_dyn_env_var(symbol.to_string(), var);

                } else {
                    let var = RLVar::NilVar(RLNil::new());
                    block.add_dyn_env_var(symbol.to_string(), var);
                }
            }

            block.set_progn_ll(&linked_list);

            Ok(RLResult::BlockRes(block))
         }
        _ => unreachable!(),
    }
}

// Special Form SETQ
#[allow(non_snake_case)]
pub fn SETQ(_sexpr: &SExpr, _env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    /*
    setq {pair}* => result
    setq var1 form1 var2 form2 ...
    */

    Ok(RLResult::NilRes(RLNil::new()))
}

#[allow(non_snake_case, unused_assignments)]
pub fn BLOCK(sexpr: &SExpr, _env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from BLOCK");
    println!("");

    match sexpr {
        SExpr::Cons(_symb, ll) => {
            let mut linked_list = ll.clone();

            println!("linked_list_1: {:?}", linked_list);
            println!("");

            // get Block name
            let mut block_name = "".to_string();
            let sexpr_block_name = linked_list.pop_front();

            println!("linked_list after block_name: {:?}", linked_list);
            println!("");

            if let Some(SExpr::Symb(symb)) = sexpr_block_name {
                let symb_name = symb.get_symbol_name();

                println!("block_name: {}", symb_name);
                println!("");

                block_name = symb_name.clone();
            } else {
                let mut no_symbol_expr = SExpr::Nil(RLNil::new());

                match sexpr_block_name {
                    Some(SExpr::Atom(atom)) => {
                        no_symbol_expr = SExpr::Atom(atom);
                    }

                    Some(SExpr::SList(slist)) => {
                        no_symbol_expr = SExpr::SList(slist);
                    }

                    _ => {}
                }

                let err_description = format!(
                    "The block name {} is not a symbol", no_symbol_expr);

                let err = BlockError::new(&err_description);
                return Err(RLError::BlockError(err))
            }

            // build Block
            if linked_list.is_empty() {
                return Ok(RLResult::NilRes(RLNil::new()))

            } else {
                println!("linked_list end: {:?}", linked_list);
                println!("");

                let mut block = RLBlock::new(block_name.to_uppercase());

                block.set_progn_ll(&linked_list);

                Ok(RLResult::BlockRes(block))
            }
        }
        _ => unreachable!(),
    }
}

#[allow(non_snake_case, unused_assignments)]
pub fn RETURN_FROM(sexpr: &SExpr, env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from RETURN-FROM");
    println!("");

    match sexpr {
        SExpr::Cons(_symb, ll) => {
            let mut linked_list = ll.clone();

            println!("linked_list_1: {:?}", linked_list);
            println!("");

            // get Block name
            let mut block_name = "".to_string();
            let sexpr_block_name = linked_list.pop_front();

            println!("linked_list after block_name: {:?}", linked_list);
            println!("");

            if let Some(SExpr::Symb(symb)) = sexpr_block_name {
                let symb_name = symb.get_symbol_name();

                println!("block_name: {}", symb_name);
                println!("");

                block_name = symb_name.clone();
            } else {
                let mut no_symbol_expr = SExpr::Nil(RLNil::new());

                match sexpr_block_name {
                    Some(SExpr::Atom(atom)) => {
                        no_symbol_expr = SExpr::Atom(atom);
                    }

                    Some(SExpr::SList(slist)) => {
                        no_symbol_expr = SExpr::SList(slist);
                    }

                    _ => {}
                }

                let err_description = format!(
                    "The block name {} is not a symbol", no_symbol_expr);

                let err = BlockError::new(&err_description);
                return Err(RLError::BlockError(err))
            }

            // build Block
            if linked_list.is_empty() {
                return Ok(RLResult::NilRes(RLNil::new()))

            } else {
                println!("linked_list end: {:?}", linked_list);
                println!("");

                let block_chain = env_ref.block_chain_retain(&block_name);

                println!("block_chain: {:?}", block_chain);
                println!("");

                if let Some(mut block) = env_ref.get_last_of_block_chain() {
                    println!("block: {:?}", block);
                    println!("");

                    block.set_progn_ll(&linked_list);

                    println!("block.get_progn_ll: {:?}",
                        block.get_progn_ll());
                    println!("");

                    // block.set_forms_tokens(&forms_tokens);

                    println!("block: {:?}", block);
                    println!("");

                    return Ok(RLResult::ReturnFromRes(block))
                } else {
                    let err = ReturnFromError::new(&block_name);
                    return Err(RLError::ReturnFromError(err))
                }
            }
        }
        _ => unreachable!(),
    }
}

// Special Operator PROGN
#[allow(non_snake_case)]
pub fn PROGN(sexpr: &SExpr, _env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from PROGN");
    println!("");

    match sexpr {
        SExpr::Cons(_symb, ll) => {
            if ll.is_empty() {
                return Ok(RLResult::NilRes(RLNil::new()))
            } else {
                let mut block = RLBlock::new("(PROGN)-START".to_string());

                block.set_progn_ll(&ll);

                Ok(RLResult::BlockRes(block))
            }
        }
        _ => unreachable!(),
    }
}

/*
Function APPLY

// Macro Defun

Accessor FDEFINITION

Function FBOUNDP

Function FMAKUNBOUND

Special Operator FLET, LABELS, MACROLET

Function FUNCALL

// Special Operator FUNCTION

Function FUNCTION-LAMBDA-EXPRESSION

Function FUNCTIONP

Function COMPILED-FUNCTION-P

Constant Variable CALL-ARGUMENTS-LIMIT

Constant Variable LAMBDA-LIST-KEYWORDS

Constant Variable LAMBDA-PARAMETERS-LIMIT

Macro DEFCONSTANT

Macro DEFPARAMETER, DEFVAR

Macro DESTRUCTURING-BIND

Special Operator LET, LET*

// Special Operator PROGV

Special Form SETQ

Macro PSETQ

// Special Operator BLOCK

Special Operator CATCH

Special Operator GO

// Special Operator RETURN-FROM

Macro RETURN

Special Operator TAGBODY

Special Operator THROW

Special Operator UNWIND-PROTECT

Constant Variable NIL

Function NOT

Constant Variable T

Function EQ

Function EQL

Function EQUAL

Function EQUALP

Function IDENTITY

Function COMPLEMENT

Function CONSTANTLY

Function EVERY, SOME, NOTEVERY, NOTANY

Macro AND

Macro COND

Special Operator IF

Macro OR

Macro WHEN, UNLESS

Macro CASE, CCASE, ECASE

Macro TYPECASE, CTYPECASE, ETYPECASE

Macro MULTIPLE-VALUE-BIND

Special Operator MULTIPLE-VALUE-CALL

Macro MULTIPLE-VALUE-LIST

Special Operator MULTIPLE-VALUE-PROG1

Macro MULTIPLE-VALUE-SETQ

Accessor VALUES

Function VALUES-LIST

Constant Variable MULTIPLE-VALUES-LIMIT

Macro NTH-VALUE

Macro PROG, PROG*

Macro PROG1, PROG2

Special Operator PROGN

Macro DEFINE-MODIFY-MACRO

Macro DEFSETF

Macro DEFINE-SETF-EXPANDER

Function GET-SETF-EXPANSION

Macro SETF, PSETF

Macro SHIFTF

Macro ROTATEF

Condition Type CONTROL-ERROR

Condition Type PROGRAM-ERROR

Condition Type UNDEFINED-FUNCTION
*/



/*
Special Operator FLET
Special Operator LABELS
Special Operator MACROLET
// Special Operator FUNCTION
Special Operator LET
Special Operator LET*
// Special Operator PROGV
Special Form SETQ
// Special Operator BLOCK
Special Operator CATCH
Special Operator GO
// Special Operator RETURN-FROM
Special Operator TAGBODY
Special Operator THROW
Special Operator UNWIND-PROTECT
Special Operator IF
Special Operator MULTIPLE-VALUE-CALL
Special Operator MULTIPLE-VALUE-PROG1
// Special Operator PROGN
*/
