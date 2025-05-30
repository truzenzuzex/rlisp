// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::LinkedList;

use crate::sexpr::SExpr;

#[derive(Debug, Clone)]
pub struct RLComma {
    list_bq_count: i64,
    elem_bq_count: i64,
    comma_count: i64,

    is_comma_at: bool,
    eval_expr: bool,
    is_list: bool,

    comma_expr: LinkedList<SExpr>,
    str_expr: String,
}

impl RLComma {
    pub fn new() -> RLComma {
        let list_bq_count = 0;
        let elem_bq_count = 0;
        let comma_count = 0;

        let is_comma_at = false;
        let eval_expr = false;
        let is_list = false;

        let comma_expr = LinkedList::<SExpr>::new();
        let str_expr = "".to_string();

        RLComma{
            list_bq_count,
            elem_bq_count,
            comma_count,

            is_comma_at,
            eval_expr,
            is_list,

            comma_expr,
            str_expr,
        }
    }

    pub fn set_list_bq_count(&mut self, bq_count: i64) {
        self.list_bq_count = bq_count;
    }

    pub fn get_list_bq_count(&self) -> i64 {
        self.list_bq_count
    }

    pub fn get_elem_bq_count(&self) -> i64 {
        self.elem_bq_count
    }

    pub fn get_comma_count(&self) -> i64 {
        self.comma_count
    }

    pub fn get_is_comma_at(&self) -> bool {
        self.is_comma_at
    }

    pub fn set_eval_expr(&mut self, value: bool) {
        self.eval_expr = value;
    }

    pub fn get_eval_expr(&self) -> bool {
        self.eval_expr
    }

    pub fn set_is_list(&mut self, value: bool) {
        self.is_list = value;
    }

    pub fn get_is_list(&self) -> bool {
        self.is_list
    }

    pub fn get_comma_expr(&self) -> LinkedList<SExpr> {
        self.comma_expr.clone()
    }

    pub fn get_str_expr(&self) -> String {
        self.str_expr.clone()
    }

    pub fn init_members(&mut self, ll: &LinkedList<SExpr>) {

        /*
                X      ,X      ,@X
             ,',X     ,,X     ,@,X
            ,@',X    ,,@X    ,@,@X
        */

        let mut linked_list = ll.clone();

        match linked_list.pop_back() {
            Some(SExpr::Symb(_symb)) => {
                self.is_list = false;
            }

            Some(SExpr::SList(_slist)) => {
                self.is_list = true;
            }

            Some(SExpr::Atom(_atom)) => {
                self.is_list = false;
            }

            Some(SExpr::Nil(_nil)) => {
                self.is_list = false;
            }

            None => {}

            _ => unreachable!(),
        }

        loop {
            match linked_list.pop_front() {
                Some(SExpr::Symb(symb)) => {

                    let symb_name = symb.get_symbol_name();

                    match &*symb_name {
                        "`"  => {
                            self.elem_bq_count = self.elem_bq_count + 1;

                            self.comma_expr.push_back(
                                SExpr::Symb(symb.clone()));

                            self.str_expr.push_str(&*symb_name);
                        }

                        ","  => {
                            self.comma_count = self.comma_count + 1;

                            self.comma_expr.push_back(
                                SExpr::Symb(symb.clone()));

                            self.str_expr.push_str(&*symb_name);
                        }

                        ",@" => {
                            self.comma_count = self.comma_count + 1;
                            self.is_comma_at = true;

                            self.comma_expr.push_back(
                                SExpr::Symb(symb.clone()));

                            self.str_expr.push_str(&*symb_name);
                        }

                        &_ => {
                            break;
                        }
                    }
                }

                None => {
                    break;
                }

                _ => unreachable!(),
            }
        }
    }

    /*
    pub fn init_eval(&mut self) -> Result<(), RLError> {
        let sum_count = self.list_bq_count + self.elem_bq_count;

        if self.comma_count > sum_count {
            let err = SimpleError::new("Comma not inside a backquote.");

            return Err(RLError::SimpleError(err));
        }

        if self.is_comma_at {
            let err = SimpleError::new(
                &format!("{} is not a well-formed backquote expression.",
                    self.str_expr));

            return Err(RLError::SimpleError(err));
        }

        println!("self.comma_expr: {:?}", self.comma_expr);
        println!("");

        if self.elem_bq_count == 1 &&
           self.str_expr.eq("`") &&
           self.is_list {}


        if self.elem_bq_count == 1 &&
           self.str_expr.eq("`") &&
           self.is_list.not() {}

        Ok(())
    }
    */
}
