// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;

use crate::result::RLResult;
use crate::var::RLVar;

use err::err::RLError;

use expr::nil::RLNil;
use expr::sexpr::SExpr;
use expr::string::RLString;

#[derive(Debug)]
pub struct RLDynVar {
    // name: String,

    docstring: Option<RLString>,

    package: String,
    package_formatter: bool,

    value: Option<RLVar>,
}

impl RLDynVar {
    pub fn new( // name: String,
               docstring: Option<RLString>,
               package: String,
               value: RLVar) -> Self {
        // let name = name;

        let docstring = docstring;

        let package = package;
        let package_formatter = false;

        let value = Some(value);

        Self { // name,
               docstring,
               package,
               package_formatter,
               value,
        }
    }

    pub fn package_formatter(&mut self, setter: bool) {
        self.package_formatter = setter
    }

    pub fn get_var(&self) -> Option<RLVar> {
        self.value.clone()
    }

    pub fn set_var(&mut self, val: RLVar) {
        self.value = Some(val);
    }

    pub fn get_doc_string(&self) -> Result<RLResult, RLError> {
        if let Some(string) = &self.docstring {
            Ok(RLResult::StringRes(string.clone()))

        } else {
            Ok(RLResult::NilRes(RLNil::new()))
        }
    }

    pub fn set_doc_string(&mut self, docstring: Option<RLString>) {
        self.docstring = docstring;
    }

    pub fn get_pack_string(&self) -> String {
        self.package.clone()
    }

    pub fn set_pack_string(&mut self, pack: String) {
        self.package = pack;
    }

    pub fn var_to_sexpr(&mut self) -> SExpr {
        let result: SExpr;

        match &self.value {
            Some(RLVar::SAtomVar(SExpr::Atom(atom))) => {
                result = SExpr::Atom(atom.clone());
            }

            /*
            Some(RLVar::SAtomVar(SExpr::Func(atom))) => {
                result = SExpr::Func(atom.clone());
            }
            */

            Some(RLVar::SAtomVar(SExpr::Lambda(atom))) => {
                result = SExpr::Lambda(atom.clone());
            }

            Some(RLVar::NilVar(nil)) => {
                result = SExpr::Nil(nil.clone());
            }

            Some(RLVar::SListVar(SExpr::SList(slist))) => {
                result = SExpr::SList(slist.clone());
            }

            _ => todo!(),
        }
        result
    }
}

impl Clone for RLDynVar {
    fn clone(&self) -> Self {
        Self {
            // name: self.name.clone(),
            //  <Option<RLVar> as Clone>::clone(
            // &self.value).unwrap().clone()

            docstring: match &self.docstring {
                Some(x) => Some(x.clone()),
                None => None,
            },
            package: self.package.clone(),
            package_formatter: self.package_formatter.clone(),
            value: match &self.value {
                Some(x) => Some(x.clone()),
                None => None,
            },
        }
    }
}

impl fmt::Display for RLDynVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RLDynVar {
                docstring: _doc,
                package: pack,
                package_formatter: pack_form,
                value: val
            } => {
                if *pack_form {
                    write!(f, "#<PACKAGE \"{}\">", pack)
                } else {
                    match val {
                        Some(x) => write!(f, "{}", x),
                        None => write!(f, ""),
                    }
                }
            }
        }
    }
}
