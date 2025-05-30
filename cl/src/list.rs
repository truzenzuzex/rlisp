// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::hash_map::HashMap;

use env::named_lambda::RLNamedLambda;
use env::result::RLResult;
use env::symb::RLEnvSymbol;

use err::err::{RLError,
               SimpleTypeError,
               TypeError};

use expr::atom::RLAtom;
use expr::qexpr::QExpr;
use expr::sexpr::SExpr;
use expr::expr::{sexpr_to_qexpr};
use expr::list::RLList;
use expr::nil::RLNil;
use expr::string::RLString;

use hash::hash::RLHash;

/*
remember QExpr types
    Atom(String),
    Nil,
    Symb(Symbol),
    QList(LinkedList<QExpr>),
    Cons(Box<QExpr>, Box<QExpr>),
*/

pub struct ListFuncs {
}

impl ListFuncs {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                cl_pack_hash: &mut HashMap<String, RLEnvSymbol, RLHash>) {

        // cl_pack_hash
        cl_pack_hash.insert("car".to_string(),
            RLEnvSymbol::new_named_lambda("CAR".to_string(),
                RLNamedLambda::new_func(
                    "CAR".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Return the 1st object in a list."))),
                    "COMMON-LISP".to_string(),
                    Some(|sexpr| {
                        Ok::<RLResult, RLError>(
                            CAR(&sexpr)?) }),
                    None)));

        cl_pack_hash.insert("cdr".to_string(),
            RLEnvSymbol::new_named_lambda("CDR".to_string(),
                RLNamedLambda::new_func(
                    "CDR".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Return all but the first object in a list."))),
                    "COMMON-LISP".to_string(),
                    Some(|sexpr| {
                        Ok::<RLResult, RLError>(
                            CDR(&sexpr)?) }),
                    None)));

        cl_pack_hash.insert("concatenate".to_string(),
            RLEnvSymbol::new_named_lambda("CONCATENATE".to_string(),
                RLNamedLambda::new_func(
                    "CONCATENATE".to_string(),
                    None,
                    "COMMON-LISP".to_string(),
                    Some(|sexpr| {
                        Ok::<RLResult, RLError>(
                            CONCATENATE(&sexpr)?) }),
                    None)));

        cl_pack_hash.insert("cons".to_string(),
            RLEnvSymbol::new_named_lambda("CONS".to_string(),
                RLNamedLambda::new_func(
                    "CONS".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Return a list with SE1 as the CAR and SE2 as the CDR."))),
                    "COMMON-LISP".to_string(),
                    Some(|sexpr| {
                        Ok(RLResult::QExprRes(
                            CONS(&sexpr)?)) }),
                    None)));

        cl_pack_hash.insert("list".to_string(),
            RLEnvSymbol::new_named_lambda("LIST".to_string(),
                RLNamedLambda::new_func(
                    "LIST".to_string(),
                    Some(SExpr::Atom(RLAtom::new("Construct and return a list containing the objects ARGS."))),
                    "COMMON-LISP".to_string(),
                    Some(|sexpr| {
                        Ok::<RLResult, RLError>(
                            LIST(&sexpr)?) }),
                    None)));
    }
}

#[allow(non_snake_case)]
pub fn LIST(sexpr: &SExpr) -> Result<RLResult, RLError> {
    /*
    Syntax:

    */

     println!("Hello from LIST");
     println!("");

     match sexpr {
        SExpr::Cons(symb, ll) => {
            match &*symb.name {
                "list" => {
                    // let result_qlist = QExpr::new_list(ll.clone());

                    let result_qlist = RLList::<QExpr>::new_qlist(ll);

                    // println!("result_qlist: {:?}", result_qlist);
                    // println!("");

                    /*
                    if result_qlist.is_list_with_cons() {
                        return Ok(RLResult::QExprRes(result_qlist));
                    } else {
                        return Err(RLError::SimpleProgramError);
                    }
                    */

                    return Ok(RLResult::QExprRes(QExpr::QList2(result_qlist)));
                }
                &_ => unreachable!()
            }
        }
        _ => unreachable!()
    }
}

