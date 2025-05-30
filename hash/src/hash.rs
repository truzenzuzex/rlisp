// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::hash::Hash;

#[derive (Debug, Clone)]
pub struct RLHash { }

impl RLHash { }

impl BuildHasher for RLHash {
    type Hasher = DefaultHasher;

    fn build_hasher(&self) -> DefaultHasher { DefaultHasher::default() }
}

pub fn clone_hash_map<K: Hash + Clone, V: Clone, RLHash: Clone>(
    hash_map: &HashMap<K, V, RLHash>) -> HashMap<K, V, RLHash> {

    let hash_clone: HashMap<K, V, RLHash> =
        <HashMap<K, V, RLHash> as Clone>::clone(hash_map).clone();

    hash_clone
}
