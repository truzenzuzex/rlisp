// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;

// use crate::bool::RLBool;

// use err::err::RLError;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct RLString {
    string: String,
}

impl RLString {
    #[allow(unused_assignments)]
    pub fn new(str: &str) -> RLString {
        // str.escape_unicode()

        let str_len = str.len();

        let mut string: String = "".to_string();

        if str.starts_with("\"") &&
           str.ends_with("\"") {

            unsafe {
                string = str.get_unchecked(1..str_len -1).to_string();
            }
        } else {
            string = str.to_string();
        }

        RLString {
            string,
        }
    }

    pub fn get(&self) -> String {
        self.string.clone()
    }

    pub fn set(&mut self, str: &str) {
        self.string = str.to_string();
    }

    pub fn concatenate_strings(&mut self, second_string: &RLString) ->
        RLString {

        self.string = self.string.clone() + &second_string.string.clone();

        RLString { string: self.get() }

        /*
                        let mut result_string = RLString::new("");

                        if linked_list.len() == 0 {
                            return Ok(RLResult::StrRes(
                                result_string.clone()))
                        }

                        let first_item = linked_list.pop_front().unwrap();

                        if let SExpr::Atom(ref _atom) = first_item {

                        } else {
                            let err = TypeError::new(&first_item.to_string(),
                                                    "CHARACTER");
                                return Err(RLError::TypeError(err));
                       }

                       result_string = first_item.get_atom_rlstring();

                       for _n in 0..linked_list.len() {
                           let expr = linked_list.pop_front().unwrap();

                           if let SExpr::Atom(atom) = expr {

                               println!("atom: {:?}", atom);
                               println!("");

                               result_string.concatenate(
                                   &SExpr::Atom(atom).get_atom_rlstring());
                           } else {
                               let err = TypeError::new(&expr.to_string(),
                                                        "CHARACTER");
                               return Err(RLError::TypeError(err));
                           }
                       }
        */
    }

    // rust style wrapper for capitalization
    pub fn to_uppercase(&mut self) -> RLString {
        self.string = self.string.to_uppercase();
        RLString { string: self.string.clone() }
    }

    // lisp style wrapper for capitalization
    pub fn string_capitalize(&mut self) -> RLString {
        self.string = self.string.to_uppercase();
        RLString { string: self.string.clone() }
    }

    /*
    pub fn stringp(&self, string: String) -> RLBool {
        if string.starts_with("\"") &&
           string.ends_with("\"") {

           return RLBool::True;
        } else {
            return RLBool::Nil;
        }
    }
    */
}

impl fmt::Display for RLString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // println!("in fmt for String");
        // println!("self: {:?}", self);
      write!(f, "\"{}\"", &self.string)
    }
}
