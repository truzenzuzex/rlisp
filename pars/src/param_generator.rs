// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::HashMap;
use std::collections::LinkedList;

use err::err::RLError;

use expr::nil::RLNil;
use expr::sexpr::SExpr;
use expr::symb::RLSymbol;

use hash::hash::RLHash;

use lex::lexer::RLLexer;

use pars_symb::symbol::Symbol;
// use pars_symb::token::Token;


#[derive(Debug)]
pub struct RLParamGenerator {
    // a single symbol
    name: Option<SExpr>,

    // lambda-list of "defun"
    ordinary_lambda_list: Option<SExpr>,

    // docstring of e.g "defun"
    docstring: Option<SExpr>,

    // body of "eval"
    form: Option<SExpr>,

    // the order of params to be generated
    param_order: Vec<String>,

    // list of strings
    string_list: Option<SExpr>,

    // list of symbols, created by a "quote"
    symbol_list: Option<SExpr>,

    // list of values, created by a "quote"
    value_list: Option<SExpr>,

    closure: fn(&mut RLParamGenerator, &mut RLLexer) ->
        Result<LinkedList<SExpr>, RLError>,
}

impl RLParamGenerator {
    pub fn new(closure: fn(&mut RLParamGenerator, &mut RLLexer) ->
                   Result<LinkedList<SExpr>, RLError>) -> RLParamGenerator {

        let name = None;

        let ordinary_lambda_list = None;

        let docstring = None;

        let form = None;

        let param_order = Vec::<String>::new();

        let string_list = None;

        let symbol_list = None;

        let value_list = None;

        let closure = closure;

        RLParamGenerator {
            name,
            ordinary_lambda_list,
            docstring,
            form,
            param_order,
            string_list,
            symbol_list,
            value_list,
            closure,
        }
    }

    pub fn set_name(&mut self, sexpr: SExpr) {
        if let SExpr::Symb(symb) = sexpr {
            self.name = Some(SExpr::Symb(symb));
        } else { }
    }

    pub fn get_name(&self) -> SExpr {
        if let Some(SExpr::Symb(symb)) = &self.name {
            SExpr::Symb(RLSymbol::new_with_symb(
                &Symbol { name: symb.get_symbol_name() }))
        } else {
            SExpr::Nil(RLNil::new())
        }
    }

    pub fn set_lambda_list(&mut self, lambda_list: SExpr) {
        if let SExpr::SList(ll_list) = lambda_list {
            self.ordinary_lambda_list = Some(SExpr::SList(ll_list.clone()));
        } else {}
    }

    pub fn get_lambda_list(&self) -> SExpr {
        if let Some(ll_list) = &self.ordinary_lambda_list {
            ll_list.clone()
        } else {
            SExpr::Nil(RLNil::new())
        }
    }

    pub fn set_docstring(&mut self, sexpr: SExpr) {
        if let SExpr::Atom(atom) = sexpr {
            self.docstring = Some(SExpr::Atom(atom));
        } else {}
    }

    pub fn get_docstring(&self) -> SExpr {
        if let Some(SExpr::Atom(atom)) = &self.docstring {
            SExpr::Atom(atom.clone())
        } else {
            SExpr::Nil(RLNil::new())
        }
    }

    pub fn set_form(&mut self, body_list: SExpr) {
        if let SExpr::SList(form) = body_list {
            self.form = Some(SExpr::SList(form.clone()));
        } else {}
    }

    pub fn get_form(&self) -> SExpr {
        if let Some(form) = &self.form {
            form.clone()
        } else {
            SExpr::Nil(RLNil::new())
        }
    }

    pub fn set_param_order(&mut self, order: Vec<String>) {
        self.param_order = order;
    }

    pub fn set_symbol_list(&mut self, symbol_list: SExpr) {
        if let SExpr::SList(symbol_list) = symbol_list {
            self.symbol_list = Some(SExpr::SList(symbol_list.clone()));
        } else {}
    }

