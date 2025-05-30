// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::HashMap;
use std::collections::LinkedList;
use std::ops::Not;

use crate::param_generator::RLParamGenerator;

use err::err::{RLError, ParseError, SimpleError};

use expr::atom::RLAtom;
use expr::list::RLList;
use expr::nil::RLNil;
use expr::sexpr::SExpr;
use expr::symb::RLSymbol;

use hash::hash::RLHash;

use lex::lexer::RLLexer;

use pars_symb::symbol::Symbol;
use pars_symb::token::Token;


pub struct DataControlParams {
}

impl DataControlParams {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                param_gens: &mut HashMap<String, RLParamGenerator, RLHash>) {
        param_gens.insert("function".to_string(),
            RLParamGenerator::new(|param_gen, lexer| {
                Ok::<LinkedList<SExpr>, RLError>(
                    function(param_gen, lexer)?) }));

        param_gens.insert("defun".to_string(),
            RLParamGenerator::new(|param_gen, lexer| {
                Ok::<LinkedList<SExpr>, RLError>(
                    defun(param_gen, lexer)?) }));

        param_gens.insert("progv".to_string(),
            RLParamGenerator::new(|param_gen, lexer| {
                Ok::<LinkedList<SExpr>, RLError>(
                    progv(param_gen, lexer)?) }));

        param_gens.insert("block".to_string(),
            RLParamGenerator::new(|param_gen, lexer| {
                Ok::<LinkedList<SExpr>, RLError>(
                    block(param_gen, lexer)?) }));

        param_gens.insert("return-from".to_string(),
            RLParamGenerator::new(|param_gen, lexer| {
                Ok::<LinkedList<SExpr>, RLError>(
                    return_from(param_gen, lexer)?) }));
    }
}

pub fn function(param_gen: &mut RLParamGenerator, lexer: &mut RLLexer) ->
    Result<LinkedList<SExpr>, RLError> {

    /*
    (lambda lambda-list [[declaration* | documentation]] form*)
    ==
    (function (lambda lambda-list [[declaration* | documentation]] form*))
    ==
    #'(lambda lambda-list [[declaration* | documentation]] form*)
    */

    println!("Hello from FUNCTION-PARAMS");
    println!("");

    let mut list = LinkedList::<SExpr>::new();

    // match function name or lambda expr
    match lexer.next() {
        Token::Atom(atom) => {
            let err = ParseError::new("Parse Function",
                pars_symb::token::Token::Atom(atom));

            return Err(RLError::ParseError(err))
        }

        Token::Symb(symb) => {
            match &*symb.name {
                "(" |
                ")" => {
                    // lexer.restore_token();
                }

                _ => {
                    println!("symb.name: {:?}", symb.name);
                    println!("");

                    param_gen.set_name(
                        SExpr::Symb(RLSymbol::new_with_symb(
                            &Symbol { name: symb.name.clone() })));

                    list.push_back(
                        SExpr::Symb(RLSymbol::new_with_symb(
                            &Symbol { name: symb.name.clone() })));

                    param_gen.set_param_order(vec!["name".to_string()]);
                }
            }
        }

        Token::Eof => {
           let err = ParseError::new("wrong number of args to FUNCTION",
                pars_symb::token::Token::Eof);

            return Err(RLError::ParseError(err))
        }
    }
    Ok(list)
}

