// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::cell::RefMut;
use std::collections::{BTreeMap, HashMap, LinkedList};
use std::ops::Not;

use env::block::RLBlock;
use env::env_trait::EnvTrait;
use env::var::RLVar;

use err::err::{RLError,
               SimpleError,
               UnboundVariableError};

use expr::atom::RLAtom;
use expr::comma::RLComma;
use expr::list::RLList;
use expr::nil::RLNil;
use expr::sexpr::SExpr;

use hash::hash::RLHash;

use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         0123456789";

const ID_LEN: usize = 8;

#[derive(Debug)]
pub struct RLNamedDsBind {
    expr_ll: LinkedList<SExpr>,

    sexpr_ll: LinkedList<SExpr>,

    sexpr_with_scons: SExpr,

    scons: BTreeMap<String, SExpr>,

    // the two parameters from the Lisp spec
    // expr:
    // env:

    block: RLBlock,
}

impl RLNamedDsBind {

    pub fn new() -> RLNamedDsBind {

        let expr_ll = LinkedList::<SExpr>::new();

        let sexpr_ll = LinkedList::<SExpr>::new();

        let sexpr_with_scons = SExpr::Nil(RLNil::new());

        let scons = BTreeMap::<String, SExpr>::new();

        let block = RLBlock::new("Dummy".to_string());

        Self {
            expr_ll,
            sexpr_ll,
            sexpr_with_scons,
            scons,
            block,
        }
    }

    pub fn set_block (&mut self, block: &RLBlock) {
        self.block = block.clone();
    }

    pub fn set_expr_ll(&mut self, ll: &LinkedList<SExpr>) {
        self.expr_ll = ll.clone()
    }

    pub fn contains_sexpr_slist(&self, ll: &mut LinkedList<SExpr>) -> bool {
        loop {
            match ll.pop_front() {
                Some(SExpr::SList(_slist)) => {
                    return true;
                }

                None => {
                    return false;
                }

                _ => {}
            }
        }
    }

    pub fn bq_comma(&self, ll: &mut LinkedList<SExpr>) -> RLComma {
        println!("Begin bq_comma");
        println!("");

        let mut result_ll = LinkedList::<SExpr>::new();

        let mut stored_item: SExpr;

        loop {
            let item = ll.pop_front().unwrap();

            stored_item = item.clone();

            println!("item: {:?}", item);
            println!("");

            match item {
                SExpr::Symb(symb) => {
                    let symb_name = symb.get_symbol_name();

                    match &*symb_name {
                        "`" => {
                            result_ll.push_back(SExpr::Symb(symb));
                        }

                        "," => {
                            result_ll.push_back(SExpr::Symb(symb));
                        }

                        "'" => {
                            result_ll.push_back(SExpr::Symb(symb));
                        }

                        ",@" => {
                            result_ll.push_back(SExpr::Symb(symb));
                        }

                        &_ => {
                            println!("stored_item: {:?}", stored_item);
                            println!("");

                            result_ll.push_back(stored_item.clone());

                            ll.push_front(stored_item);
                            break;
                        }
                    }
                }

                _ => {
                    println!("stored_item: {:?}", stored_item);
                    println!("");

                    result_ll.push_back(stored_item.clone());

                    ll.push_front(stored_item);
                    break;
                }
            }
        }

        println!("ll: {:?}", ll);
        println!("");

        println!("result_ll: {:?}", result_ll);
        println!("");

        let mut bq_comma_expr = RLComma::new();
        bq_comma_expr.init_members(&result_ll);

        println!("bq_comma_expr: {:?}", bq_comma_expr);
        println!("");

        println!("End bq_comma");
        println!("");

        bq_comma_expr
    }





    // `(+ `,(+ 1 2) 3)
    // `(+ ,(+ 1 `,(+ 2 3) 4) 5)
    // `(+ ,(+ 1 `,(+ `,x `,y) `,z) 3)
    // `(+ 1 ,a)

