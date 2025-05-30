// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::block::RLBlock;
use crate::dyn_var::RLDynVar;
use crate::env_trait::{EnvRef, EnvTrait};
use crate::pack::{RLPackage, make_rlpackage_hash_map};
use crate::symb::RLEnvSymbol;
use crate::result::RLResult;

use crate::var::{RLVar, make_rlvar_hash_map};

use err::err::RLError;

use expr::bool::RLBool;
use expr::sexpr::SExpr;
use expr::string::RLString;
use expr::t::RLT;

use hash::hash::{RLHash, clone_hash_map};

/*
use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         0123456789";

const ID_LEN: usize = 8;
*/

#[derive(Debug)]
pub struct RLEnvironment {
    pub packages: HashMap<RLString, RLPackage, RLHash>,

    // Current Package:
    current_package: RLString,
    package_formatter: bool,

    // Chain of Blocks in Evaluation
    block_chain: Vec<RLBlock>,

    // toplevel RLVar dyn_env
    toplevel_dyn_env: HashMap<String, RLVar, RLHash>,

    // curr_dyn_env for block and function evaluation
    curr_eval_dyn_env: Option<HashMap<String, RLVar, RLHash>>,

    dyn_env_stack: Vec<HashMap<String, RLVar, RLHash>>,
}

impl RLEnvironment {
    pub fn new() -> EnvRef {

        let packages = make_rlpackage_hash_map();

        let cl_user_pack_string = RLString::new("COMMON-LISP-USER");

        let package_formatter = false;

        let block_chain = Vec::<RLBlock>::new();

        let toplevel_dyn_env = make_rlvar_hash_map();

        // let curr_pack_dyn_env = None;

        let curr_eval_dyn_env = None;

        let dyn_env_stack = Vec::new();

        Rc::new(RefCell::new(Self {
            packages: packages,

            current_package: cl_user_pack_string,

            package_formatter: package_formatter,

            block_chain: block_chain,

            toplevel_dyn_env: toplevel_dyn_env,

            curr_eval_dyn_env: curr_eval_dyn_env,

            dyn_env_stack: dyn_env_stack,
        }))
    }

    pub fn init(&mut self) {
        let cl_package = RLPackage::new("COMMON-LISP");

        let cl_user_package = RLPackage::new("COMMON-LISP-USER");

        let mut keyword_package = RLPackage::new("KEYWORD");

        keyword_package.add_symbol(
            ":compile-toplevel".to_string(),
            RLEnvSymbol::new_var(":compile-toplevel".to_string(),
                                 RLDynVar::new(None,
                                               "KEYWORD".to_string(),
                                               RLVar::BoolVar(
                                                   RLBool::T(RLT::new())))));

        keyword_package.add_symbol(
            ":load-toplevel".to_string(),
            RLEnvSymbol::new_var(":load-toplevel".to_string(),
                                 RLDynVar::new(None,
                                               "KEYWORD".to_string(),
                                               RLVar::BoolVar(
                                                   RLBool::T(RLT::new())))));

        keyword_package.add_symbol(
            ":execute".to_string(),
            RLEnvSymbol::new_var(":execute".to_string(),
                                 RLDynVar::new(None,
                                               "KEYWORD".to_string(),
                                               RLVar::BoolVar(
                                                   RLBool::T(RLT::new())))));

        self.add_package(RLString::new("COMMON-LISP"), cl_package);

        self.add_package(RLString::new("COMMON-LISP-USER"), cl_user_package);

        self.add_package(RLString::new("KEYWORD"), keyword_package);

        self.curr_eval_dyn_env = Some(clone_hash_map(&self.toplevel_dyn_env));
        self.dyn_env_stack.push(clone_hash_map(&self.toplevel_dyn_env));
    }

    pub fn package_formatter(&mut self, setter: bool) {
        self.package_formatter = setter
    }
}

impl EnvTrait for RLEnvironment {
    fn init(&mut self) {
        self.init();
    }

    fn hello(&self) {
       println!("Hello, hello from env");
       println!("");
    }

