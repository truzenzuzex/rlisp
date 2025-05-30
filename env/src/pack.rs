// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::hash_map::HashMap;
use std::mem::drop;

use crate::lambda::{RLLambda, make_rllambda_hash_map};
use crate::symb::{RLEnvSymbol, make_rlenvsymbol_hash_map};

use expr::string::RLString;

use hash::hash::{RLHash, clone_hash_map};

#[derive(Debug)]
pub struct RLPackage {
    name: String,

    lambdas: HashMap<String, RLLambda, RLHash>,
    symbols: HashMap<String, RLEnvSymbol, RLHash>,
}

impl RLPackage {
    pub fn new(name: &str) -> RLPackage {

        let name = name.to_string();

        let lambdas = make_rllambda_hash_map();

        let symbols = make_rlenvsymbol_hash_map();

        RLPackage {
            name,
            lambdas,
            symbols,
        }
    }

    pub fn check_for_symbol(&self, symbol: &String) -> bool {
        println!("self.symbols: {:?}",  self.symbols);
        println!("");

        self.symbols.contains_key(symbol)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn check_for_lambda(&mut self, id: &String) -> bool {
        self.lambdas.contains_key(id)
    }

    pub fn set_lambdas_hash_map(&mut self,
                                hash_map: &HashMap<String, RLLambda, RLHash>) {
        self.lambdas = clone_hash_map(&hash_map);
    }

    pub fn get_lambdas_hash_map(&mut self) ->
        &mut HashMap<String, RLLambda, RLHash> {

        &mut self.lambdas
    }

    pub fn get_lambda(&mut self, key: &String) -> Option<&mut RLLambda> {
        if let Some(lambda) = self.lambdas.get_mut(key) {
            return Some(lambda);
        } else { return None }
    }

    pub fn add_lambda(&mut self, id: String , val: RLLambda) {
        self.lambdas.insert(id, val);
    }

    pub fn delete_lambda(&mut self, id: String) {
        self.lambdas.remove(&id);
    }

    pub fn get_symbols_hash_map(&mut self) ->
        &mut HashMap<String, RLEnvSymbol, RLHash> {

        &mut self.symbols
    }

    pub fn get_symbol(&mut self, key: &String) -> Option<&mut RLEnvSymbol> {
       if let Some(symbol) = self.symbols.get_mut(key) {
           return Some(symbol);
       } else { return None }
    }

    pub fn add_symbol(&mut self, symb: String , val: RLEnvSymbol) {
        self.symbols.insert(symb, val);
    }

    pub fn delete_symbol(&mut self, symb: String) {
        self.symbols.remove(&symb);
    }

    pub fn delete_package(self) {
        drop(self)
    }
}

pub fn make_rlpackage_hash_map() -> HashMap<RLString, RLPackage, RLHash> {
    HashMap::with_hasher(RLHash { })
}

impl Clone for RLPackage {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            lambdas: clone_hash_map(&self.lambdas),
            symbols: clone_hash_map(&self.symbols),
        }
    }
}