    pub fn create_comma_exprs(&mut self,
                              ll: &LinkedList<SExpr>,
                              mut dangling_bq: i64) ->
        LinkedList<SExpr> {

        println!("Begin - create_comma_exprs");
        println!("");

        println!("dangling_bq: {:?}", dangling_bq);
        println!("");

        let mut linked_list = ll.clone();

        println!("linked_list: {:?}", linked_list);
        println!("");

        let mut result_ll = LinkedList::<SExpr>::new();

        // let stored_ll = linked_list.clone();

        // let comma_expr_ll = LinkedList::<SExpr>::new();
        let mut slist_ll = LinkedList::<SExpr>::new();

        let mut work_on_slist = false;

        let mut comma_expr = RLComma::new();

        let mut list_comma_expr = RLComma::new();

        let mut comma_expr_created = false;
        let mut list_comma_expr_created = false;

        let mut paren_count = 0;

        loop {
            match linked_list.pop_front() {
                Some(SExpr::Symb(mut symb)) => {
                    let symb_name = symb.get_symbol_name();

                    match &*symb_name {
                        "`" => {
                            if work_on_slist {
                                slist_ll.push_back(SExpr::Symb(symb));
                            } else {
                                linked_list.push_front(SExpr::Symb(symb));

                                comma_expr =
                                    self.bq_comma(&mut linked_list);

                                comma_expr_created = true;
                            }
                        }

                        "," => {
                            if work_on_slist {
                                slist_ll.push_back(SExpr::Symb(symb));
                            } else {
                                linked_list.push_front(SExpr::Symb(symb));

                                comma_expr = self.bq_comma(&mut linked_list);
                                comma_expr_created = true;
                            }
                        }

                        ",@" => {
                            if work_on_slist {
                                slist_ll.push_back(SExpr::Symb(symb));
                            } else {
                                linked_list.push_front(SExpr::Symb(symb));

                                comma_expr =
                                    self.bq_comma(&mut linked_list);

                                comma_expr_created = true;
                            }
                        }

                        "(" => {
                            if work_on_slist &&
                               comma_expr_created.not() {

                                paren_count = paren_count + 1;
                                slist_ll.push_back(SExpr::Symb(symb));

                            } else if work_on_slist &&
                                      comma_expr_created {

                                // comma_expr_created = false;

                                paren_count = paren_count + 1;
                                slist_ll.push_back(SExpr::Symb(symb))

                            } else if work_on_slist.not() &&
                                      comma_expr_created {

                                work_on_slist = true;

                                comma_expr_created = false;

                                list_comma_expr = comma_expr.clone();

                                println!("list_comma_expr: {:?}",
                                     list_comma_expr);
                                println!("");

                                let list_bq_count =
                                    list_comma_expr.get_list_bq_count();

                                let elem_bq_count =
                                    list_comma_expr.get_elem_bq_count();

                                let comma_count =
                                    list_comma_expr.get_comma_count();

                                let bq_comma_diff =
                                    list_bq_count +
                                    elem_bq_count -
                                    comma_count;

                                if bq_comma_diff > 0 {
                                    dangling_bq = dangling_bq +
                                                  bq_comma_diff;
                                }

                                list_comma_expr.set_list_bq_count(
                                    dangling_bq);

                                println!("list_comma_expr -after setting danglin_bq: {:?}", list_comma_expr);
                                println!("");

                                list_comma_expr.set_is_list(true);

                                list_comma_expr_created = true;

                                paren_count = paren_count + 1;
                                slist_ll.push_back(SExpr::Symb(symb));

                            } else if work_on_slist.not() &&
                                      comma_expr_created.not() {

                               result_ll.push_back(SExpr::Symb(symb));
                            }
                        }

                        ")" => {
                            if work_on_slist {
                                paren_count = paren_count - 1;
                                slist_ll.push_back(SExpr::Symb(symb));

                                if paren_count == 0 {
                                    work_on_slist = false;

                                    let mut new_rllist =
                                        RLList::<SExpr>::new_with_list(
                                            &slist_ll);

                                    if list_comma_expr_created {
                                        list_comma_expr_created = false;
                                        new_rllist.set_comma(&list_comma_expr);
                                        println!("list_comma_expr: {:?}",
                                            list_comma_expr);
                                        println!("");
                                    }

                                    let new_ll =
                                        self.create_comma_exprs(&slist_ll,
                                                                dangling_bq);

                                    new_rllist.set_linked_list(&new_ll);


                                    result_ll.push_back(
                                        SExpr::SList(new_rllist));

                                    slist_ll.clear();
                                }
                            } else {
                                result_ll.push_back(SExpr::Symb(symb));
                            }
                        }

                        &_ => {
                            if work_on_slist &&
                               comma_expr_created {

                               symb.set_comma(&comma_expr);

                               comma_expr_created = false;

                               slist_ll.push_back(SExpr::Symb(symb));

                            } else if work_on_slist &&
                                      comma_expr_created.not() {

                                slist_ll.push_back(SExpr::Symb(symb));

                            } else if work_on_slist.not() &&
                                      comma_expr_created {

                                comma_expr.set_list_bq_count(dangling_bq);

                                symb.set_comma(&comma_expr);

                                comma_expr_created = false;

                                result_ll.push_back(SExpr::Symb(symb));
                            } else {

                                result_ll.push_back(SExpr::Symb(symb));
                            }
                        }
                    }
                }

                Some(SExpr::Atom(mut atom)) => {
                    if work_on_slist &&
                       comma_expr_created {

                        atom.set_comma(&comma_expr);

                        comma_expr_created = false;
                        // slist_ll.push_back(SExpr::Atom(atom));
                        slist_ll.push_back(SExpr::Atom(atom));

                    } else if work_on_slist &&
                              comma_expr_created.not() {

                        slist_ll.push_back(SExpr::Atom(atom));
                    } else if work_on_slist.not() &&
                              comma_expr_created {

                        atom.set_comma(&comma_expr);

                        comma_expr_created = false;

                        result_ll.push_back(SExpr::Atom(atom));
                    } else {
                        result_ll.push_back(SExpr::Atom(atom));
                    }
                }

                Some(SExpr::Nil(nil)) => {
                    if work_on_slist &&
                       comma_expr_created {

                        comma_expr_created = false;

                        slist_ll.push_back(SExpr::Nil(nil));

                    } else if work_on_slist &&
                              comma_expr_created.not() {

                        slist_ll.push_back(SExpr::Nil(nil));

                    } else if work_on_slist.not() &&
                              comma_expr_created {

                        comma_expr_created = false;

                        result_ll.push_back(SExpr::Nil(nil));

                    } else {
                        result_ll.push_back(SExpr::Nil(nil));
                    }
                }

                None => {
                    break;
                }

                _ => unreachable!(),
            }
        }

        println!("final result_ll: {:?}", result_ll);
        println!("");

        result_ll
    }