    fn is_keyword(&self, mut keyword: String) -> bool {
        let rest = keyword.split_off(1);

        let mut chars = rest.chars();

        let mut low = true;

        for _n in 0..rest.len() {
            if chars.next().unwrap().is_lowercase() {
                low = true;
            } else {
                low = false;
            }
        };

        if keyword.starts_with(":") &&
           low {
            return true;
        } else {
            return false;
        }
    }

    fn package_formatter(&mut self, setter: bool) {
        self.package_formatter(setter)
    }

    // get/add a package to packages hash_map
    fn get_ref_package(&self, key: &RLString) -> Option<&RLPackage> {
        if let Some(pack) = self.packages.get(key) {
            return Some(pack);
        } else { return None }
    }

    fn get_package(&mut self, key: &RLString) -> Option<&mut RLPackage> {
        if let Some(pack) = self.packages.get_mut(key) {
            return Some(pack);
        } else { return None }
    }

    fn add_package(&mut self, id: RLString , val: RLPackage) {
        self.packages.insert(id, val);
    }

    // COMMON-LISP Package
    fn get_mut_cl_package(&mut self) -> &mut RLPackage {
        self.get_package(&RLString::new("COMMON-LISP")).unwrap()
    }

    // COMMON-LISP-USER Package
    fn get_mut_cl_user_package(&mut self) -> &mut RLPackage {
        self.get_package(&RLString::new("COMMON-LISP-USER")).unwrap()
    }

    // KEYWORD Package
    fn get_mut_keyword_package(&mut self) -> &mut RLPackage {
        self.get_package(&RLString::new("KEYWORD")).unwrap()
    }

    fn get_mut_current_package(&mut self) -> &mut RLPackage {
        self.get_package(&self.get_name_current_package()).unwrap()
    }

    fn delete_package(&mut self, id: RLString) {
        self.packages.remove(&id);
    }

    fn get_name_current_package(&self) -> RLString {
        self.current_package.clone()
    }

    fn set_name_current_package(&mut self, name: &RLString) {
        self.current_package = name.clone()
    }

    //////////////////////////////////////////////////////////

    // block chain functions
    fn get_block_chain_len(&self) -> usize {
        self.block_chain.len()
    }

    fn block_chain_push(&mut self, block: &RLBlock) {
        self.block_chain.push(block.clone());
    }

    fn block_chain_pop(&mut self) -> Option<RLBlock> {
        self.block_chain.pop()
    }

    fn is_in_block_chain(&self, block_name: String) -> bool {
        let mut is_in = false;

        let mut block_chain = self.block_chain.clone();

        block_chain.retain(|x| if x.get_name().eq(&block_name) {
                is_in = true;
                true
            } else {
                is_in = false;
                true
        });

        is_in
    }

    fn block_chain_clear(&mut self) {
        self.block_chain.clear()
    }

    fn remove_from_block_chain(&mut self, block_name: String) {
        self.block_chain.retain(|x| if x.get_name().eq(&block_name) {
                false
            } else {
                true
        })
    }

    fn block_chain_is_empty(&mut self) -> bool {
        self.block_chain.is_empty()
    }

    fn block_chain_retain(&mut self, name: &String) -> Vec<RLBlock> {
        let block_name = name.clone().to_uppercase();

        let mut block_chain = self.block_chain.clone();

        block_chain.retain(|x| x.get_name().eq(&block_name));
        block_chain
    }

    fn get_block_chain(&mut self) -> Vec<RLBlock> {
        self.block_chain.clone()
    }

    fn get_ref_mut_block_chain(&mut self) -> &mut Vec<RLBlock> {
        &mut self.block_chain
    }

    fn get_last_of_block_chain(&mut self) -> Option<RLBlock> {
        if let Some(block) = self.block_chain.pop() {
                self.block_chain.push(block.clone());
                Some(block)
        } else {
            None
        }
    }

    fn get_block_by_id(&mut self, id: &String) -> Option<RLBlock> {
        let mut block_chain = self.block_chain.clone();

        block_chain.retain(|x| x.get_id().eq(id));

        if let Some(block) = block_chain.pop() {
            Some(block)
        } else {
            None
        }
    }

    //////////////////////////////////////////////////////////////

