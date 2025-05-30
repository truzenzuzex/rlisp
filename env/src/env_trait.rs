// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::block::RLBlock;
// use crate::named_lambda::RLNamedLambda;
use crate::pack::RLPackage;
use crate::result::RLResult;
use crate::symb::RLEnvSymbol;
use crate::var::RLVar;

use err::err::RLError;

use expr::sexpr::SExpr;
use expr::string::RLString;

use hash::hash::RLHash;

pub type EnvRef = Rc<RefCell<dyn EnvTrait>>;

pub trait EnvTrait {
    fn init(&mut self);

    fn hello (&self);

    fn is_keyword(&self, keyword: String) -> bool;

    fn package_formatter(&mut self, setter: bool);

    fn get_ref_package(&self, key: &RLString) -> Option<&RLPackage>;

    fn get_package(&mut self, key: &RLString) -> Option<&mut RLPackage>;

    fn add_package(&mut self, id: RLString , val: RLPackage);

    fn get_mut_cl_package(&mut self) -> &mut RLPackage;

    fn get_mut_cl_user_package(&mut self) -> &mut RLPackage;

    fn get_mut_keyword_package(&mut self) -> &mut RLPackage;

    fn get_mut_current_package(&mut self) -> &mut RLPackage;

    fn delete_package(&mut self, id: RLString);

    fn get_name_current_package(&self) -> RLString;

    fn set_name_current_package(&mut self, name: &RLString);

    //////////////////////////////////////////////////////////

    // RLEnvironment's block_chain related functions
    fn get_block_chain_len(&self) -> usize;

    fn block_chain_push(&mut self, block: &RLBlock);

    fn block_chain_pop(&mut self) -> Option<RLBlock>;

    fn is_in_block_chain(&self, block_name: String) -> bool;

    fn block_chain_clear(&mut self);

    fn remove_from_block_chain(&mut self, block_name: String);

    fn block_chain_is_empty(&mut self) -> bool;

    fn block_chain_retain(&mut self, name: &String) -> Vec<RLBlock>;

    fn get_block_chain(&mut self) -> Vec<RLBlock>;

    fn get_ref_mut_block_chain(&mut self) -> &mut Vec<RLBlock>;

    fn get_last_of_block_chain(&mut self) -> Option<RLBlock>;

    fn get_block_by_id(&mut self, id: &String) -> Option<RLBlock>;

    //////////////////////////////////////////////////////////////

    /* RLSymbol related functions */
    /*
    fn run_closure(&mut self, sexpr: SExpr, rl_symbol: &mut RLSymbol) ->
        Result<RLResult, RLError>;
    */

    /*
    fn run_closure(&mut self,
                   sexpr: SExpr,
                   closure: Box<fn(SExpr, &mut RLEnvironment) ->
                       Result<RLResult, RLError>>) ->
        Result<RLResult, RLError>;
    */

    /*
    fn run_closure(&mut self,
                   sexpr: SExpr,
                   closure: fn(SExpr, &mut RLEnvironment) ->
                       Result<RLResult, RLError>) ->
        Result<RLResult, RLError>;
    */

    fn run_closure_rl_symbol(&mut self,
                             sexpr: SExpr,
                             rl_symbol: &mut RLEnvSymbol) ->
        Result<RLResult, RLError>;

    fn symbol_in_current_package(&self, key: &String) -> bool;

    fn get_cl_symbol(&mut self, key: &String) -> Option<&mut RLEnvSymbol>;

    fn get_symbol(&mut self, key: &String) -> Option<&mut RLEnvSymbol>;

    fn add_symbol(&mut self, symb: String , val: RLEnvSymbol);

    fn delete_symbol(&mut self, symb: String);

    ///////////////////////////////////////////////////////////////

    // control of named_lambdas Hash Map

    /*
    fn generate_id(&self, named_lambda: &RLNamedLambda) -> String;

    fn check_for_named_lambda(&mut self, id: &String) -> bool;

    fn get_named_lambdas_hash_map(&mut self) ->
        &mut HashMap<String, RLNamedLambda, RLHash>;

    fn get_named_lambda(&mut self, key: &String) ->
        Option<&mut RLNamedLambda>;

    fn add_named_lambda(&mut self, id: String , val: RLNamedLambda);

    fn delete_named_lambda(&mut self, id: String);
    */

    ///////////////////////////////////////////////////////////////

    // curr_eval_dyn_env functions
    fn get_curr_eval_dyn_env_var(&mut self, key: &str) -> Option<RLVar>;

    fn add_curr_eval_dyn_env_var(&mut self, name: String, var: RLVar);

    fn replace_curr_eval_dyn_env(&mut self,
                                 map: &HashMap<String, RLVar, RLHash>);

    fn release_curr_eval_dyn_env(&mut self);

    fn is_toplevel(&self) -> bool;

    fn get_ref_curr_eval_dyn_env(&mut self) ->
        &Option<HashMap<String, RLVar, RLHash>>;
}