    pub fn build_macro_sexpr(&mut self) {
        println!("Begin build_macro_sexpr");
        println!("");

        let mut linked_list = self.expr_ll.clone();

        let mut result_ll = LinkedList::<SExpr>::new();

        let first_bq_comma = self.bq_comma(&mut linked_list);

        let list_bq_count = first_bq_comma.get_list_bq_count();
        let elem_bq_count = first_bq_comma.get_elem_bq_count();
        let comma_count   = first_bq_comma.get_comma_count();

        let bq_comma_diff = list_bq_count + elem_bq_count - comma_count;

        let mut dangling_bq: i64 = 0;

        if bq_comma_diff > 0 {
            dangling_bq = bq_comma_diff;
        }

        println!("linked_list: {:?}", linked_list);
        println!("");

        let sexpr_item = linked_list.pop_front().expect("REASON");

        match sexpr_item {
            SExpr::Symb(mut symb)   => {
                symb.set_comma(&first_bq_comma);

                result_ll.push_back(SExpr::Symb(symb));
            }

            SExpr::Atom(mut atom)   => {
                atom.set_comma(&first_bq_comma);

                result_ll.push_back(SExpr::Atom(atom));
            }

            SExpr::SList(mut slist) => {
                slist.set_comma(&first_bq_comma);

                let mut slist_ll = slist.get_linked_list();

                slist_ll = self.create_comma_exprs(&slist_ll,
                                                   dangling_bq);

                slist.set_linked_list(&slist_ll);

                result_ll.push_back(SExpr::SList(slist));
            }

            SExpr::Nil(mut nil)     => {
                nil.set_comma(&first_bq_comma);

                result_ll.push_back(SExpr::Nil(nil));
            }

            _ => unreachable!(),
        }

        println!("result_ll: {:?}", result_ll);
        println!("");

        self.sexpr_ll = result_ll.clone();

        println!("End build_macro_sexpr");
        println!("");
    }





    pub fn get_ref_dyn_env_block(&mut self) ->
        &HashMap<String, RLVar, RLHash> {

        self.block.get_ref_dyn_env_block()
    }

    pub fn get_dyn_env_var(&mut self, key: &str) -> Option<RLVar> {
        if let Some(item) = self.block.get_dyn_env_var(&key.to_string()
                                                           .to_uppercase()) {
            return Some(item);
        } else {
            return None;
        }
    }

    pub fn add_dyn_env_var(&mut self, name: String, var: RLVar) {
        self.block.add_dyn_env_var(name.to_uppercase(), var.clone());
    }

