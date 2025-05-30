// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;
use std::collections::HashMap;

use crate::block::RLBlock;
use crate::env::RLEnvironment;
use crate::lambda::RLLambda;
use crate::ordinary_lambda_list::RLOrdinaryLambdaList;
use crate::result::RLResult;
use crate::var::RLVar;

use err::err::RLError;

use expr::nil::RLNil;
use expr::sexpr::SExpr;
use expr::string::RLString;

use hash::hash::RLHash;

#[derive(Debug)]
pub struct RLNamedLambda {
    name: String,

    is_macro: bool,

    parameters: RLOrdinaryLambdaList,

    docstring: Option<SExpr>,

    package: String,
    package_formatter: bool,

    // body is either a SExpr which will occur
    // in the correspondings closure
    // body: Option<SExpr>,
    // body: SExpr,

    block: Option<RLBlock>,

    lambda: Option<RLLambda>,

    // Box<dyn FnMut() ... necessary to store closure in a HashMap
    /*
    pub closure: Option<Box<dyn FnMut(SExpr) ->
        Result<RLResult, RLError>>>,

    pub closure_env: Option<Box<dyn FnMut(SExpr, &mut RLEnvironment) ->
        Result<RLResult, RLError>>,
    */

    pub closure: Option<fn(SExpr) ->
        Result<RLResult, RLError>>,

    pub closure_env: Option<fn(SExpr,
                               &mut RLEnvironment) ->
        Result<RLResult, RLError>>,
}

impl RLNamedLambda {
    pub fn new_func(
        name: String,

        docstring: Option<SExpr>,

        pack: String,

        /*
        closure: Option<Box<dyn FnMut(SExpr) -> Result<RLResult, RLError>>>,

        closure_env: Option<Box<dyn FnMut(SExpr, &mut RLEnvironment) ->
            Result<RLResult, RLError>>>) -> Self {
        */

        closure: Option<fn(SExpr) -> Result<RLResult, RLError>>,

        closure_env: Option<fn(SExpr, &mut RLEnvironment) ->
            Result<RLResult, RLError>>) -> Self {

        let name = name.clone();

        let is_macro = false;

        let parameters = RLOrdinaryLambdaList::new();

        let docstring = docstring;

        let package = pack.clone();

        let package_formatter = false;

        // let body = SExpr::Nil;

        let block = Some(RLBlock::new(name.clone().to_uppercase()));

        let lambda = None;

        let closure = closure;

        let closure_env = closure_env;

        Self {
            name,
            is_macro,
            parameters,
            docstring,
            package,
            package_formatter,
            // body,
            block,
            lambda,
            closure,
            closure_env,
        }
    }