pub fn defun(param_gen: &mut RLParamGenerator, lexer: &mut RLLexer) ->
    Result<LinkedList<SExpr>, RLError> {

    println!("Hello from DEFUN-PARAMS");
    println!("");

    let mut list = LinkedList::<SExpr>::new();

    // match function name
    match lexer.next() {
        Token::Atom(atom) => {
            match &*atom {
                "()"  | "nil" |
                "NIL" | "Nil" |
                "NIl" | "NiL" |
                "niL" | "nIL" |
                "nIl" => {
                    let err = ParseError::new("Parse DEFUN",
                        pars_symb::token::Token::Atom(atom));

                    return Err(RLError::ParseError(err))
                }

                _ => {
                    param_gen.set_name(
                        SExpr::Symb(RLSymbol::new_with_symb(
                            &Symbol { name: atom.clone() })));

                    // list.push_back(
                    //    SExpr::Symb(RLSymbol::new_with_symb(
                    //        &Symbol { name: atom.clone() })));

                }
            }
        }

        Token::Symb(symb) => {
            let err = ParseError::new("Parse DEFUN",
                 pars_symb::token::Token::Symb(symb));

            return Err(RLError::ParseError(err))
        }

        Token::Eof => {
            let err = ParseError::new("Parse DEFUN",
                pars_symb::token::Token::Eof);

            return Err(RLError::ParseError(err))
        }
    }

    // match defun params
    let mut param_list = LinkedList::<SExpr>::new();

    loop {
        match lexer.next() {
            Token::Symb(symb) => {
                match &*symb.name {
                    "(" => {
                        param_list.push_back(
                            SExpr::Symb(RLSymbol::new_with_str("(")))
                    }

                    ")" => {
                        param_list.push_back(
                            SExpr::Symb(RLSymbol::new_with_str(")")));

                            param_gen.set_lambda_list(
                                SExpr::SList(
                                    RLList::<SExpr>::new_with_list(
                                        &param_list)));

                            // list.push_back(SExpr::SList(lambda_list));
                            break;
                    }

                    _ =>  {
                        param_list.push_back(
                            SExpr::Symb(RLSymbol::new_with_symb(&symb)))
                    }
                }
            }

            Token::Atom(atom) => {
                match &*atom {
                    "()"  | "nil" |
                    "NIL" | "Nil" |
                    "NIl" | "NiL" |
                    "niL" | "nIL" |
                    "nIl" => {
                        let err = ParseError::new("Parse DEFUN",
                            pars_symb::token::Token::Atom(atom));

                        return Err(RLError::ParseError(err))
                    }

                    _ => {
                        param_list.push_back(SExpr::Atom(RLAtom::new(&atom)));
                    }
                }
            }

            Token::Eof => {
                let err = ParseError::new("Parse DEFUN",
                    pars_symb::token::Token::Eof);

                return Err(RLError::ParseError(err))
            }
        }
    } // end loop

    // match docstring
    match lexer.next() {
        Token::Atom(atom) => {
            match &*atom {
                "()"  | "nil" |
                "NIL" | "Nil" |
                "NIl" | "NiL" |
                "niL" | "nIL" |
                "nIl" => {
                    param_gen.set_docstring(SExpr::Nil(RLNil::new()));
                    // list.push_back(SExpr::Nil(RLNil::new()));
                }

                _ => {
                    param_gen.set_docstring(SExpr::Atom(RLAtom::new(&atom)));
                    // list.push_back(SExpr::Atom(RLAtom::new(&atom)));
                }
            }
        }

        Token::Symb(_symb) => {
            lexer.restore_token();
        }

        Token::Eof => {
            let err = ParseError::new("Parse DEFUN",
                pars_symb::token::Token::Eof);

            return Err(RLError::ParseError(err))
        }
    }

    let mut form_list  = Vec::<Token>::new();

    // match SExpr::Cons, function body
    let mut func_body_list = LinkedList::<SExpr>::new();

    let mut paren_count = 0;

    let mut lambda_list = LinkedList::<SExpr>::new();

    let mut lambda_param = LinkedList::<SExpr>::new();

    let mut is_lambda_param:bool;

    let mut lambda_body = LinkedList::<SExpr>::new();

    if let Token::Symb(symb) = lexer.next() {
        match &*symb.name {
            "(" => {
                   paren_count = paren_count + 1;

                   form_list.push(Token::Symb(symb));

                   func_body_list.push_back(
                       SExpr::Symb(RLSymbol::new_with_str("(")));

                   lambda_list.push_back(
                       SExpr::Symb(RLSymbol::new_with_str("(")));
            }

            &_ => {}
        }
    }

    if let Token::Symb(symb) = lexer.next() {
        match &*symb.name {
            "function" => {
                func_body_list.push_back(
                    SExpr::Symb(RLSymbol::new_with_str("function")));

                is_lambda_param = true;

                lambda_list.pop_back();

                loop {
                    match lexer.next() {
                        Token::Symb(symb) => {
                            match &*symb.name {
                                "(" => {
                                    paren_count = paren_count + 1;

                                    if is_lambda_param &&
                                       paren_count == 2 {
                                        lambda_list.push_back(
                                            SExpr::Symb(
                                                RLSymbol::new_with_str("(")));
                                    }

                                    if is_lambda_param &&
                                       paren_count == 3 {

                                        lambda_param.push_back(
                                            SExpr::Symb(
                                                RLSymbol::new_with_str("(")));
                                    }

                                    if is_lambda_param.not() {
                                        lambda_body.push_back(
                                            SExpr::Symb(
                                                RLSymbol::new_with_str("(")));
                                    }
                                }

                                ")" => {
                                    paren_count = paren_count - 1;

                                    if is_lambda_param.not() &&
                                       paren_count != 1 &&
                                       paren_count != 0 {
                                        lambda_body.push_back(
                                            SExpr::Symb(
                                                RLSymbol::new_with_str(")")));
                                    }

                                    if is_lambda_param &&
                                       paren_count == 2 {

                                        is_lambda_param = false;

                                        lambda_param.push_back(
                                            SExpr::Symb(
                                                RLSymbol::new_with_str(")")));
                                    }

                                    if paren_count == 0 {

                                    // param_gen.set_func_body(
                                    // SExpr::SList(func_body_list.clone()));
                                        let param_list =
                                            RLList::<SExpr>::new_with_list(
                                                &lambda_param);

                                        lambda_list.push_back(
                                            SExpr::SList(param_list));

                                        let body_list =
                                            RLList::<SExpr>::new_with_list(
                                                &lambda_body);

                                        lambda_list.push_back(
                                            SExpr::SList(body_list));

                                        lambda_list.push_back(
                                            SExpr::Symb(
                                                RLSymbol::new_with_str(")")));

                                        println!("lambda_list: {:?}",
                                            lambda_list);
                                        println!("");

                                        let mut lambda_rl =
                                            RLList::<SExpr>::new_with_list(
                                                &lambda_list);

                                        let lcons = lambda_rl.slist_to_cons();

                                        func_body_list.push_back(lcons);

                                        func_body_list.push_back(
                                            SExpr::Symb(
                                                RLSymbol::new_with_str(")")));

                                        let mut func_rl =
                                            RLList::<SExpr>::new_with_list(
                                                &func_body_list);

                                        let cons = func_rl.slist_to_cons();

                                        println!("cons: {:?}", cons);
                                        println!("");

                                        list.push_back(cons);

                                        println!("list: {:?}", list);
                                        println!("");

                                        break;

                                    }
                                }

                                "lambda" => {
                                    lambda_list.push_back(
                                        SExpr::Symb(
                                            RLSymbol::new_with_str("lambda")));
                                }

                                _ => {
                                    if is_lambda_param {
                                        lambda_param.push_back(
                                        SExpr::Symb(
                                            RLSymbol::new_with_symb(&symb)));
                                    } else {
                                        lambda_body.push_back(
                                        SExpr::Symb(
                                            RLSymbol::new_with_symb(&symb)));
                                    }
                                }
                            }
                        }

                        Token::Atom(atom) => {
                            match &*atom {
                                "()"  | "nil" |
                                "NIL" | "Nil" |
                                "NIl" | "NiL" |
                                "niL" | "nIL" |
                                "nIl" => {
                                    if is_lambda_param {
                                        lambda_param.push_back(
                                            SExpr::Nil(
                                                RLNil::new()));
                                    } else {
                                        lambda_body.push_back(
                                            SExpr::Nil(
                                                RLNil::new()));
                                    }
                                }

                                _ => {
                                    if is_lambda_param {
                                        lambda_param.push_back(
                                            SExpr::Atom(
                                                RLAtom::new(&atom)));
                                    } else {
                                        lambda_body.push_back(
                                            SExpr::Atom(
                                                RLAtom::new(&atom)));
                                    }
                                }
                            }
                        }

                        Token::Eof => break,
                    } // match lexer.next()
                }  // loop
            }

            "lambda" => {
                lambda_list.push_back(
                    SExpr::Symb(RLSymbol::new_with_str("lambda")));

                is_lambda_param = true;

                loop {
                    match lexer.next() {
                        Token::Symb(symb) => {
                            match &*symb.name {
                                "(" => {
                                    paren_count = paren_count + 1;

                                    if is_lambda_param &&
                                       paren_count == 1 {
                                        lambda_param.push_back(
                                            SExpr::Symb(
                                                RLSymbol::new_with_str("(")));
                                    }

                                    if is_lambda_param &&
                                       paren_count == 2 {

                                        lambda_param.push_back(
                                            SExpr::Symb(
                                                RLSymbol::new_with_str("(")));
                                    }

                                    if is_lambda_param.not() {
                                        lambda_body.push_back(
                                            SExpr::Symb(
                                                RLSymbol::new_with_str("(")));
                                    }
                                }

                                ")" => {
                                    paren_count = paren_count - 1;

                                    if is_lambda_param.not() &&
                                       paren_count != 0 {
                                        lambda_body.push_back(
                                            SExpr::Symb(
                                                RLSymbol::new_with_str(")")));
                                    }

                                    if is_lambda_param &&
                                       paren_count == 1 {

                                        is_lambda_param = false;

                                        lambda_param.push_back(
                                            SExpr::Symb(
                                                RLSymbol::new_with_str(")")));
                                    }

                                    if paren_count == 0 {
                                        let param_list =
                                            RLList::<SExpr>::new_with_list(
                                                &lambda_param);

                                        lambda_list.push_back(
                                            SExpr::SList(param_list));

                                        let body_list =
                                            RLList::<SExpr>::new_with_list(
                                                &lambda_body);

                                        lambda_list.push_back(
                                            SExpr::SList(body_list));

                                        lambda_list.push_back(
                                            SExpr::Symb(
                                                RLSymbol::new_with_str(")")));

                                        println!("lambda_list: {:?}",
                                            lambda_list);
                                        println!("");

                                        let mut lambda_rl =
                                            RLList::<SExpr>::new_with_list(
                                                &lambda_list);

                                        let lcons = lambda_rl.slist_to_cons();

                                        println!("lcons: {:?}", lcons);
                                        println!("");

                                        list.push_back(lcons);

                                        println!("list: {:?}", list);
                                        println!("");

                                        break;

                                    }
                                }

                                _ => {
                                    if is_lambda_param {
                                        lambda_param.push_back(
                                        SExpr::Symb(
                                            RLSymbol::new_with_symb(&symb)));
                                    } else {
                                        lambda_body.push_back(
                                        SExpr::Symb(
                                            RLSymbol::new_with_symb(&symb)));
                                    }
                                }
                            }
                        }

                        Token::Atom(atom) => {
                            match &*atom {
                                "()"  | "nil" |
                                "NIL" | "Nil" |
                                "NIl" | "NiL" |
                                "niL" | "nIL" |
                                "nIl" => {
                                    if is_lambda_param {
                                        lambda_param.push_back(
                                            SExpr::Nil(
                                                RLNil::new()));
                                    } else {
                                        lambda_body.push_back(
                                            SExpr::Nil(
                                                RLNil::new()));
                                    }
                                }

                                _ => {
                                    if is_lambda_param {
                                        lambda_param.push_back(
                                            SExpr::Atom(
                                                RLAtom::new(&atom)));
                                    } else {
                                        lambda_body.push_back(
                                            SExpr::Atom(
                                                RLAtom::new(&atom)));
                                    }
                                }
                            }
                        }

                        Token::Eof => break,
                    } // match lexer.next()
                }  // loop
            }

            &_ => {
                form_list.push(Token::Symb(symb));

                loop {
                    match lexer.next() {
                        Token::Symb(symb) => {
                            match &*symb.name {
                                "(" => {
                                    paren_count = paren_count + 1;

                                    form_list.push(Token::Symb(symb));
                                }

                                ")" => {
                                    paren_count = paren_count - 1;

                                    if paren_count == 0 {
                                        form_list.push(Token::Symb(symb));

                                        form_list.reverse();

                                        list.push_back(
                                            SExpr::SForm(form_list.clone()));

                                        form_list.clear();

                                    } else if paren_count < 0 {
                                        break;
                                    } else {
                                        form_list.push(Token::Symb(symb));
                                    }
                                }

                                _ => {
                                    if paren_count == 0 {
                                        list.push_back(
                                            SExpr::SToken(Token::Symb(symb)));

                                    } else {
                                        form_list.push(Token::Symb(symb));
                                    }
                                }
                            }
                        }

                        Token::Atom(atom) => {
                            match &*atom {
                                "()"  | "nil" |
                                "NIL" | "Nil" |
                                "NIl" | "NiL" |
                                "niL" | "nIL" |
                                "nIl" => {
                                    if paren_count == 0 {
                                        list.push_back(
                                            SExpr::SToken(Token::Atom(atom)));
                                    } else {
                                        form_list.push(Token::Atom(atom));
                                    }
                                }

                                _ => {
                                    if paren_count == 0 {
                                        list.push_back(
                                            SExpr::SToken(Token::Atom(atom)));
                                    } else {
                                        form_list.push(Token::Atom(atom));
                                    }
                                }
                            }
                        }

                        Token::Eof => break,
                    } // match lexer.next()
                } // loop
            }
        }
    }

    println!("list: {:?}", list);
    println!("");

    println!("End defun-params");
    println!("");

    param_gen.set_param_order(vec!["name".to_string(),
                                   "lambda_list".to_string(),
                                   "docstring".to_string()]);
    //                             "func_body".to_string()]);


    Ok(list)
}