    pub fn get_replace_expr(&mut self,
                            comma_var: String,
                            env_ref: &mut RefMut<'_, dyn EnvTrait>) ->
        Result<SExpr, RLError> {

        let mut result_expr = SExpr::Nil(RLNil::new());

        /*
        if let Some(mut var) = env_ref.get_curr_eval_dyn_env_var(&comma_var) {

            result_expr = var.var_to_sexpr();
        } else {
        */

        let comma_var_upp = &comma_var.clone().to_uppercase();

        if let Some(mut var) = self.get_dyn_env_var(&comma_var_upp) {
            result_expr = var.var_to_sexpr();
        } else {
            println!("symbol_current_package: {:?}",
                env_ref.symbol_in_current_package(&comma_var_upp));
            println!("");

            if let Some(symb) = env_ref.get_symbol(&comma_var_upp) {

                println!("symb:  {:?}", symb);
                println!("");

                if let Some(mut var) = symb.get_dyn_var() {
                    println!("var: {:?}", var);
                    println!("");

                    result_expr = var.var_to_sexpr();
                }
            } else {
                let err = UnboundVariableError::new(&comma_var.to_uppercase());
                return Err(RLError::UnboundVariableError(err));
            }
        }
        Ok(result_expr)
    }

    fn replace_scon(&mut self, ll: &LinkedList<SExpr>,
                    key: String, sexpr: SExpr) -> LinkedList<SExpr> {

        println!("Begin - replace_scon");
        println!("");

        let mut linked_list = ll.clone();

        println!("linked_list: {:?}", linked_list);
        println!("");

        let mut result_ll = LinkedList::<SExpr>::new();

        loop {
            match linked_list.pop_front() {
                Some(SExpr::Atom(atom)) => {
                    let atom_string = atom.get_atom_string();

                    if atom_string.eq(&key) {
                        result_ll.push_back(sexpr.clone());
                    } else {
                        result_ll.push_back(SExpr::Atom(atom));
                    }
                }

                Some(SExpr::SList(slist)) => {
                    let list = slist.get_linked_list();

                    result_ll.push_back(
                        SExpr::SList(
                            RLList::<SExpr>::new_with_list(
                                &self.replace_scon(&list,
                                                  key.clone(),
                                                  sexpr.clone()))));
                }

                Some(t) => {
                    result_ll.push_back(t);
                }

                None => {
                    break;
                }
            }
        }

        println!("result_ll: {:?}", result_ll);
        println!("");

        // self.sexpr_with_scons = result_ll;

        println!("End - replace_scon");
        println!("");

        result_ll
    }

    pub fn replace_scons(&mut self, mut btm: BTreeMap<String, SExpr>) -> SExpr {
        println!("Begin - replace_scons");
        println!("");

        println!("self.sexpr_with_scons: {:?}", self.sexpr_with_scons);
        println!("");

        let sexpr = self.sexpr_with_scons.clone();

        let mut result_ll: LinkedList::<SExpr>;

        match sexpr {
            SExpr::SList(slist) => {
                result_ll = slist.get_linked_list();

                for _n in 0..btm.len() {
                    if let Some((key, sexpr)) = btm.pop_first() {
                        println!("key: {:?}", key);
                        println!("sexpr: {:?}", sexpr);
                        println!("");

                        result_ll =
                            self.replace_scon(&result_ll,
                                              key.clone(),
                                              sexpr.clone());
                    }
                }
            }
            _ => unreachable!(),
        }

        println!("result_ll: {:?}", result_ll);
        println!("");

        println!("End - replace_scons");
        println!("");

        SExpr::SList(RLList::<SExpr>::new_with_list(&result_ll))
    }


    fn eval_list_comma_expr(&mut self,
                            sexpr: SExpr) ->
        Result<LinkedList<SExpr>, RLError> {

        println!("Begin eval_list_comma_expr");
        println!("");

        let mut result_ll = LinkedList::<SExpr>::new();

        let slist_ll: LinkedList<SExpr>;

        let mut rllist: RLList<SExpr>;

        let mut result: Result<LinkedList<SExpr>, RLError> =
            Ok(result_ll.clone());

        let mut comma: RLComma;

        match sexpr {
            SExpr::SList(ref slist) => {

                slist_ll = slist.get_linked_list();

                rllist = slist.clone();

                println!("slist_ll: {:?}", slist_ll);
                println!("");

                if let Some(slist_comma) = slist.get_comma() {
                    println!("slist_comma: {:?}", slist_comma);
                    println!("");

                    comma = slist_comma;
                } else {
                    return Ok(slist_ll);
                }

                 println!("comma: {:?}", comma);
                 println!("");
            }
            _ => unreachable!(),
        }

        let list_bq_count = comma.get_list_bq_count();
        let elem_bq_count = comma.get_elem_bq_count();
        let comma_count = comma.get_comma_count();

        // let is_comma_at = comma.get_is_comma_at();
        let mut eval_expr = comma.get_eval_expr();
        // let is_list = comma.get_is_list();

        let mut comma_expr = comma.get_comma_expr();

        println!("comma_expr: {:?}", comma_expr);
        println!("");

        let str_expr = comma.get_str_expr();

        // define certain eval conditions
        let bq_comma_diff = list_bq_count + elem_bq_count - comma_count;

        if list_bq_count + elem_bq_count == comma_count &&
           bq_comma_diff == 0 {
            comma.set_eval_expr(true);
            eval_expr = true;
        }

        if comma_count > list_bq_count + elem_bq_count &&
           bq_comma_diff < 0 {
            let err = SimpleError::new("Comma not inside a backquote.");

            result = Err(RLError::SimpleError(err));
        }

        if eval_expr &&
           str_expr.eq(",") {

            /*
            for _n in 0..comma_expr.len() {
                result_ll.push_back(comma_expr.pop_front().unwrap())
            }

            for _n in 0..slist_ll.len() {
                if let Some(expr) = slist_ll.pop_front() {
                    result_ll.push_back(expr);
                }
            }

            result = Ok(result_ll.clone());
            */

            // let scon = rllist.slist_to_cons();
            let scon = rllist.slist_to_sform();

            let mut rng = rand::thread_rng();

            let mut scon_id = "scon".to_string();

            scon_id.push('_');

            let id: String = (0..ID_LEN).map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            }).collect();

            scon_id.push_str(id.as_str());

            println!("scon_id: {:?}", scon_id);
            println!("");

            self.scons.insert(scon_id.clone(), scon.clone());

            println!("scons: {:?}", self.scons);
            println!("");

            result_ll.push_back(SExpr::Atom(RLAtom::new(&scon_id)));

            println!("result_ll after slist_to_cons: {:?}", result_ll);
            println!("");

            result = Ok(result_ll.clone());
        }