#[allow(non_snake_case)]
pub fn CAR(sexpr: &SExpr) -> Result<RLResult, RLError> {
    /*
    Syntax:

    */

    println!("Hello from CAR");
    println!("");

    match sexpr {
        SExpr::Cons(symb, ll) => {
            match &*symb.name {
                "car" => {

                    let mut linked_list = ll.clone();

                    if linked_list.is_empty() {
                        return Err(RLError::SimpleProgramError);
                    } else if linked_list.len() > 1 {
                        return Err(RLError::SimpleProgramError);
                    } else {
                        if let Some(sexpr) = linked_list.pop_front() {
                            let mut qlist = sexpr_to_qexpr(sexpr);

                            println!("qlist: {:?}", qlist);
                            println!("");

                            match qlist.car() {
                                Ok(result_car) => {
                                    return Ok(RLResult::QExprRes(result_car));
                                }

                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        } else {
                            return Err(RLError::SimpleProgramError);
                        }
                    }
                }
                &_ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

#[allow(non_snake_case)]
pub fn CDR(sexpr: &SExpr) -> Result<RLResult, RLError> {
    /*
    Syntax:
    */

    println!("Hello from CDR");
    println!("");

    match sexpr {
        SExpr::Cons(symb, ll) => {
            match &*symb.name {
                "cdr" => {
                    let mut linked_list = ll.clone();

                    if linked_list.len() > 1 {
                        return Err(RLError::SimpleProgramError);
                    } else {
                        if let Some(sexpr) = linked_list.pop_front() {
                            let mut qlist = sexpr_to_qexpr(sexpr);

                            println!("qlist: {:?}", qlist);
                            println!("");

                            match qlist.cdr() {
                                Ok(result_cdr) => {
                                    return Ok(RLResult::QExprRes(result_cdr));
                                }

                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        } else {
                            return Err(RLError::SimpleProgramError);
                        }
                    }
                }

                &_ => unreachable!(),
            }
        }

        _ => unreachable!(),
    }
}

#[allow(non_snake_case)]
pub fn CONCATENATE(sexpr: &SExpr) ->
    Result<RLResult, RLError> {

    /*
    Syntax:
    concatenate result-type &rest sequences => result-sequence
    */

    println!("Hello from CONCATENATE");
    println!("");

    match sexpr {
        SExpr::Cons(symb, ll) => {
            let mut linked_list = ll.clone();

            match &*symb.name {
                "concatenate" => {
                    let type_ident = linked_list.pop_front();

                    let mut type_ident_rlstring = RLString::new("");

                    if let Some(SExpr::Symb(ref symb)) = type_ident {
                        let symb_name = symb.get_symbol_name();

                        type_ident_rlstring =
                            RLString::new(&symb_name).to_uppercase();
                    }

                    if let Some(SExpr::Atom(atom)) = type_ident {
                        type_ident_rlstring =
                            atom.get_atom_rlstring().to_uppercase();
                    }

                    println!("type_ident: {:?}", type_ident_rlstring.get());
                    println!("");

                    let first_op = linked_list.pop_front().unwrap();

                    if "LIST".to_string()
                             .eq(&type_ident_rlstring.get()) {

                        let mut result: QExpr;

                        let first_op_qexpr = sexpr_to_qexpr(first_op);

                        let mut rllist = RLList::<QExpr>::new();

                        match first_op_qexpr {
                            QExpr::Nil(ref _nil)     => {
                                result = QExpr::Nil(RLNil::new());
                            }

                            QExpr::QList2(ref qlist) => {
                                rllist = first_op_qexpr.get_rllist();
                                result = QExpr::QList2(qlist.clone());
                            }

                            _ => {
                                let err = TypeError::new(
                                    &first_op_qexpr.to_string(), "SEQUENCE");

                                return Err(RLError::TypeError(err));
                            }
                        }

                        for _n in 0..linked_list.len() {
                            let next_op = linked_list.pop_front().unwrap();

                            let next_op_qexpr = sexpr_to_qexpr(next_op);

                            match next_op_qexpr {
                                QExpr::Nil(ref _nil) => {
                                    if let QExpr::Nil(nil) = result {
                                        result = QExpr::Nil(nil);
                                    } else {
                                        result = QExpr::QList2(rllist.clone());
                                    }
                                }

                                QExpr::QList2(ref qlist) => {
                                    if let QExpr::Nil(_nil) = result {
                                        rllist = next_op_qexpr.get_rllist();

                                        result = QExpr::QList2(qlist.clone());
                                    } else {
                                        result = QExpr::QList2(
                                            rllist.concatenate_lists(
                                                &next_op_qexpr));
                                    }
                                }

                                _ => {
                                    let err = TypeError::new(
                                        &first_op_qexpr.to_string(),"SEQUENCE");

                                    return Err(RLError::TypeError(err));
                                }
                            }

                        } // for

                        return Ok(RLResult::QExprRes(result));

                    } else if "STRING".to_string()
                                      .eq(&type_ident_rlstring.get()) {

                        let mut result = RLString::new("");

                        if let SExpr::Atom(ref first_atom) = first_op {

                            if first_atom.is_rlstring_atom() {

                                let mut first_op_rlstring =
                                    first_atom.get_atom_rlstring();

                                for _n in 0..linked_list.len() {
                                    let next_op = linked_list.pop_front()
                                                             .unwrap();

                                    if let SExpr::Atom(ref next_atom) =
                                        next_op {

                                        if next_atom.is_rlstring_atom() {

                                            let next_op_rlstring =
                                                next_atom.get_atom_rlstring();

                                            result =
                                                first_op_rlstring
                                                    .concatenate_strings(
                                                        &next_op_rlstring);
                                        } else {
                                            let err = TypeError::new(
                                                &next_atom.to_string(),
                                                "SEQUENCE");

                                            return Err(
                                                RLError::TypeError(err));
                                        }

                                    } else {
                                        let err = TypeError::new(
                                            &next_op.to_string(),
                                            "SEQUENCE");

                                        return Err(RLError::TypeError(err));
                                    }
                                }
                            } else {
                                let err = TypeError::new(
                                    &first_atom.to_string(),
                                    "SEQUENCE");

                                return Err(RLError::TypeError(err));
                            }

                        } else {
                                let err = TypeError::new(
                                    &first_op.to_string(),
                                    "SEQUENCE");
                                return Err(RLError::TypeError(err));
                        }

                        return Ok(RLResult::StringRes(result));

                    } else {
                        let err = SimpleTypeError::new(
                            &type_ident_rlstring.get(),
                            "SEQUENCE");

                        return Err(RLError::SimpleTypeError(err));
                    }
                }
                &_ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

#[allow(non_snake_case)]
pub fn CONS(sexpr: &SExpr) -> Result<QExpr, RLError> {
    /*
    Syntax:

    */

    println!("Hello from CONS");
    println!("");

    match sexpr {
        SExpr::Cons(symb, ll) => {
            match &*symb.name {
                "cons" => {
                    let mut s_list = ll.clone();

                    if s_list.len() < 2 {
                        return Err(RLError::SimpleProgramError);
                    } else if s_list.len() == 2 {
                        let first = s_list.pop_front().unwrap();
                        let second = s_list.pop_front().unwrap();

                        return Ok(QExpr::new_cons((
                            sexpr_to_qexpr(first),
                            sexpr_to_qexpr(second))))

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
