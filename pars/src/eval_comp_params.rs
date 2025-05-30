// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::HashMap;
use std::collections::LinkedList;
use std::ops::Not;

use crate::param_generator::RLParamGenerator;

use err::err::{ParseError, RLError};

use expr::atom::RLAtom;
use expr::list::RLList;
use expr::nil::RLNil;
use expr::sexpr::SExpr;
use expr::symb::RLSymbol;

use hash::hash::RLHash;

use lex::lexer::RLLexer;

use pars_symb::symbol::Symbol;
use pars_symb::token::Token;

pub struct EvalCompParams {
}

impl EvalCompParams {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                param_gens: &mut HashMap<String, RLParamGenerator, RLHash>) {
        param_gens.insert("backquote".to_string(),
            RLParamGenerator::new(|param_gen, lexer| {
                Ok::<LinkedList<SExpr>, RLError>(
                    backquote(param_gen, lexer)?) }));

        param_gens.insert("eval".to_string(),
            RLParamGenerator::new(|param_gen, lexer| {
                Ok::<LinkedList<SExpr>, RLError>(
                    eval(param_gen, lexer)?) }));

        param_gens.insert("lambda".to_string(),
            RLParamGenerator::new(|param_gen, lexer| {
                Ok::<LinkedList<SExpr>, RLError>(
                    lambda(param_gen, lexer)?) }));

        param_gens.insert("quote".to_string(),
            RLParamGenerator::new(|param_gen, lexer| {
                Ok::<LinkedList<SExpr>, RLError>(
                    quote(param_gen, lexer)?) }));

        param_gens.insert("defmacro".to_string(),
            RLParamGenerator::new(|param_gen, lexer| {
                Ok::<LinkedList<SExpr>, RLError>(
                    defmacro(param_gen, lexer)?) }));
    }
}

