// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;
use std::collections::{HashMap, LinkedList};

use crate::ordinary_lambda_list::RLOrdinaryLambdaList;
use crate::var::{RLVar, make_rlvar_hash_map};

use expr::nil::RLNil;
use expr::sexpr::SExpr;

use err::err::RLError;

use hash::hash::{RLHash, clone_hash_map};

use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         0123456789";

const ID_LEN: usize = 8;

#[derive(Debug)]
pub struct RLLambda {
    id: String,

    named_lambda_name: Option<String>,

    parameters: RLOrdinaryLambdaList,

    // declarations

    docstring: Option<String>,

    body: SExpr,

    dyn_env_lambda: HashMap<String, RLVar, RLHash>,
}

impl RLLambda {
    pub fn new(// parameters: RLOrdinaryLambdaList,
               option_name: Option<String>,
               docstring: Option<String>) -> Self {

        let mut rng = rand::thread_rng();

        let id: String = (0..ID_LEN).map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            }).collect();

        println!("ID: {:?}", id);
        println!("");

        let named_lambda_name = option_name;

        let parameters = RLOrdinaryLambdaList::new();

        let docstring = docstring;

        let body = SExpr::Nil(RLNil::new());

        let dyn_env_lambda = make_rlvar_hash_map();

        Self {
            id,
            named_lambda_name,
            parameters,
            docstring,
            body,
            dyn_env_lambda,
        }
    }

    pub fn set_id(&mut self, id: &String) {
        self.id = id.clone();
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn set_named_lambda_name (&mut self, name: &mut String) {
        self.named_lambda_name = Some(name.clone());

        name.push('_');

        name.push_str(self.id.as_str());

        self.set_id(&name);
    }

    pub fn get_named_lambda_name(&self) -> Option<String> {
        self.named_lambda_name.clone()
    }

    pub fn set_doc_string(&mut self, docstring: Option<String>) {
        self.docstring = docstring;
    }

    pub fn get_body(&self) -> SExpr {
        self.body.clone()
    }

    pub fn set_body(&mut self, body: SExpr) {
        self.body = body;
    }

    pub fn init_parameters(&mut self, sexpr: SExpr) -> Result<(), RLError> {
        self.parameters.parse_parameters(sexpr)
    }

    pub fn get_parameters(&self) -> RLOrdinaryLambdaList {
        self.parameters.clone()
    }

    pub fn get_cleaned_req_params(&self) -> LinkedList<SExpr> {
        println!("in get_cleaned_req_params()");
        println!("");

        let mut result_ll = LinkedList::<SExpr>::new();

        println!("self.parameters: {:?}", self.parameters);
        println!("");

        let rl_params = self.parameters.get_required_params();

        if let SExpr::SList(slist) = rl_params {
            let mut params_ll = slist.get_linked_list();

            println!("params_ll: {:?}", params_ll);
            println!("");

            for _n in 0..params_ll.len() {
                let sexpr = params_ll.pop_front().unwrap();

                match sexpr {
                    SExpr::Atom(atom) => {
                        let atom_string = atom.get_atom_string();

                        for item in self.dyn_env_lambda.keys() {
                            println!("item: {:?}", item);
                            println!("");

                            let string = item.clone().to_lowercase();

                            if atom_string.eq(&string) {
                            } else {
                                result_ll.push_back(SExpr::Atom(atom.clone()));
                            }
                        }
                    }

                    SExpr::Symb(_symb) => {}

                    _ => todo!(),
                }
            }
        }

        result_ll
    }

    pub fn get_dyn_env_var(&mut self, key: &str) -> Option<RLVar> {
        if let Some(item) = self.dyn_env_lambda.get(&key.to_string()
                                .to_uppercase()) {
            return Some(item.clone());
        } else {
            return None
        }
    }

    pub fn add_dyn_env_var(&mut self, name: String, var: RLVar) {
        self.dyn_env_lambda.insert(name.to_uppercase(), var);
    }

    pub fn get_ref_dyn_env_lambda(&mut self) ->
        &HashMap<String, RLVar, RLHash> {

        &self.dyn_env_lambda
    }
}

pub fn make_rllambda_hash_map() -> HashMap<String, RLLambda, RLHash> {
    HashMap::with_hasher(RLHash { })
}

impl Clone for RLLambda {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),

            named_lambda_name: self.named_lambda_name.clone(),

            parameters: self.parameters.clone(),

            docstring: match &self.docstring {
                Some(x) => Some(x.clone()),
                None => None,
            },

            body: self.body.clone(),

            dyn_env_lambda: clone_hash_map(&self.dyn_env_lambda),
        }
    }
}

impl fmt::Display for RLLambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RLLambda {
                id,

                named_lambda_name: nl_name,

                parameters: params,

                docstring: _doc,

                body: _b,

                dyn_env_lambda: _dyn_env,
            } => {

                println!("params.get_required_params(): {:?}",
                    params.get_required_params());
                println!("");

                if let Some(name) = &nl_name {
                    write!(f, "#<FUNCTION (LAMBDA {} :in {})  {{{}}}>",
                        params.get_required_params(),
                        name,
                        id)
                } else {
                    write!(f, "#<FUNCTION (LAMBDA {}) {{{}}}>",
                        params.get_required_params(),
                        id)
                }
            }
        }
    }
}
