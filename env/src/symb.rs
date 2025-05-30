// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;
use std::collections::HashMap;

use crate::dyn_var::RLDynVar;
use crate::env::RLEnvironment;
use crate::named_lambda::RLNamedLambda;
use crate::result::RLResult;
use crate::var::RLVar;

use err::err::RLError;

use expr::sexpr::SExpr;

use hash::hash::RLHash;

#[derive(Debug)]
pub struct RLEnvSymbol {
    name: String,

    is_macro: bool,

    home_package: String,

    // p_list: Option<p-list?>,

    pub named_lambda: Option<RLNamedLambda>,
    pub dyn_var: Option<RLDynVar>,
}

impl RLEnvSymbol {
    pub fn new_named_lambda(name: String,
                            named_lambda: RLNamedLambda) -> RLEnvSymbol {

        let name = name;

        let is_macro = named_lambda.get_is_macro();

        let home_package = named_lambda.get_pack_string();

        let named_lambda = Some(named_lambda);
        let dyn_var = None;

        RLEnvSymbol {
            name,
            is_macro,
            home_package,
            named_lambda,
            dyn_var,
        }
    }

    pub fn new_var(name: String,
                   var: RLDynVar) -> RLEnvSymbol {
        let name  = name;

        let is_macro = false;

        let home_package = var.get_pack_string();

        let named_lambda = None;

        let dyn_var = Some(var);

        RLEnvSymbol {
            name,
            is_macro,
            home_package,
            named_lambda,
            dyn_var,
        }
    }

    pub fn get_is_macro(&self) -> bool {
        self.is_macro
    }

    pub fn get_named_lambda(&mut self) -> Option<RLNamedLambda> {
        // <Option<RLNamedLambda> as Clone>::clone(&self.named_lambda)
        self.named_lambda.clone()
    }

    pub fn set_named_lambda(&mut self, named_lambda: &RLNamedLambda) {
        let is_macro = named_lambda.get_is_macro();

        self.is_macro = is_macro;

        self.named_lambda = Some(named_lambda.clone());
    }

    /*
    pub fn run_closure(&mut self, sexpr: SExpr) ->
        Result<SimpleResult, RLError> {

        return self.named_lambda.as_mut()
                                .unwrap()
                                .run_closure(sexpr);
    }
    */

    pub fn run_closure(&mut self,
                       sexpr: SExpr,
                       env_ref: &mut RLEnvironment) ->
        Result <RLResult, RLError> {

        /*
        let _ = self.named_lambda.as_mut()
                                 .unwrap()
                                 .init_parameters(sexpr.clone());
        */

        self.named_lambda.as_mut()
                         .unwrap()
                         .run_closure(sexpr, env_ref)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_pack_name(&self) -> String {
        self.home_package.clone()
    }

    pub fn set_pack_name(&mut self, pack: String) {
        self.home_package = pack;
    }

    pub fn get_dyn_var(&self) -> Option<RLDynVar> {
        self.dyn_var.clone()
    }

    pub fn set_dyn_var(&mut self, dyn_var: RLDynVar) {
        self.dyn_var = Some(dyn_var);
    }

    pub fn get_dyn_var_value(&self) -> Option<RLVar> {
        if let Some(dyn_var) = &self.dyn_var {
            Some(dyn_var.get_var()?)
        } else { None }
    }

    pub fn set_dyn_var_value(&mut self, var: RLVar) {
        if let Some(_dyn_var) = &self.dyn_var  {
            // <Option<RLDynVar> as Clone>::clone(
            //     &self.dyn_var).unwrap().set_var(var)
            self.dyn_var.clone().unwrap().set_var(var)
        }
    }
}

pub fn make_rlenvsymbol_hash_map() -> HashMap<String, RLEnvSymbol, RLHash> {
    HashMap::with_hasher(RLHash { })
}

impl Clone for RLEnvSymbol {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            is_macro: self.is_macro.clone(),
            home_package: self.home_package.clone(),
            named_lambda: match &self.named_lambda {
                Some(x) => Some(x.clone()),
                None => None,
            },
            dyn_var: match &self.dyn_var {
                Some(x) => Some(x.clone()),
                None => None,
            },
        }
    }
}

impl fmt::Display for RLEnvSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}