    pub fn get_symbol_list(&self) -> SExpr {
        if let Some(symbol_list) = &self.symbol_list {
            symbol_list.clone()
        } else {
            SExpr::Nil(RLNil::new())
        }
    }

    pub fn set_value_list(&mut self, value_list: SExpr) {
        if let SExpr::SList(value_list) = value_list {
            self.value_list = Some(SExpr::SList(value_list.clone()));
        } else {}
    }

    pub fn get_value_list(&self) -> SExpr {
        if let Some(value_list) = &self.value_list {
            value_list.clone()
        } else {
            SExpr::Nil(RLNil::new())
        }
    }

    pub fn build_param_list(&mut self, mut result_ll: LinkedList<SExpr>) -> LinkedList<SExpr> {
        // let mut result_ll = LinkedList::<SExpr>::new();

        println!("param_order: {:?}", self.param_order);
        println!("");

        println!("start build_param_list - result_ll: {:?}", result_ll);
        println!("");

        for _n in 0..self.param_order.len() {
            if let Some(param) = self.param_order.pop() {
                match &*param {

                    "name" => {
                        if let Some(SExpr::Symb(symb)) = &self.name {
                            result_ll.push_front(SExpr::Symb(symb.clone()))
                        }
                    }

                    "lambda_list" => {
                        if let Some(SExpr::SList(ll_list)) =
                            &self.ordinary_lambda_list {

                            result_ll.push_front(SExpr::SList(ll_list.clone()))
                        }
                    }

                    "docstring" => {
                        if let Some(SExpr::Atom(atom)) = &self.docstring {
                            result_ll.push_front(SExpr::Atom(atom.clone()));
                        }
                    }

                    "form" => {
                        if let Some(SExpr::SList(form)) =
                            &self.form {

                            result_ll.push_front(
                                SExpr::SList(form.clone()))
                        }
                    }

                    "string_list" => {
                        if let Some(SExpr::SList(string_list)) =
                            &self.string_list {

                            result_ll.push_front(
                                SExpr::SList(string_list.clone()))
                        }
                    }

                    "symbol_list" => {
                        if let Some(SExpr::SList(symbol_list)) =
                            &self.symbol_list {

                            result_ll.push_front(
                                SExpr::SList(symbol_list.clone()));
                        }
                    }

                    "value_list" => {
                        if let Some(SExpr::SList(value_list)) =
                            &self.value_list {

                            result_ll.push_front(
                                SExpr::SList(value_list.clone()));
                        }
                    }

                    _ => {}
                }
            }
        }

        println!("end build_param_list - result_ll: {:?}", result_ll);
        println!("");

        result_ll
    }

    pub fn run_closure(&mut self, lexer: &mut RLLexer) ->
        Result<LinkedList<SExpr>, RLError> {


        let result: LinkedList<SExpr> = match self.closure {
                ref mut f  => f(self, lexer)?,
        };

        /*
        match result {
            Ok(res)  => return Ok(res),
            Err(err) => return Err(err),
        }
        */

        Ok(result)
    }
}

pub fn make_param_gens_hash_map() -> HashMap<String, RLParamGenerator, RLHash> {
    HashMap::with_hasher(RLHash { })
}

impl Clone for RLParamGenerator {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            ordinary_lambda_list: self.ordinary_lambda_list.clone(),
            docstring: self.docstring.clone(),
            form: self.form.clone(),
            param_order: self.param_order.clone(),
            string_list: self.string_list.clone(),
            symbol_list: self.symbol_list.clone(),
            value_list:  self.value_list.clone(),
            closure: self.closure.clone(),

            /*
            {
                Some(x) => Some(<for<'a>fn(::expr::sexpr::SExpr) ->
                               Result<RLResult, RLError>>::clone(&x)),
                None    => None,
            },
            */
        }
    }
}
