// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    pub fn new(name: &str) -> Symbol {
        let name  = name.to_string();

        Symbol { name }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}











/*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    package:  String,

    // p-list: ?,
}

impl Symbol {
    pub fn new(input: String) -> Symbol {
        let name  = input.clone();
        let package = "".to_string();

        Symbol { name, package }
    }

    pub fn new_with_pkg(input: String, pkg: String) -> Symbol {
        let name  = input.clone();
        let package = pkg.clone();

        Symbol { name, package }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}
*/