    // RLSymbol related functions
    /*
    fn run_closure(&mut self,
                   sexpr: SExpr,
                   mut closure: Box<fn(SExpr, &mut RLEnvironment) ->
                       Result<RLResult, RLError>>) ->
        Result<RLResult, RLError> {

        let func_env = match closure {
               ref mut f => f(sexpr, self),
                    _ => panic!("Closure not found."),
        };

        match func_env {
            Ok(res)  => return Ok(res),
            Err(err) => return Err(err),
        }
    }
    */

    /*
    fn run_closure(&mut self,
                   sexpr: SExpr,
                   mut closure: fn(SExpr, &mut RLEnvironment) ->
                       Result<RLResult, RLError>) ->
        Result<RLResult, RLError> {

        let func_env = match closure {
               ref mut f => f(sexpr, self),
                  //  _ => unreachable!(),
        };

        match func_env {
            Ok(res)  => return Ok(res),
            Err(err) => return Err(err),
        }
    }
    */

    fn run_closure_rl_symbol(&mut self,
                             sexpr: SExpr,
                             rl_symbol: &mut RLEnvSymbol) ->
        Result<RLResult, RLError> {

        rl_symbol.run_closure(sexpr, self)
    }

    fn symbol_in_current_package(&self, key: &String) -> bool {
        let name_curr_pack = self.get_name_current_package();

        println!("name_curr_pack: {:?}", name_curr_pack);
        println!("");

        let curr_pack = self.get_ref_package(&name_curr_pack).expect("REASON");
        println!("curr_pack: {:?}", curr_pack.get_name());
        println!("");


        curr_pack.check_for_symbol(key)
    }

    fn get_cl_symbol(&mut self, key: &String) -> Option<&mut RLEnvSymbol> {
        let cl_pack = self.get_mut_cl_package();

        if let Some(item) = cl_pack.get_symbol(key) {
            return Some(item);
        } else {
            None
        }
    }

    #[allow(unused_assignments)]
    fn get_symbol(&mut self, key: &String) -> Option<&mut RLEnvSymbol> {
        println!("begin get_symbol...");

        let mut pack: Option<&mut RLPackage> = None;

        let is_keyword = self.is_keyword(key.clone());
        let name_curr_pack = self.get_name_current_package();
        let key_in_curr_pack = self.symbol_in_current_package(key);

        if is_keyword {
            pack = Some(self.get_mut_keyword_package());

        } else if name_curr_pack.get().eq("COMMON-LISP-USER") &&
                  key_in_curr_pack {
            pack = Some(self.get_mut_cl_user_package());

        } else if name_curr_pack.get().eq("COMMON-LISP-USER") &&
                  !key_in_curr_pack {
            pack = Some(self.get_mut_cl_package());

        } else {
            pack = self.get_package(&name_curr_pack);
        }

        if let Some(item) = pack?.get_symbol(key) {
            return Some(item);
        } else {
            None
        }
    }

    fn add_symbol(&mut self, symb: String , val: RLEnvSymbol) {
        if let Some(pack) = self.get_package(&self.get_name_current_package()) {
            pack.add_symbol(symb, val)
        } else { }
    }

    fn delete_symbol(&mut self, symb: String) {
        if let Some(pack) = self.get_package(&self.get_name_current_package()) {
            pack.delete_symbol(symb)
        } else { }
    }

    /////////////////////////////////////////////////////////////////////

    /*
    fn generate_id(&self, named_lambda: &RLNamedLambda) -> String {
        let mut id = named_lambda.get_name();

        let lambda_id: String;

        if let Some(lambda) = named_lambda.get_lambda() {
            lambda_id = lambda.get_id();
        } else {

            let mut rng = rand::thread_rng();

            lambda_id = (0..ID_LEN).map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            }).collect();
        }

        id.push('_');

        id.push_str(lambda_id.as_str());

        println!("ID: {:?}", id);
        println!("");

        id
    }

    fn check_for_named_lambda(&mut self, id: &String) -> bool {
        self.named_lambdas.contains_key(id)
    }

    fn get_named_lambdas_hash_map(&mut self) ->
        &mut HashMap<String, RLNamedLambda, RLHash> {

        &mut self.named_lambdas
    }

    fn get_named_lambda(&mut self, key: &String) ->
        Option<&mut RLNamedLambda> {

        if let Some(named_lambda) = self.named_lambdas.get_mut(key) {
            return Some(named_lambda);
        } else { return None }
    }

    fn add_named_lambda(&mut self, id: String , val: RLNamedLambda) {
        self.named_lambdas.insert(id, val);
    }

    fn delete_named_lambda(&mut self, id: String) {
        self.named_lambdas.remove(&id);
    }
    */

