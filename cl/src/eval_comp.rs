// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::hash_map::HashMap;
use std::ops::Not;

use env::block::RLBlock;
use env::env::RLEnvironment;
use env::env_trait::EnvTrait;
use env::lambda::RLLambda;
use env::named_lambda::RLNamedLambda;
use env::result::RLResult;
use env::symb::RLEnvSymbol;
use env::var::RLVar;

use err::err::RLError;

use expr::atom::RLAtom;
use expr::expr::Expr;
use expr::nil::RLNil;
// use expr::QuoteTrait;
use expr::sexpr::SExpr;
use expr::symb::RLSymbol;

use hash::hash::RLHash;

pub struct EvalCompilationFuncs {
}

impl EvalCompilationFuncs {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                cl_pack_hash:  &mut HashMap<String, RLEnvSymbol, RLHash>) {

        cl_pack_hash.insert("eval".to_string(),
            RLEnvSymbol::new_named_lambda("EVAL".to_string(),
                RLNamedLambda::new_func(
                    "EVAL".to_string(),
                    None,
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        let result = EVAL(&sexpr, env_ref);
                        match result {
                            Ok(eval) => { return Ok(eval); }
                            Err(err) => { return Err(err); }
                        }
                    }))));

        cl_pack_hash.insert("lambda".to_string(),
            RLEnvSymbol::new_named_lambda("LAMBDA".to_string(),
                RLNamedLambda::new_func(
                    "LAMBDA".to_string(),
                    None,
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        // Ok(RLResult::LambdaRes(LAMBDA(&sexpr)?)),
                        let result = LAMBDA(&sexpr, env_ref);
                        match result {
                            Ok(lambda) => { return Ok(lambda); }
                            Err(err)     => { return Err(err); }
                        }
                    }))));

        cl_pack_hash.insert("quote".to_string(),
            RLEnvSymbol::new_named_lambda("QUOTE".to_string(),
                RLNamedLambda::new_func(
                    "QUOTE".to_string(),
                    Some(SExpr::Atom(RLAtom::new("QUOTE value

Return VALUE without evaluating it."))),
                    "COMMON-LISP".to_string(),
                    Some(|sexpr| {
                        Ok(RLResult::ExprRes(QUOTE(&sexpr)?)) }),
                    None)));

        cl_pack_hash.insert("backquote".to_string(),
            RLEnvSymbol::new_named_lambda("BACKQUOTE".to_string(),
                RLNamedLambda::new_func(
                    "BACKQUOTE".to_string(),
                    Some(SExpr::Atom(RLAtom::new(""))),
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            BACKQUOTE(&sexpr, env_ref)?) }))));

        cl_pack_hash.insert("defmacro".to_string(),
            RLEnvSymbol::new_named_lambda("DEFMACRO".to_string(),
                RLNamedLambda::new_func(
                    "DEFMACRO".to_string(),
                    Some(SExpr::Atom(RLAtom::new(""))),
                    "COMMON-LISP".to_string(),
                    None,
                    Some(|sexpr, env_ref| {
                        Ok::<RLResult, RLError>(
                            DEFMACRO(&sexpr, env_ref)?) }))));
    }
}

/*
Symbol LAMBDA

Macro LAMBDA

Function COMPILE
*/