    pub fn new_macro(
        name: String,

        docstring: Option<SExpr>,

        pack: String,

        closure: Option<fn(SExpr) -> Result<RLResult, RLError>>,

        closure_env: Option<fn(SExpr, &mut RLEnvironment) ->
            Result<RLResult, RLError>>) -> Self {

        let name = name.clone();

        let is_macro = true;

        let parameters = RLOrdinaryLambdaList::new();

        let docstring = docstring;

        let package = pack.clone();

        let package_formatter = false;

        // let body = SExpr::Nil;

        let block = Some(RLBlock::new(name.clone().to_uppercase()));

        let lambda = None;

        let closure = closure;

        let closure_env = closure_env;

        Self { name,
               is_macro,
               parameters,
               docstring,
               package,
               package_formatter,
               // body,
               block,
               lambda,
               closure,
               closure_env,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_is_macro(&self) -> bool {
        self.is_macro
    }

    pub fn package_formatter(&mut self, setter: bool) {
        self.package_formatter = setter
    }

    pub fn get_closure(&mut self) ->
        Box<dyn FnMut(SExpr, &mut RLEnvironment) ->
            Result<RLResult, RLError> + '_> {
        Box::new(self.closure_env.as_mut().unwrap())
    }

    /*
    pub fn run_closure(&mut self, sexpr: SExpr) ->
        Result<RLResult, RLError> {

        let func = match self.closure {
               Some(ref mut f)  => f(sexpr),
                    _ => panic!("Closure not found."),
        };

        match func {
            Ok(res)  => return Ok(res),
            Err(err) => return Err(err),
        }
    }
    */

    pub fn run_closure(&mut self,
                       sexpr: SExpr,
                       env_ref: &mut RLEnvironment) ->
        Result<RLResult, RLError> {

        let mut result: Result<RLResult, RLError> =
            Ok(RLResult::NilRes(RLNil::new()));

        if let Some(_closure) = self.closure {
            result = match self.closure {
                Some(ref mut f)  => f(sexpr),
                               _ => panic!("Closure not found."),
            };

        } else if let Some(_closure) = self.closure_env {
            result = match self.closure_env {
                Some(ref mut f)  => f(sexpr, env_ref),
                               _ => panic!("Closure_env not found."),
            };
        }

        match result {
            Ok(res)  => return Ok(res),
            Err(err) => return Err(err),
        }
    }

    pub fn get_parameters(&self) -> RLOrdinaryLambdaList {
        self.parameters.clone()
    }

    pub fn set_parameters(&mut self, lambda_list: RLOrdinaryLambdaList) {
        self.parameters = lambda_list;
    }

    pub fn get_doc_string(&self) -> Result<RLResult, RLError> {
        if let Some(SExpr::Atom(atom)) = &self.docstring {
            let string = atom.get_atom_string();
            Ok(RLResult::StringRes(RLString::new(&string)))

        } else {
            Ok(RLResult::NilRes(RLNil::new()))
        }
    }

    pub fn set_doc_string(&mut self, docstring: Option<SExpr>) {
        self.docstring = docstring;
    }

    pub fn get_pack_string(&self) -> String {
        self.package.clone()
    }

    pub fn set_pack_string(&mut self, pack: String) {
        self.package = pack;
    }

    /*
    pub fn get_body(&self) -> SExpr {
        self.body.clone()
    }

    pub fn set_body(&mut self, body: SExpr) {
        self.body = body;
    }
    */

    pub fn get_block(&self) -> Option<RLBlock> {
        if let Some(block) = &self.block {
            Some(block.clone())
        } else {
            None
        }
    }

    pub fn set_block(&mut self, block: &RLBlock) {
        self.block = Some(block.clone())
    }

    pub fn get_lambda(&self) -> Option<RLLambda> {
        if let Some(lambda) = &self.lambda {
            Some(lambda.clone())
        } else {
            None
        }
    }

    pub fn set_lambda(&mut self, lambda: &mut RLLambda) {
        self.block = None;

        let mut name = self.get_name();

        lambda.set_named_lambda_name(&mut name);

        self.lambda = Some(lambda.clone());
    }

    pub fn get_dyn_env_var(&mut self, key: &str) -> Option<RLVar> {
        if let Some(item) = self.block.clone()?.get_dyn_env_var(
                                &key.to_string().to_uppercase()) {
            return Some(item);
        }

        if let Some(item) = self.lambda.clone()?.get_dyn_env_var(
                                &key.to_string().to_uppercase()) {
            return Some(item);
        } else {
            return None;
        }
    }

    pub fn add_dyn_env_var(&mut self, name: String, var: RLVar) {

        if let None = self.block {
            self.lambda.as_mut()
                       .unwrap()
                       .add_dyn_env_var(name.to_uppercase(), var.clone());
        } else {
            self.block.as_mut()
                      .unwrap()
                      .add_dyn_env_var(name.to_uppercase(), var.clone());
        }
    }

    pub fn init_parameters(&mut self, sexpr: SExpr) -> Result<(), RLError> {
        self.parameters.parse_parameters(sexpr)
    }

    pub fn get_ref_dyn_env(&mut self) ->
        &HashMap<String, RLVar, RLHash> {

        if let None = self.block {
            return self.lambda.as_mut()
                              .unwrap()
                              .get_ref_dyn_env_lambda();
        } else {
            return self.block.as_mut()
                             .unwrap()
                             .get_ref_dyn_env_block();
        }
    }
}

impl Clone for RLNamedLambda {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            is_macro: self.is_macro.clone(),
            parameters: self.parameters.clone(),
            docstring: match &self.docstring {
                Some(x) => Some(x.clone()),
                None => None,
            },
            package: self.package.clone(),
            package_formatter: self.package_formatter.clone(),
            // body: self.body.clone(),
            block: self.block.clone(),
            lambda: self.lambda.clone(),
            // named_ds_bind: self.named_ds_bind.clone(),

            closure: match self.closure {
                Some(x) => Some(<for<'a>fn(expr::sexpr::SExpr) ->
                               Result<RLResult, RLError>>::clone(&x)),
                None    => None,
            },

            closure_env: match self.closure_env {
                Some(x) => Some(<for<'a>fn(expr::sexpr::SExpr,
                                           &'a mut RLEnvironment) ->
                               Result<RLResult, RLError>>::clone(&x)),
                None    => None,
            },

            // body_function: self.body_function.clone(),
            // dyn_env_func: clone_hash_map(&self.dyn_env_func),
        }
    }
}

impl fmt::Display for RLNamedLambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RLNamedLambda {
                name: n,
                is_macro: _b,
                parameters: _params,
                docstring: _doc,
                package: pack,
                package_formatter: pack_form,
                // body: _body,
                block: _block,
                lambda: _lambda,
                closure: _clos,
                closure_env: _clos_env,
            } => {
                if *pack_form {
                    write!(f, "#<PACKAGE \"{}\">", pack)
                } else {
                    write!(f, "#<FUNCTION {}>", n)
                }
            }
        }
    }
}