pub fn progv(param_gen: &mut RLParamGenerator, lexer: &mut RLLexer) ->
    Result<LinkedList<SExpr>, RLError> {

    /*
        symbols values form*
    */

    println!("Hello from PROGV-PARAMS");
    println!("");

    let mut list = LinkedList::<SExpr>::new();

    // match symbols
    let mut symbol_list = LinkedList::<SExpr>::new();

    let mut symbol_second_paren = 0;

    loop {
        match lexer.next() {
            Token::Symb(symb) => {
                match &*symb.name {
                    "(" => {
                        symbol_list.push_back(
                            SExpr::Symb(RLSymbol::new_with_str("(")))
                    }

                    ")" => {
                        symbol_second_paren = symbol_second_paren + 1;

                        if symbol_second_paren == 1 {
                            symbol_list.push_back(
                                SExpr::Symb(RLSymbol::new_with_str(")")));

                        } else {
                            symbol_list.push_back(
                                SExpr::Symb(RLSymbol::new_with_str(")")));

                            param_gen.set_symbol_list(
                                SExpr::SList(
                                    RLList::<SExpr>::new_with_list(
                                        &symbol_list)));

                            println!("SYMBOL-LIST: {:?}", symbol_list);
                            println!("");

                            break;
                        }
                    }

                    _ =>  {
                        symbol_list.push_back(
                            SExpr::Symb(RLSymbol::new_with_symb(&symb)))
                    }
                }
            }

            Token::Atom(atom) => {
                match &*atom {
                    "()"  | "nil" |
                    "NIL" | "Nil" |
                    "NIl" | "NiL" |
                    "niL" | "nIL" |
                    "nIl" => {
                        let err = SimpleError::new(
                            r"Nihil ex nihil. (can't bind NIL)");

                        return Err(RLError::SimpleError(err))
                    }

                    _ => {
                        symbol_list.push_back(
                            SExpr::Symb(RLSymbol::new_with_symb(
                                &Symbol { name: atom.clone() })));
                    }
                }
            }

            Token::Eof => {
                let err = ParseError::new("Parse PROGV",
                    pars_symb::token::Token::Eof);

                    return Err(RLError::ParseError(err))
            }
        }
    } // end loop

    // match values
    let mut value_list = LinkedList::<SExpr>::new();

    let mut value_second_paren = 0;

    loop {
        match lexer.next() {
            Token::Symb(symb) => {
                match &*symb.name {
                    "(" => {
                        value_list.push_back(
                            SExpr::Symb(RLSymbol::new_with_str("(")))
                    }

                    ")" => {
                        value_second_paren = value_second_paren + 1;

                        if value_second_paren == 1 {
                            value_list.push_back(
                                SExpr::Symb(RLSymbol::new_with_str(")")));

                        } else {
                            value_list.push_back(
                                SExpr::Symb(RLSymbol::new_with_str(")")));

                            param_gen.set_value_list(
                                SExpr::SList(
                                    RLList::<SExpr>::new_with_list(
                                       &value_list)));

                            println!("VALUE-LIST: {:?}", value_list);
                            println!("");

                            break;
                        }
                    }

                    _ =>  {
                        value_list.push_back(
                            SExpr::Symb(RLSymbol::new_with_symb(&symb)))
                    }
                }
            }

            Token::Atom(atom) => {
                match &*atom {
                    "()"  | "nil" |
                    "NIL" | "Nil" |
                    "NIl" | "NiL" |
                    "niL" | "nIL" |
                    "nIl" => {
                        value_list.push_back(SExpr::Atom(RLAtom::new(&atom)));
                    }

                    _ => {
                        value_list.push_back(SExpr::Atom(RLAtom::new(&atom)));
                    }
                }
            }

            Token::Eof => {
                let err = ParseError::new("Parse PROGV",
                    pars_symb::token::Token::Eof);

                return Err(RLError::ParseError(err))
            }
        }
    } // end loop

    // match progv exprs
    let mut form_list  = Vec::<Token>::new();

    let mut paren_count = 0;

    loop {
        match lexer.next() {
            Token::Symb(symb) => {
                match &*symb.name {
                    "(" => {
                        paren_count = paren_count + 1;

                        form_list.push(Token::Symb(symb));
                    }

                    ")" => {
                        paren_count = paren_count - 1;

                        if paren_count == 0 {
                            form_list.push(Token::Symb(symb));

                            form_list.reverse();

                            list.push_back(SExpr::SForm(form_list.clone()));

                            form_list.clear();

                        } else if paren_count < 0 {
                            break;
                        } else {
                            form_list.push(Token::Symb(symb));
                        }
                    }

                    _ => {
                        if paren_count == 0 {
                            list.push_back(SExpr::SToken(Token::Symb(symb)))

                        } else {
                            form_list.push(Token::Symb(symb));
                        }
                    }
                }
            }

            Token::Atom(atom) => {
                match &*atom {
                    "()"  | "nil" |
                    "NIL" | "Nil" |
                    "NIl" | "NiL" |
                    "niL" | "nIL" |
                    "nIl" => {
                        if paren_count == 0 {
                            list.push_back(SExpr::SToken(Token::Atom(atom)))
                        } else {
                            form_list.push(Token::Atom(atom));
                        }
                    }

                    _ => {
                        if paren_count == 0 {
                            list.push_back(SExpr::SToken(Token::Atom(atom)));
                        } else {
                            form_list.push(Token::Atom(atom));
                        }
                    }
                }
            }

            Token::Eof => break,
        }
    }

    println!("list: {:?}", list);
    println!("");

    param_gen.set_param_order(vec!["symbol_list".to_string(),
                                   "value_list".to_string()]);

    Ok(list)
}

