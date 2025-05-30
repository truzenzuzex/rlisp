// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;
use std::collections::{HashMap, LinkedList};

// use crate::env::RLEnvironment;
// use crate::result::RLResult;
use crate::var::{RLVar, make_rlvar_hash_map};

// use err::err::RLError;

use expr::sexpr::SExpr;

use hash::hash::{RLHash, clone_hash_map};

use pars_symb::token::Token;

use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         0123456789";

const ID_LEN: usize = 8;

#[derive(Debug)]
pub struct RLBlock {
    id: String,

    name: String,

    progn_ll: LinkedList<SExpr>,

    return_from: bool,

    /*
    closure_env: Option<fn(SExpr,
                           &mut RLEnvironment) -> Result<RLResult, RLError>>,
    */

    dyn_env_block: HashMap<String, RLVar, RLHash>,
}

impl RLBlock {
    pub fn new(name: String,

               /*
               closure_env: Option<fn(SExpr, &mut RLEnvironment) ->
                   Result<RLResult, RLError>>
               */
               ) -> RLBlock {

        let mut rng = rand::thread_rng();

        let id: String = (0..ID_LEN).map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
            }).collect();

        let name = name.clone();

        let progn_ll = LinkedList::<SExpr>::new();

        let return_from = false;

        // let closure_env = closure_env;

        let dyn_env_block = make_rlvar_hash_map();

        RLBlock { id,
                  name,
                  progn_ll,
                  return_from,
                  // closure_env,
                  dyn_env_block,
        }
    }

    pub fn new_with_id_as_name() -> RLBlock {

        let mut rng = rand::thread_rng();

        let id: String = (0..ID_LEN).map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
            }).collect();

        let name = id.clone();

        let progn_ll = LinkedList::<SExpr>::new();

        let return_from = false;

        let dyn_env_block = make_rlvar_hash_map();

        RLBlock { id,
                  name,
                  progn_ll,
                  return_from,
                  dyn_env_block,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_progn_ll(&mut self, progn_ll: &LinkedList<SExpr>) {
        self.progn_ll = progn_ll.clone();
    }

    pub fn get_progn_ll(& self) -> LinkedList<SExpr> {
        self.progn_ll.clone()
    }

    pub fn set_return_from(&mut self, value: bool) {
        self.return_from = value;
    }

    pub fn get_return_from (&self) -> bool {
        self.return_from
    }

    /*
    pub fn get_closure(&mut self) ->
        Box<dyn FnMut(SExpr, &mut RLEnvironment) ->
            Result<RLResult, RLError> + '_> {
        Box::new(self.closure_env.as_mut().unwrap())
    }

    pub fn run_closure(&mut self,
                       sexpr: SExpr,
                       env_ref: &mut RLEnvironment) ->
        Result<RLResult, RLError> {
    }
    */

    pub fn get_dyn_env_var(&mut self, key: &str) -> Option<RLVar> {
        if let Some(item) = self.dyn_env_block.get(&key.to_string()
                                                       .to_uppercase()) {
            return Some(item.clone());
        } else {
            return None
        }
    }

    pub fn add_dyn_env_var(&mut self, name: String, var: RLVar) {
        self.dyn_env_block.insert(name.to_uppercase(), var);
    }

    pub fn get_dyn_env_block(&mut self) -> HashMap<String, RLVar, RLHash> {
        self.dyn_env_block.clone()
    }

    pub fn get_ref_dyn_env_block(&mut self) ->
        &HashMap<String, RLVar, RLHash> {

        &self.dyn_env_block
    }

    pub fn contains_function(&self) -> bool {
        let mut progn_ll = self.progn_ll.clone();

        if let Some(SExpr::SForm(mut vec_token)) = progn_ll.pop_front() {

            vec_token.pop();

            let symb = vec_token.pop();

            match symb {
                Some(Token::Symb(symb)) => {
                    match &*symb.name {
                        "function" => {
                            return true;
                        }

                        &_ => {
                            return false;
                        }
                    }
                }

                None => {
                    return false;
                }

                _ => {
                    return false;
                }
            }
        } else {
            return false;
        }
    }

}

impl Clone for RLBlock {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            progn_ll: self.progn_ll.clone(),
            return_from: self.return_from.clone(),

            /*
            closure_env: match self.closure_env {
                Some(x) => Some(<for<'a>fn(expr::sexpr::SExpr,
                                           &'a mut RLEnvironment) ->
                               Result<RLResult, RLError>>::clone(&x)),
                None    => None,
            },
            */

            /*
            dyn_env_block: match &self.dyn_env_block {
                Some(map) => {
                    Some(clone_hash_map(&map))
                }

                None    => None,
            }
            */

            dyn_env_block: clone_hash_map(&self.dyn_env_block),
        }
    }
}

impl fmt::Display for RLBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RLBlock {
                id: _id,
                name: _name,
                progn_ll,
                return_from: _return_from,
                // closure_env: _closure_env,
                dyn_env_block: _,
            } => {
                write!(f, "{:?}", progn_ll)
                // write!(f, "{}", sexpr)
            }
        }
    }
}