#[allow(non_snake_case, unused_assignments)]
pub fn LAMBDA(sexpr: &SExpr, env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from LAMBDA");
    println!("");

    let curr_pack = &mut env_ref.get_mut_current_package();

    let lambdas_hash_map = curr_pack.get_lambdas_hash_map();

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            println!("linked_list: {:?}", linked_list);
            println!("");

            match &*symb.name {
                "lambda" => {
                     if linked_list.is_empty() {
                        return Err(RLError::SimpleProgramError);
                    } else {
                        let mut init_parameters = SExpr::Nil(RLNil::new());

                        if let Some(SExpr::SList(mut list)) =
                            linked_list.pop_front() {
                                list.pop_front();
                                list.pop_back();

                                init_parameters = SExpr::SList(list);

                                println!("init_parameters: {:?}",
                                    init_parameters);
                                println!("");

                        } else {
                            // which RLError?!
                            return Err(RLError::SimpleProgramError);
                        }

                        let opt_docstring: Option<String>;

                        let mut body: SExpr = SExpr::Nil(RLNil::new());

                        let next_param = linked_list.pop_front();

                        if let Some(SExpr::Atom(ref atom)) = next_param {
                            opt_docstring = Some(atom.to_string());

                            println!("{}", opt_docstring.clone()
                                                        .unwrap());
                            println!("");

                            if let Some(SExpr::SList(mut slist)) =
                                linked_list.pop_front() {

                                println!("SExpr::SList(slist): {:?}",
                                    SExpr::SList(slist.clone()));
                                println!("");

                                body = slist.slist_to_cons();

                                println!("body with doc: {:?}", body);
                                println!("");

                            } else {
                                // which RLError?!
                                return Err(RLError::SimpleProgramError);
                            }

                        } else {
                            opt_docstring = None;

                            if let Some(SExpr::SList(mut slist)) =
                                next_param {

                                println!("SExpr::SList(slist): {:?}",
                                    SExpr::SList(slist.clone()));
                                println!("");

                                body = slist.slist_to_cons();

                                println!("body without doc: {:?}", body);
                                println!("");
                            } else {
                                // which RLError?!
                                return Err(RLError::SimpleProgramError);
                            }
                        }

                        let mut lambda = RLLambda::new(
                            None,
                            opt_docstring,
                            // curr_pack_name,
                            // None,
                            );

                        let _ = lambda.init_parameters(init_parameters);

                        lambda.set_body(body);

                        println!("lambda: {:?}", lambda);
                        println!("");

                        println!("body lambda: {:?}", lambda.get_body());
                        println!("");

                        lambdas_hash_map.insert(lambda.get_id(),
                                                lambda.clone());

                        Ok(RLResult::LambdaRes(lambda))
                    }

                }

                &_ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

#[allow(non_snake_case)]
pub fn EVAL(sexpr: &SExpr, _env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from EVAL");
    println!("");

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            println!("{:?}", linked_list);
            println!("");

            match &*symb.name {
                "eval" => {
                    if linked_list.is_empty() {
                        return Err(RLError::SimpleProgramError);
                    } else {

                    if let Some(sexpr) = linked_list.pop_front() {
                            match sexpr {
                                SExpr::SList(mut slist) => {

                                    let form = slist.slist_to_sform();

                                    println!("eval form: {:?}", form);
                                    println!("");

                                    return Ok(RLResult::SExprRes(form));
                                }

                                SExpr::Atom(atom) => {
                                    return Ok(RLResult::SExprRes(
                                        SExpr::Atom(atom)));
                                }

                                _ => todo!(),
                            }
                     } else {
                         return Ok(RLResult::SExprRes(
                             SExpr::Nil(RLNil::new())));
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
Special Operator EVAL-WHEN

Special Operator LOAD-TIME-VALUE
*/

// Special Operator QUOTE
#[allow(non_snake_case)]
pub fn QUOTE(sexpr: &SExpr) ->
    Result<Expr, RLError> {

    match sexpr {
        SExpr::Cons(symb, ll) => {
            match &*symb.name {
                "quote" => {
                    println!("Hello from QUOTE");
                    println!("");

                    println!("ll: {:?}", ll);
                    println!("");

                    let mut linked_list = ll.clone();

                    /*
                    if linked_list.len() > 1 {
                        while linked_list.len() > 1 {
                            linked_list.pop_back();
                        }
                    }
                    */

                    if linked_list.is_empty().not() {
                        let linked_list_item =
                            linked_list.pop_front().unwrap();

                         match linked_list_item {
                            SExpr::Cons(symb, ll) => {
                                // return Ok(Expr::SExpr(<expr::sexpr::SExpr as QuoteTrait>::quote(SExpr::Cons(symb, list))?));
                                return Ok(expr::expr::Expr::SExpr(
                                    SExpr::Cons(symb, ll)));
                            }

                            SExpr::SList(slist) => {
                                println!("slist: {:?}", slist);
                                println!("");

                                // return Ok(Expr::SExpr(<expr::sexpr::SExpr as QuoteTrait>::quote(SExpr::SList(s_list))?));
                                return Ok(expr::expr::Expr::SExpr(
                                    SExpr::SList(slist)));
                            }

                            SExpr::Atom(atom) =>  {
                                // return Ok(Expr::SExpr(<expr::sexpr::SExpr as QuoteTrait>::quote(SExpr::Atom(atom.to_string().to_uppercase()))?));
                                return Ok(expr::expr::Expr::SExpr(
                                    SExpr::Atom(atom)));
                            }

                            SExpr::Nil(nil) => {
                                // return Ok(Expr::SExpr(<expr::sexpr::SExpr as QuoteTrait>::quote(SExpr::Nil)?));
                                return Ok(expr::expr::Expr::SExpr(
                                    SExpr::Nil(nil)));
                            }

                            SExpr::Symb(symb) => {
                                // return Ok(Expr::SExpr(<expr::sexpr::SExpr as QuoteTrait>::quote(SExpr::Symb(Symbol { name: symb.name }))?));
                                return Ok(expr::expr::Expr::SExpr(
                                    SExpr::Symb(symb)));
                            }

                            SExpr::QList(qlist) => {
                                println!("quote - QList");
                                println!("");
                                // return Ok(Expr::QExpr(<expr::qexpr::QExpr as QuoteTrait>::quote(SExpr::QList(qlist))?));
                                  return Ok(expr::expr::Expr::SExpr(
                                    SExpr::QList(qlist)));
                            }

                            _ => todo!(),
                         }
                    } else {
                        return Err(RLError::SimpleProgramError)
                    }
                }

                &_ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

#[allow(non_snake_case)]
pub fn BACKQUOTE(sexpr: &SExpr, _env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from BACKQUOTE");
    println!("");

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            println!("linked_list: {:?}", linked_list);
            println!("");

            match &*symb.name {
                "backquote" => {
                    if linked_list.len() == 0 {
                        return Err(RLError::SimpleProgramError)
                    } else {
                        let mut block = RLBlock::new_with_id_as_name();

                        linked_list.push_front(
                            SExpr::Symb(RLSymbol::new_with_str("`")));

                        block.set_progn_ll(&linked_list);

                        return Ok(RLResult::NamedDsBindRes(block.clone()));
                    }
                }
                &_ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

#[allow(non_snake_case, unused_assignments)]
pub fn DEFMACRO(sexpr: &SExpr, env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    /*
    Syntax:
        defmacro name lambda-list [[declaration* | documentation]] form*
    */

    println!("Hello from DEFMACRO");
    println!("");

    let curr_pack = &mut env_ref.get_mut_current_package();

    let curr_pack_name = curr_pack.get_name();

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            println!("linked_list: {:?}", linked_list);
            println!("");

            match &*symb.name {
                "defmacro" => {
                    if linked_list.is_empty() {
                        return Err(RLError::SimpleProgramError);
                    } else {
                        let mut name = "dummy".to_string();

                        if let Some(SExpr::Symb(symb)) =
                            linked_list.pop_front() {
                                let symb_name = symb.get_symbol_name();

                                name = symb_name;

                                println!("{}", name);
                        } else {
                            // Which RLError?!
                            return Err(RLError::SimpleProgramError);
                        }

                        let mut parameters: SExpr = SExpr::Nil(RLNil::new());

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

                        // let mut lambda_string = "".to_string();

                        let next_param = linked_list.pop_front();

                        match next_param {
                            Some(SExpr::Nil(nil)) => {
                                opt_docstring = Some(SExpr::Nil(nil));
                            }

                            Some(SExpr::Atom(atom)) => {
                                opt_docstring = Some(SExpr::Atom(atom));
                            }

                            /*
                            Some(SExpr::Lambda(atom)) => {
                                opt_docstring = None;

                                    lambda_string =
                                        atom.get_atom_string();

                                    println!("lambda_string: {:?}",
                                        lambda_string);
                                    println!("")
                            }
                            */

                            Some(SExpr::SForm(form)) => {
                                opt_docstring = None;

                                linked_list.push_front(SExpr::SForm(form));
                            }

                            Some(SExpr::SToken(token)) => {
                                opt_docstring = None;

                                linked_list.push_front(
                                    SExpr::SToken(token));
                            }

                            _ => unreachable!(),
                        }

                        let mut named_lambda = RLNamedLambda::new_macro(
                            name.clone().to_uppercase(),
                            opt_docstring,
                            curr_pack_name,
                            None,
                            Some(|sexpr, env_ref| {
                                Ok::<RLResult, RLError>(
                                    RLMACRO(&sexpr, env_ref)?) }));

                        /*
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
                            /*
                            let mut block = RLBlock::new(name.clone()
                                                             .to_uppercase());

                            block.set_progn_ll(&linked_list);

                            named_lambda.set_block(&block);
                            */


                            if let Some(mut block) =
                                named_lambda.get_block() {

                                block.set_progn_ll(&linked_list);

                                named_lambda.set_block(&block);
                            }
                        }
                        */

                        let _ = named_lambda.init_parameters(parameters);

                        if let Some(mut block) = named_lambda.get_block() {

                            block.set_progn_ll(&linked_list);

                            named_lambda.set_block(&block);

                            println!("named_lambda: {:?}", named_lambda);
                            println!("");
                        }

                        let symbol = RLEnvSymbol::new_named_lambda(
                            name.clone().to_uppercase(),
                            named_lambda.clone());

                        curr_pack.add_symbol(name.clone(), symbol);

                        println!("last of defmacro");
                        println!("named_lambda: {:?}", named_lambda);
                        println!("");

                        Ok(RLResult::StrRes(name.clone().to_uppercase()))
                    }
                }

                &_ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}


#[allow(non_snake_case)]
pub fn RLMACRO(sexpr: &SExpr, env_ref: &mut RLEnvironment) ->
    Result<RLResult, RLError> {

    println!("Hello from RLMACRO");
    println!("");

    let curr_pack = &mut env_ref.get_mut_current_package();

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            let symb_string = &*symb.name;

            println!("{:?}", symb_string);

            let symbol = curr_pack.get_symbol(&symb_string.to_string())
                .unwrap()
                .clone();

            println!("symbol: {:?}", symbol);
            println!("");

            let mut named_lambda = symbol.named_lambda.clone().unwrap();

            let mut parameters = named_lambda.get_parameters();

            if let Some(SExpr::SList(list)) =
                parameters.get_req_params_as_option() {

                let ll = list.get_linked_list();

                for item in ll {
                    if let SExpr::Atom(atom) = item {
                        let param = atom.get_atom_string();

                        let var = RLVar::SAtomVar(linked_list.pop_front().expect("REASON"));

                        named_lambda.add_dyn_env_var(param, var);
                    }
                }

                println!("named_lambda: {:?}", named_lambda);
                println!("");

                Ok(RLResult::MacroRes(named_lambda))
            } else {
                // which RLError?!
                return Err(RLError::SimpleProgramError)
            }
        }
        _ => unreachable!(),
    }
}

/*
// Symbol LAMBDA

// Macro LAMBDA

Function COMPILE

// Function EVAL

Special Operator EVAL-WHEN

Special Operator LOAD-TIME-VALUE

// Special Operator QUOTE

Accessor COMPILER-MACRO-FUNCTION

Macro DEFINE-COMPILER-MACRO

Macro DEFMACRO

Accessor MACRO-FUNCTION

Function MACROEXPAND, MACROEXPAND-1

Macro DEFINE-SYMBOL-MACRO

Special Operator SYMBOL-MACROLET

Variable *MACROEXPAND-HOOK*

Function PROCLAIM

Macro DECLAIM

Symbol DECLARE

Declaration IGNORE, IGNORABLE

Declaration DYNAMIC-EXTENT

Declaration TYPE

Declaration INLINE, NOTINLINE

Declaration FTYPE

Declaration DECLARATION

Declaration OPTIMIZE

Declaration SPECIAL

Special Operator LOCALLY

Special Operator THE

Function SPECIAL-OPERATOR-P

Function CONSTANTP
*/

/*
Special Operator EVAL-WHEN
Special Operator LOAD-TIME-VALUE
Special Operator QUOTE
Special Operator SYMBOL-MACROLET
Special Operator LOCALLY
Special Operator THE
*/