    /////////////////////////////////////////////////////////////////////

    // curr_eval_dyn_env related functions
    fn get_curr_eval_dyn_env_var(&mut self, key: &str) -> Option<RLVar> {

        if let Some(ref mut map) = self.curr_eval_dyn_env {
            if let Some(item) = map.get(&key.to_string()
                                            .to_uppercase()) {
                return Some(item.clone());
            } else {
                return None
            }
        } else {
            return None;
        }
    }

    fn add_curr_eval_dyn_env_var(&mut self, name: String, var: RLVar) {
        if let Some(ref mut map) = self.curr_eval_dyn_env {
            map.insert(name.to_uppercase(), var);
        }
    }

    fn replace_curr_eval_dyn_env(&mut self,
                                 map: &HashMap<String, RLVar, RLHash>) {

        if let Some(hash_map) = &self.curr_eval_dyn_env {
            self.dyn_env_stack.push(clone_hash_map(&hash_map));
        }

        self.curr_eval_dyn_env = None;
        self.curr_eval_dyn_env = Some(clone_hash_map(&map));
    }

    fn release_curr_eval_dyn_env(&mut self) {
        if self.dyn_env_stack.len() > 1 {
            if let Some(hash_map) = self.dyn_env_stack.pop() {

                self.curr_eval_dyn_env = None;
                self.curr_eval_dyn_env = Some(hash_map);
            }
        } else if self.dyn_env_stack.len() == 1 {
            if let Some(hash_map) = self.dyn_env_stack.pop() {
                self.curr_eval_dyn_env = None;
                self.curr_eval_dyn_env = Some(clone_hash_map(&hash_map));

                self.toplevel_dyn_env = clone_hash_map(&hash_map);
                self.dyn_env_stack.push(clone_hash_map(&hash_map));
            }
        }
    }

    fn is_toplevel(&self) -> bool {
        if self.dyn_env_stack.len() == 1 {
            return true;
        } else {
            return false;
        }
    }

    fn get_ref_curr_eval_dyn_env(&mut self) ->
        &Option<HashMap<String, RLVar, RLHash>> {

        &self.curr_eval_dyn_env
    }
}

impl Clone for RLEnvironment {
    fn clone(&self) -> Self {
        let option_curr_eval_dyn_env: Option<HashMap<String, RLVar, RLHash>>;

        if let Some(map) = &self.curr_eval_dyn_env {
            let cloned_dyn_env = clone_hash_map(&map);
            option_curr_eval_dyn_env = Some(cloned_dyn_env);
        } else {
            option_curr_eval_dyn_env = None;
        }

        Self {
            packages: self.packages.clone(),
            current_package: self.current_package.clone(),
            package_formatter: self.package_formatter.clone(),
            block_chain: self.block_chain.clone(),
            // named_lambdas: self.named_lambdas.clone(),
            toplevel_dyn_env: clone_hash_map(&self.toplevel_dyn_env),
            curr_eval_dyn_env: option_curr_eval_dyn_env,
            dyn_env_stack: self.dyn_env_stack.clone(),
        }
    }
}

impl fmt::Display for RLEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RLEnvironment {
                packages: _packages,

                current_package: curr_pack,
                package_formatter: pack_form,

                block_chain: _block_chain,

                // named_lambdas: _named_lambdas,

                toplevel_dyn_env: _toplevel_dyn_env,

                curr_eval_dyn_env: _curr_eval_dyn_env,

                dyn_env_stack: _dyn_env_stack,
            } => {
                if *pack_form {
                    write!(f, "#<PACKAGE {}>", curr_pack)
                } else {
                    write!(f, "")
                }
            }
        }
    }
}