pub fn backquote(_param_gen: &mut RLParamGenerator, lexer: &mut RLLexer) ->
    Result<LinkedList<SExpr>, RLError> {

    /*
    Syntax:
    */

    println!("Hello from BACKQUOTE-PARAMS");
    println!("");

    let mut list = LinkedList::<SExpr>::new();

    let mut slist = LinkedList::<SExpr>::new();

    let mut found_list = false;

    let mut paren_count = 0;

    // match quoted expr
    loop {
        match lexer.next() {
            Token::Symb(symb) => {
                match &*symb.name {
                    "(" => {
                        found_list = true;

                        paren_count = paren_count + 1;

                        slist.push_back(
                            SExpr::Symb(RLSymbol::new_with_str("(")));
                    }

                    ")" => {
                        paren_count = paren_count - 1;

                        if found_list &&
                           paren_count == 0 {
                            slist.push_back(
                                SExpr::Symb(RLSymbol::new_with_str(")")));

                            list.push_back(
                                SExpr::SList(
                                    RLList::<SExpr>::new_with_list(&slist)));

                            break;
                        } else if found_list &&
                                  paren_count != 0 {

                            slist.push_back(
                                SExpr::Symb(RLSymbol::new_with_str(")")));

                        } else if found_list.not() {
                            // throw an error
                        }
                    }

                    _ => {
                        if found_list {
                            slist.push_back(
                                SExpr::Symb(RLSymbol::new_with_symb(&symb)));
                        } else {
                            list.push_back(
                                SExpr::Symb(RLSymbol::new_with_symb(&symb)));
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
                        if found_list {
                            slist.push_back(SExpr::Nil(RLNil::new()));
                        } else {
                            list.push_back(SExpr::Nil(RLNil::new()));

                            break;
                        }
                    }

                    _ => {
                        if found_list {
                            slist.push_back(SExpr::Atom(RLAtom::new(&atom)));
                        } else {
                            list.push_back(SExpr::Atom(RLAtom::new(&atom)));

                            break;
                        }
                    }
                }
            }

            Token::Eof => break,
        }
    } // loop

    Ok(list)
}

pub fn eval(param_gen: &mut RLParamGenerator, lexer: &mut RLLexer) ->
    Result<LinkedList<SExpr>, RLError> {

    /*
    eval form => result*
    */

    println!("Hello from EVAL-PARAMS");
    println!("");

    let mut list = LinkedList::<SExpr>::new();

    // match SExpr::Cons, form to be evaluated
    let mut form = LinkedList::<SExpr>::new();

    let mut paren_count = 0;

    loop {
        match lexer.next() {
            Token::Symb(symb) => {
                match &*symb.name {
                    "(" => {
                        paren_count = paren_count + 1;

                        form.push_back(
                            SExpr::Symb(RLSymbol::new_with_str("(")));
                    }

                    ")" => {
                        paren_count = paren_count - 1;

                        if paren_count == 0 {
                            form.push_back(
                                SExpr::Symb(RLSymbol::new_with_str(")")));

                            list.push_back(
                                SExpr::SList(
                                    RLList::<SExpr>::new_with_list(&form)));

                            break;
                        } else if paren_count < 0 {
                            param_gen.set_form(
                                SExpr::SList(
                                    RLList::<SExpr>::new_with_list(&form)));

                            list.push_back(
                                SExpr::SList(
                                    RLList::<SExpr>::new_with_list(&form)));

                            break;
                        } else {
                            form.push_back(
                                SExpr::Symb(RLSymbol::new_with_str(")")));
                        }
                    }

                    _ => {
                        form.push_back(
                            SExpr::Symb(RLSymbol::new_with_symb(&symb)));
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
                        form.push_back(SExpr::Nil(RLNil::new()));
                    }

                    _ => {
                        form.push_back(SExpr::Atom(RLAtom::new(&atom)));
                    }
                }
            }

            Token::Eof => break,
        }; // match lexer.next()
    }  // loop

    param_gen.set_param_order(vec!["form".to_string()]);
    Ok(list)
}

pub fn lambda(param_gen: &mut RLParamGenerator, lexer: &mut RLLexer) ->
    Result<LinkedList<SExpr>, RLError> {

    /*
    (lambda lambda-list [[declaration* | documentation]] form*)
    */

    println!("Hello from LAMBDA-PARAMS");
    println!("");

    let mut list = LinkedList::<SExpr>::new();

    let mut lambda_list = LinkedList::<SExpr>::new();

    // match lambda expr
    loop {
        match lexer.next() {
            Token::Symb(symb) => {
                match &*symb.name {
                    "(" => {
                        lambda_list.push_back(
                            SExpr::Symb(RLSymbol::new_with_str("(")))
                    }

                    ")" => {
                        lambda_list.push_back(
                            SExpr::Symb(RLSymbol::new_with_str(")")));

                        param_gen.set_lambda_list(
                            SExpr::SList(
                                RLList::<SExpr>::new_with_list(&lambda_list)));

                        /*
                        list.push_back(
                            SExpr::SList(
                                RLList::<SExpr>::new_with_list(&lambda_list)));
                        */

                        break;
                    }

                    _ =>  {
                        lambda_list.push_back(
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
                        let err = ParseError::new(
                            "NIL cannot be used as a local variable.",
                            pars_symb::token::Token::Atom(atom));

                        return Err(RLError::ParseError(err))
                    }

                    _ => {
                        lambda_list.push_back(SExpr::Atom(RLAtom::new(&atom)));
                    }
                }
            }

            Token::Eof => break,
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
            let err = ParseError::new("Parse LAMBDA",
                pars_symb::token::Token::Eof);

            return Err(RLError::ParseError(err))
        }
    }

    // match SExpr::Cons, function body
    let mut form = LinkedList::<SExpr>::new();

    let mut paren_count = 0;

    loop {
        match lexer.next() {
            Token::Symb(symb) => {
                match &*symb.name {
                    "(" => {
                        paren_count = paren_count + 1;

                        form.push_back(
                            SExpr::Symb(RLSymbol::new_with_str("(")));

                        /*
                        param_gen.push_func_body_tokens(Token::Symb(symb));
                        */
                    }

                    ")" => {
                        paren_count = paren_count - 1;

                        if paren_count == 0 {
                            form.push_back(
                                SExpr::Symb(RLSymbol::new_with_str(")")));

                            list.push_back(
                                SExpr::SList(
                                    RLList::<SExpr>::new_with_list(&form)));

                            break;
                        } else if paren_count < 0 {
                            param_gen.set_form(
                                SExpr::SList(
                                    RLList::<SExpr>::new_with_list(&form)));

                            list.push_back(
                                SExpr::SList(
                                    RLList::<SExpr>::new_with_list(&form)));

                            break;
                        } else {
                            form.push_back(
                                SExpr::Symb(RLSymbol::new_with_str(")")));
                        }
                    }

                    _ => {
                        form.push_back(
                            SExpr::Symb(RLSymbol::new_with_symb(&symb)));
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
                        form.push_back(SExpr::Nil(RLNil::new()));
                    }

                    _ => {
                        form.push_back(SExpr::Atom(RLAtom::new(&atom)));
                    }
                }
            }

            Token::Eof => break,
        }; // match lexer.next()
    }  // loop

    param_gen.set_param_order(vec!["lambda_list".to_string(),
                                   "docstring".to_string(),
                                   "form".to_string()]);
    Ok(list)
}

pub fn quote(_param_gen: &mut RLParamGenerator, lexer: &mut RLLexer) ->
    Result<LinkedList<SExpr>, RLError> {

    /*
    Syntax:
    */

    println!("Hello from QUOTE-PARAMS");
    println!("");

    let mut list = LinkedList::<SExpr>::new();

    let mut slist = LinkedList::<SExpr>::new();

    let mut found_list = false;

    let mut paren_count = 0;

    // match quoted expr
    loop {
        match lexer.next() {
            Token::Symb(symb) => {
                match &*symb.name {
                    "(" => {
                        println!("(");
                        found_list = true;

                        paren_count = paren_count + 1;

                        slist.push_back(
                            SExpr::Symb(RLSymbol::new_with_str("(")));
                    }

                    ")" => {
                        paren_count = paren_count - 1;

                        if found_list &&
                           paren_count == 0 {
                            println!("vvvvv");
                            slist.push_back(
                                SExpr::Symb(RLSymbol::new_with_str(")")));

                            list.push_back(
                                SExpr::SList(
                                    RLList::<SExpr>::new_with_list(&slist)));

                            break;
                        } else if found_list &&
                                  paren_count != 0 {

                            println!("uuuuuu");
                            slist.push_back(
                                SExpr::Symb(RLSymbol::new_with_str(")")));

                        } else if found_list.not() {
                            // throw an error
                        }
                    }

                    _ => {
                        if found_list {
                           // paren_count == 0 {
                            slist.push_back(
                                SExpr::Symb(RLSymbol::new_with_symb(&symb)));
                        } else {
                            list.push_back(
                                SExpr::Symb(RLSymbol::new_with_symb(&symb)));

                            break;
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
                        if found_list {
                            slist.push_back(SExpr::Nil(RLNil::new()));
                        } else {
                            list.push_back(SExpr::Nil(RLNil::new()));

                            break;
                        }
                    }

                    _ => {
                        if found_list {
                            slist.push_back(SExpr::Atom(RLAtom::new(&atom)));
                        } else {
                            list.push_back(SExpr::Atom(RLAtom::new(&atom)));

                            break;
                        }
                    }
                }
            }

            Token::Eof => break,
        }
    } // loop

    Ok(list)
}


pub fn defmacro(param_gen: &mut RLParamGenerator, lexer: &mut RLLexer) ->
    Result<LinkedList<SExpr>, RLError> {

    println!("Hello from DEFMACRO-PARAMS");
    println!("");

    let mut list = LinkedList::<SExpr>::new();

    // match macro name
    match lexer.next() {
        Token::Atom(atom) => {

            match &*atom {
                "()"  | "nil" |
                "NIL" | "Nil" |
                "NIl" | "NiL" |
                "niL" | "nIL" |
                "nIl" => {
                    let err = ParseError::new("Parse DEFMACRO",
                        pars_symb::token::Token::Atom(atom));

                    return Err(RLError::ParseError(err))
                }

                _ => {
                    param_gen.set_name(
                        SExpr::Symb(RLSymbol::new_with_symb(
                            &Symbol { name: atom.clone() })));
                }
            }
        }

        Token::Symb(symb) => {
            let err = ParseError::new("Parse DEFMACRO",
                 pars_symb::token::Token::Symb(symb));

            return Err(RLError::ParseError(err))
        }

        Token::Eof => {
            let err = ParseError::new("Parse DEFMACRO",
                pars_symb::token::Token::Eof);

            return Err(RLError::ParseError(err))
        }
    }

    // match lambda_list
    let mut lambda_list = LinkedList::<SExpr>::new();

    loop {
        match lexer.next() {
            Token::Symb(symb) => {

                match &*symb.name {
                    "(" => {
                        lambda_list.push_back(
                            SExpr::Symb(RLSymbol::new_with_str("(")))
                    }

                    ")" => {
                        lambda_list.push_back(
                            SExpr::Symb(RLSymbol::new_with_str(")")));

                        param_gen.set_lambda_list(
                            SExpr::SList(
                                RLList::<SExpr>::new_with_list(&lambda_list)));

                        break;
                    }

                    _ =>  {
                        lambda_list.push_back(
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
                        let err = ParseError::new("Parse DEFMACRO",
                            pars_symb::token::Token::Atom(atom));

                        return Err(RLError::ParseError(err))
                    }

                    _ => {
                        lambda_list.push_back(
                            SExpr::Atom(RLAtom::new(&atom)));
                    }
                }
            }

            Token::Eof => {
                let err = ParseError::new("Parse DEFMACRO",
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
                }

                _ => {
                    param_gen.set_docstring(SExpr::Atom(RLAtom::new(&atom)));
                }
            }
        }

        Token::Symb(_symb) => {
            lexer.restore_token();
        }

        Token::Eof => {
            let err = ParseError::new("Parse DEFMACRO",
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

    param_gen.set_param_order(vec!["name".to_string(),
                                   "lambda_list".to_string(),
                                   "docstring".to_string()]);
    Ok(list)
}