        if eval_expr.not() {
            if comma_expr.len() == 1 {
                result_ll.push_back(sexpr);

            } else if comma_expr.len() > 1 {

                for _n in 0..comma_expr.len() {
                    result_ll.push_back(comma_expr.pop_front().unwrap())
                }

                result_ll.push_back(sexpr);

                result = Ok(result_ll.clone());
            }
        }

        println!("End eval_list_comma_expr");
        println!("");

        result
    }

    fn eval_member_comma_expr(&mut self,
                              sexpr: SExpr,
                              env_ref: &mut RefMut<'_, dyn EnvTrait>)
        -> Result<LinkedList<SExpr>, RLError> {

        println!("Begin eval_member_comma_expr");
        println!("");

        let name: String;

        let mut comma = RLComma::new();

        let mut result_ll = LinkedList::<SExpr>::new();

        let mut result: Result<LinkedList<SExpr>, RLError> =
            Ok(result_ll.clone());

        match sexpr {
            SExpr::Atom(ref atom) => {
                name = atom.get_atom_string();

                if let Some(atom_comma) = atom.get_comma() {
                    println!("atom_comma: {:?}", atom_comma);
                    println!("");

                    comma = atom_comma;
                }
            }

            SExpr::Symb(ref symb) => {
                name = symb.get_symbol_name();

                if let Some(symb_comma) = symb.get_comma() {
                    comma = symb_comma;
                }
            }

            SExpr::Nil(ref nil)   => {
                name = nil.get_nil_name();

                if let Some(nil_comma) = nil.get_comma() {
                    comma = nil_comma;
                }
            }

            _ => unreachable!(),
        }

        println!("eval_member_comma_expr - comma: {:?}", comma);
        println!("");

        let list_bq_count = comma.get_list_bq_count();
        let elem_bq_count = comma.get_elem_bq_count();
        let comma_count = comma.get_comma_count();

        // let is_comma_at = comma.get_is_comma_at();
        let mut eval_expr = comma.get_eval_expr();
        // let is_list = false;

        let mut comma_expr = comma.get_comma_expr();
        let str_expr = comma.get_str_expr();

        // define certain eval conditions
        let bq_comma_diff = list_bq_count + elem_bq_count - comma_count;

        if list_bq_count + elem_bq_count == comma_count &&
           bq_comma_diff == 0 {
            comma.set_eval_expr(true);
            eval_expr = true;
        }

        if comma_count > list_bq_count + elem_bq_count &&
           bq_comma_diff < 0 {
            let err = SimpleError::new("Comma not inside a backquote.");

            result = Err(RLError::SimpleError(err));
        }

        if eval_expr &&
           str_expr.eq(",") {

            if name.eq("NIL") {
                result_ll.push_back(SExpr::Nil(RLNil::new()));
            } else {
                if let Ok(expr) = self.get_replace_expr(name.clone(),
                                                        env_ref) {
                    result_ll.push_back(expr);
                }
            }

            result = Ok(result_ll.clone());
        }

        if eval_expr.not() {
            if comma_expr.len() == 1 {
                result_ll.push_back(sexpr);

            } else if comma_expr.len() > 1 {

                for _n in 0..comma_expr.len() {
                    result_ll.push_back(comma_expr.pop_front().unwrap())
                }

                result_ll.push_back(sexpr);

                result = Ok(result_ll.clone());
            }
        }

        if eval_expr &&
           str_expr.eq(",@") {

            let expr = self.get_replace_expr(name.clone(), env_ref);

            match expr {
                Ok(mut res) => {
                    if let SExpr::SList(ref mut slist) = res {
                        slist.pop_front();
                        slist.pop_back();

                        for _n in 0..slist.len() {
                            if let Some(item) = slist.pop_front() {

                                println!("item: {:?}", item);
                                println!("");

                                result_ll.push_back(item);
                            }
                        }
                    }
                    result = Ok(result_ll);
                }

                Err(err) => {
                    result = Err(err);
                }
            }
        }

        println!("result: {:?}", result);
        println!("");

        println!("End eval_member_comma_expr");
        println!("");

        result
    }

    fn eval_simple_comma_expr(&mut self,
                              sexpr: SExpr,
                              env_ref: &mut RefMut<'_, dyn EnvTrait>) ->
        Result<SExpr, RLError> {

        println!("Begin eval_simple_comma_expr");
        println!("");

        let mut result: Result<SExpr, RLError> =
            Ok(SExpr::Nil(RLNil::new()));

        let name: String;

        let mut comma = RLComma::new();

        let mut sexpr_ll = LinkedList::<SExpr>::new();

        match sexpr {
            SExpr::Atom(ref atom) => {
                name = atom.get_atom_string();

                if let Some(atom_comma) = atom.get_comma() {
                    println!("atom_comma: {:?}", atom_comma);
                    println!("");

                    comma = atom_comma;
                }
            }

            SExpr::Symb(ref symb) => {
                name = symb.get_symbol_name();

                if let Some(symb_comma) = symb.get_comma() {
                    comma = symb_comma;
                }
            }

            SExpr::Nil(ref nil)   => {
                name = nil.get_nil_name();

                if let Some(nil_comma) = nil.get_comma() {
                    comma = nil_comma;
                }
            }

            _ => unreachable!(),
        }

        println!("comma: {:?}", comma);
        println!("");

        // let list_bq_count = comma.get_list_bq_count();
        let elem_bq_count = comma.get_elem_bq_count();
        let comma_count = comma.get_comma_count();

        let is_comma_at = comma.get_is_comma_at();
        let mut eval_expr = comma.get_eval_expr();
        // let is_list = false;

        println!("is_comma_at: {:?}", is_comma_at);
        println!("");

        let mut comma_expr = comma.get_comma_expr();
        let str_expr = comma.get_str_expr();

        // define certain eval conditions
        let bq_comma_diff = elem_bq_count - comma_count;

        if elem_bq_count == comma_count &&
           bq_comma_diff == 0 {
            comma.set_eval_expr(true);
            eval_expr = true;
        }

        if comma_count > elem_bq_count &&
           bq_comma_diff < 0 {
            let err = SimpleError::new("Comma not inside a backquote.");

            result = Err(RLError::SimpleError(err));
        }

        if is_comma_at {
            let err = SimpleError::new(
                &format!("{} is not a well-formed backquote expression.",
                    str_expr));

            result =  Err(RLError::SimpleError(err));
        }

        /*
                X      ,X      ,@X
             ,',X     ,,X     ,@,X
            ,@',X    ,,@X    ,@,@X
        */

        if eval_expr &&
           str_expr.eq("`,") {

            if name.eq("NIL") {
                result = Ok(SExpr::Nil(RLNil::new()));
            } else {
                result = self.get_replace_expr(name.clone(), env_ref);
            }
        }

        if eval_expr.not() {
            if comma_expr.len() == 1 {
                result = Ok(sexpr);

            } else if comma_expr.len() > 1 {
                comma_expr.pop_front();

                for _n in 0..comma_expr.len() {
                    sexpr_ll.push_back(comma_expr.pop_front().unwrap())
                }

                sexpr_ll.push_back(sexpr);

                result = Ok(SExpr::SList(
                             RLList::<SExpr>::new_with_list(&sexpr_ll)));
            }
        }

        println!("result: {:?}", result);
        println!("");

        println!("End eval_simple_comma_expr");
        println!("");

        result
    }





    pub fn replace_comma_exprs(&mut self,
                               ll: &LinkedList<SExpr>,
                               list_bq_count: i64,
                               env_ref: &mut RefMut<'_, dyn EnvTrait>)
        -> LinkedList<SExpr> {

        println!("Begin - replace_list_comma_exprs");
        println!("");

        let mut linked_list = ll.clone();

        println!("linked_list: {:?}", linked_list);
        println!("");

        let mut result_ll = LinkedList::<SExpr>::new();

        loop {
            match linked_list.pop_front() {
                Some(SExpr::Atom(mut atom)) => {
                    if atom.has_comma() {

                        let mut comma = atom.get_comma().unwrap();
                        comma.set_list_bq_count(list_bq_count);
                        atom.set_comma(&comma);

                        if let Ok(mut sexpr_ll) =
                            self.eval_member_comma_expr(SExpr::Atom(atom),
                                                        env_ref) {

                            for _n in 0..sexpr_ll.len() {
                                if let Some(item) = sexpr_ll.pop_front() {
                                    result_ll.push_back(item);
                                }
                            }
                        }
                    } else {
                        result_ll.push_back(SExpr::Atom(atom));
                    }
                }

                Some(SExpr::Symb(mut symb)) => {
                    /*
                    let name = symb.get_symbol_name();
                    if let Some(symb_comma) = symb.get_comma() {
                        println!("symb_comma: {:?}", symb_comma);
                        println!("");
                    }
                    */

                    if symb.has_comma() {
                        let mut comma = symb.get_comma().unwrap();
                        comma.set_list_bq_count(list_bq_count);
                        symb.set_comma(&comma);

                        if let Ok(mut sexpr_ll) =
                            self.eval_member_comma_expr(SExpr::Symb(symb),
                                                        env_ref) {

                            for _n in 0..sexpr_ll.len() {
                                if let Some(item) = sexpr_ll.pop_front() {
                                    result_ll.push_back(item);
                                }
                            }
                        }
                    } else {
                        result_ll.push_back(SExpr::Symb(symb));
                    }
                }

                Some(SExpr::SList(slist)) => {
                    println!("in SExpr::SList");
                    println!("");
                    println!("slist: {:?}", slist);
                    println!("");

                    let slist_ll = slist.get_linked_list();

                    let mut next_list_comma = RLComma::new();


                    if slist.has_comma() {
                        println!("slist comma: {:?}", slist.get_comma());
                        println!("");

                        if let Some(comma) = slist.get_comma() {
                            next_list_comma = comma;
                        }
                    }

                    let list_bq_count_next_list =
                        list_bq_count + next_list_comma.get_list_bq_count();

                    if self.contains_sexpr_slist(&mut slist_ll.clone()) {
                        let mut new_rllist =
                            RLList::<SExpr>::new_with_list(&slist_ll);

                        /*
                        if list_comma_expr_created {
                            list_comma_expr_created = false;
                            new_rllist.set_comma(&list_comma_expr);
                        }
                        */

                        /*
                        if let Some(comma) = slist.get_comma() {
                            new_list.set_comma(&comma);
                        }
                        */

                        let new_ll =
                            self.replace_comma_exprs(&slist_ll,
                                                     list_bq_count_next_list,
                                                     env_ref);

                        println!("new_ll: {:?}", new_ll);
                        println!("");

                        new_rllist.set_linked_list(&new_ll);


                        result_ll.push_back(SExpr::SList(new_rllist));
                    } else {
                        /*
                        result_ll.push_back(SExpr::SList(
                                        RLList::<SExpr>::new_with_list(
                                            &self.replace_comma_exprs(
                                                &slist_ll, env_ref))));
                        */

                        let mut new_list =
                            RLList::<SExpr>::new_with_list(
                                &self.replace_comma_exprs(&slist_ll,
                                                          list_bq_count,
                                                          env_ref));

                        if let Some(comma) = slist.get_comma() {
                            new_list.set_comma(&comma);
                        }

                        println!("new_list: {:?}", new_list);
                        println!("");

                        if let Ok(mut sexpr_ll) =
                            self.eval_list_comma_expr(
                                SExpr::SList(new_list)) {

                            println!("sexpr_ll: {:?}", sexpr_ll);
                            println!("");

                            for _n in 0..sexpr_ll.len() {
                                if let Some(item) = sexpr_ll.pop_front() {
                                    result_ll.push_back(item);
                                }
                            }
                        } else {
                            println!("NOT OK!");
                            println!("");
                        }
                    }
                }

                Some(SExpr::Nil(mut nil))     => {
                    if nil.has_comma() {

                        let mut comma = nil.get_comma().unwrap();
                        comma.set_list_bq_count(list_bq_count);
                        nil.set_comma(&comma);

                        if let Ok(mut sexpr_ll) =
                            self.eval_member_comma_expr(SExpr::Nil(nil),
                                                        env_ref) {

                        for _n in 0..sexpr_ll.len() {
                            if let Some(item) = sexpr_ll.pop_front() {
                                result_ll.push_back(item);
                                }
                            }
                        }
                    } else {
                        result_ll.push_back(SExpr::Nil(nil));
                    }
                }

                None => {
                    break;
                }

                _ => unreachable!(),
            }
        }

        println!("result_ll: {:?}", result_ll);
        println!("");

        println!("End - replace_list_comma_exprs");
        println!("");

        result_ll
    }

    fn start_eval_list(&mut self,
                       sexpr: SExpr,
                       env_ref: &mut RefMut<'_, dyn EnvTrait>) ->
        Result<SExpr, RLError> {

        println!("start_eval_list");
        println!("");

        let mut result: Result<SExpr, RLError> =
            Ok(SExpr::Nil(RLNil::new()));

        let mut comma = RLComma::new();

        match sexpr {
            SExpr::SList(mut slist) => {

                if let Some(slist_comma) = slist.get_comma() {
                    comma = slist_comma;
                }

                let mut slist_ll = slist.get_linked_list();

                println!("slist_ll: {:?}", slist_ll);
                println!("");

                /*
                let list_bq_count = comma.get_list_bq_count();

                println!("start_eval_list - list_bq_count: {:?}",
                    list_bq_count);
                println!("");
                */

                let list_bq_count = comma.get_elem_bq_count();

                /*
                println!("start_eval_list - elem_bq_count: {:?}",
                    elem_bq_count);
                println!("");
                */

                slist_ll = self.replace_comma_exprs(&slist_ll,
                                                    list_bq_count,
                                                    env_ref);

                println!("slist_ll - after replace_list_comma_exprs: {:?}",
                    slist_ll);
                println!("");


                if slist_ll.len() > 1 {
                    slist.set_linked_list(&slist_ll);

                    if // slist_ll.contains_cons() &&
                       self.scons.is_empty().not() {

                       println!("---------------------------------");
                       println!("");

                       println!("self.scons: {:?}", self.scons);
                       println!("");

                       println!("---------------------------------");
                       println!("");

                       self.sexpr_with_scons = SExpr::SList(slist);

                       return Ok(SExpr::SBTreeMap(self.scons.clone()))
                    }

                    result = Ok(SExpr::SList(slist));
                }
            }

            _ => unreachable!(),
        }

        // result = self.eval_list_comma_expr(result, env_ref);

        println!("result: {:?}", result);
        println!("");

        println!("End - start_eval_list");
        println!("");

        result
    }

    pub fn do_macro_replacements(&mut self,
                                 mut env_ref: RefMut<'_, dyn EnvTrait>) ->
        Result<SExpr, RLError> {

        println!("Begin do_macro_replacements");
        println!("");

        let mut result_sexpr = SExpr::Nil(RLNil::new());

        self.scons.clear();

        println!("self.expr_ll: {:?}", self.expr_ll);
        println!("");

        println!("self.sexpr_ll: {:?}", self.sexpr_ll);
        println!("");

        let mut linked_list = self.sexpr_ll.clone();

        match linked_list.pop_front() {
            Some(SExpr::Atom(atom))   => {
                match self.eval_simple_comma_expr(SExpr::Atom(atom),
                                                  &mut env_ref) {

                    Ok(res) => {
                        result_sexpr = res;
                    }

                    Err(err) => {
                        return Err(err);
                    }
                }

                drop(env_ref);
            }

            Some(SExpr::SList(slist)) => {
                match self.start_eval_list(SExpr::SList(slist),
                                           &mut env_ref) {

                    Ok(res) => {
                        result_sexpr = res;
                    }

                    Err(err) => {
                        return Err(err);
                    }
                }

                drop(env_ref);
            }

            Some(SExpr::Symb(symb))   => {
                match self.eval_simple_comma_expr(SExpr::Symb(symb),
                                                  &mut env_ref) {

                    Ok(res) => {
                        result_sexpr = res;
                    }

                    Err(err) => {
                        return Err(err);
                    }
                }

                drop(env_ref);
            }

            Some(SExpr::Nil(nil))     => {
                match self.eval_simple_comma_expr(SExpr::Nil(nil),
                                                  &mut env_ref) {

                    Ok(res) => {
                        result_sexpr = res;
                    }

                    Err(err) => {
                        return Err(err);
                    }
                }

                drop(env_ref);
            }

            None                      => {
                drop(env_ref);
            }

                                    _ => unreachable!(),
        }

        println!("End do_macro_replacements");
        println!("");

        Ok(result_sexpr)
    }
}