pub fn block(param_gen: &mut RLParamGenerator, lexer: &mut RLLexer) ->
    Result<LinkedList<SExpr>, RLError> {

    println!("Hello from BLOCK-PARAMS");
    println!("");

    let mut list = LinkedList::<SExpr>::new();

    // match block-name
    match lexer.next() {
        Token::Atom(atom) => {
            match &*atom {
                "()"  | "nil" |
                "NIL" | "Nil" |
                "NIl" | "NiL" |
                "niL" | "nIL" |
                "nIl" => {
                    param_gen.set_name(
                        SExpr::Symb(RLSymbol::new_with_symb(
                            &Symbol { name: atom.clone() })));
                }

                _ => {
                    param_gen.set_name(
                        SExpr::Symb(RLSymbol::new_with_symb(
                            &Symbol { name: atom.clone() })));
                }
            }
        }

        Token::Symb(symb) => {
            param_gen.set_name(
                SExpr::Symb(RLSymbol::new_with_symb(
                    &Symbol { name: symb.name.clone() })));
        }

        Token::Eof => {
            let err = ParseError::new("Parse Block",
                pars_symb::token::Token::Eof);

            return Err(RLError::ParseError(err))
        }
    }

    let mut form_list  = Vec::<Token>::new();

    let mut paren_count = 0;

    loop {
        match lexer.next() {
            Token::Symb(symb) => {
                match &*symb.name {
                    "(" => {
                        paren_count = paren_count + 1;

                        form_list.push(Token::Symb(symb));
                    }

                    ")" => {
                        paren_count = paren_count - 1;

                        if paren_count == 0 {
                            form_list.push(Token::Symb(symb));

                            form_list.reverse();

                            list.push_back(
                                SExpr::SForm(form_list.clone()));

                            form_list.clear();

                        } else if paren_count < 0 {
                            break;
                        } else {
                            form_list.push(Token::Symb(symb));
                        }
                    }

                    _ => {
                        if paren_count == 0 {
                            list.push_back(SExpr::SToken(Token::Symb(symb)));
                        } else {
                            form_list.push(Token::Symb(symb));
                        }
                    }
                }
            }

            Token::Atom(atom) => {
                match &*atom {
                    "()"  | "nil" |
                    "NIL" | "Nil" |
                    "NIl" | "NiL" |
                    "niL" | "nIL" |
                    "nIl" => {
                        if paren_count == 0 {
                            list.push_back(SExpr::SToken(Token::Atom(atom)));
                        } else {
                            form_list.push(Token::Atom(atom));
                        }
                    }

                    _ => {
                        if paren_count == 0 {
                            list.push_back(SExpr::SToken(Token::Atom(atom)));
                        } else {
                            form_list.push(Token::Atom(atom));
                        }
                    }
                }
            }

            Token::Eof => break,
        }
    }

    println!("list: {:?}", list);
    println!("");

    param_gen.set_param_order(vec!["name".to_string()]);

    Ok(list)
}

pub fn return_from(param_gen: &mut RLParamGenerator, lexer: &mut RLLexer) ->
    Result<LinkedList<SExpr>, RLError> {

    println!("Hello from RETURN-FROM-PARAMS");
    println!("");

    let mut list = LinkedList::<SExpr>::new();

    // match block-name
    match lexer.next() {
        Token::Atom(atom) => {
            match &*atom {
                "()"  | "nil" |
                "NIL" | "Nil" |
                "NIl" | "NiL" |
                "niL" | "nIL" |
                "nIl" => {
                    param_gen.set_name(
                        SExpr::Symb(RLSymbol::new_with_symb(
                            &Symbol { name: atom.clone() })));
                }

                _ => {
                    param_gen.set_name(
                        SExpr::Symb(RLSymbol::new_with_symb(
                            &Symbol { name: atom.clone() })));
                }
            }
        }

        Token::Symb(symb) => {
            param_gen.set_name(
                SExpr::Symb(RLSymbol::new_with_symb(
                    &Symbol { name: symb.name.clone() })));
        }

        Token::Eof => {
            let err = ParseError::new("Parse Return-From",
                pars_symb::token::Token::Eof);

            return Err(RLError::ParseError(err))
        }
    }

    let mut form_list  = Vec::<Token>::new();

    let mut paren_count = 0;

    loop {
        match lexer.next() {
            Token::Symb(symb) => {
                match &*symb.name {
                    "(" => {
                        paren_count = paren_count + 1;

                        form_list.push(Token::Symb(symb));
                    }

                    ")" => {
                        paren_count = paren_count - 1;

                        if paren_count == 0 {
                            form_list.push(Token::Symb(symb));

                            form_list.reverse();

                            list.push_back(
                                SExpr::SForm(form_list.clone()));

                            form_list.clear();

                        } else if paren_count < 0 {
                            break;
                        } else {
                            form_list.push(Token::Symb(symb));
                        }
                    }

                    _ => {
                        if paren_count == 0 {
                            list.push_back(SExpr::SToken(Token::Symb(symb)));
                        } else {
                            form_list.push(Token::Symb(symb));
                        }
                    }
                }
            }

            Token::Atom(atom) => {
                match &*atom {
                    "()"  | "nil" |
                    "NIL" | "Nil" |
                    "NIl" | "NiL" |
                    "niL" | "nIL" |
                    "nIl" => {
                        if paren_count == 0 {
                            list.push_back(SExpr::SToken(Token::Atom(atom)));
                        } else {
                            form_list.push(Token::Atom(atom));
                        }
                    }

                    _ => {
                        if paren_count == 0 {
                            list.push_back(SExpr::SToken(Token::Atom(atom)));
                        } else {
                            form_list.push(Token::Atom(atom));
                        }
                    }
                }
            }

            Token::Eof => break,
        }
    }

    println!("list: {:?}", list);
    println!("");

    param_gen.set_param_order(vec!["name".to_string()]);

    Ok(list)
}
