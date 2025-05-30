// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::LinkedList;
use std::ops::Not;

use crate::atom::RLAtom;
use crate::comma::RLComma;
use crate::expr::sexpr_to_qexpr;
use crate::nil::RLNil;
use crate::qexpr::QExpr;
use crate::sexpr::SExpr;
use crate::symb::RLSymbol;

use err::err::RLError;

use pars_symb::symbol::Symbol;
use pars_symb::token::Token;

#[derive(Debug, Clone)]
pub struct RLList<T> {
    list: LinkedList<T>,

    comma: Option<RLComma>,
}

impl RLList<SExpr> {
    pub fn new() -> RLList<SExpr> {
        let mut list = LinkedList::<SExpr>::new();

        list.push_front(SExpr::Symb(RLSymbol::new_with_str(")")));
        list.push_front(SExpr::Symb(RLSymbol::new_with_str("(")));

        let comma = None;

        RLList {
            list,
            comma,
        }
    }

    pub fn new_with_list(ll: &LinkedList<SExpr>) -> RLList<SExpr> {
        let list = ll.clone();

        let comma = None;

        RLList {
            list,
            comma,
        }
    }

    pub fn set_comma(&mut self, comma: &RLComma) {
        self.comma = Some(comma.clone());
    }

    pub fn get_comma(&self) -> Option<RLComma> {
        self.comma.clone()
    }

    pub fn has_comma(&self) -> bool {
        if let Some(_comma) = &self.comma {
            return true
        } else {
            return false;
        }
    }

    pub fn set_linked_list(&mut self, linked_list: &LinkedList<SExpr>) {
        self.list = linked_list.clone();
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn get_linked_list(&self) -> LinkedList<SExpr> {
        self.list.clone()
    }

    pub fn pop_back(&mut self) -> Option<SExpr> {
        self.list.pop_back()
    }

    pub fn pop_front(&mut self) -> Option<SExpr> {
        self.list.pop_front()
    }

    pub fn push_back(&mut self, sexpr: SExpr) {
        self.list.push_back(sexpr);
    }

    pub fn push_front(&mut self, sexpr: SExpr) {
        self.list.push_front(sexpr);
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn get_rest_lambda_list(&mut self) -> SExpr {
        let rest_keyword = SExpr::Atom(RLAtom::new("&rest"));

        self.list.push_front(rest_keyword);

        SExpr::SList(RLList::<SExpr>::new_with_list(&self.list.clone()))
    }

    pub fn slist_to_cons(&mut self) -> SExpr {
        let mut symb = Symbol::new("dummy");

        let mut linked_list = self.get_linked_list();

        let mut ll = LinkedList::<SExpr>::new();

        println!("slist_to_cons - linked_list: {:?}", linked_list);
        println!("");

        // remove parenthesis
        linked_list.pop_front();
        linked_list.pop_back();

        // parse cons symbol
        if let Some(SExpr::Symb(symbol)) = linked_list.pop_front() {
            symb = Symbol { name: symbol.to_string() };
        }

        // build ll
        for _n in 0..linked_list.len() {
            if let Some(sexpr) = linked_list.pop_front() {
                ll.push_back(sexpr);
            }
        }

        SExpr::Cons(symb, ll)
    }

    pub fn slist_to_sform(&mut self) -> SExpr {
        // let mut symb = Symbol::new("dummy");

        let mut vec = Vec::<Token>::new();

        let mut linked_list = self.get_linked_list();

        println!("slist_to_form - linked_list: {:?}", linked_list);
        println!("");

        // remove parenthesis
        // self.list.pop_front();
        // self.list.pop_back();

        // parse cons symbol
        // if let Some(SExpr::Symb(symbol)) = self.list.pop_front() {
        //     symb = Symbol { name: symbol.to_string() };
        // }

        // build vec
        for _n in 0..linked_list.len() {
            let item = linked_list.pop_back().unwrap();

            match item {
                SExpr::Atom(atom) => {
                    let atom_string = atom.get_atom_string();

                    vec.push(Token::Atom(atom_string));
                }

                SExpr::Symb(symb) => {
                    let symb_string = symb.get_symbol_name();

                    vec.push(Token::Symb(Symbol { name: symb_string }));
                }

                SExpr::Nil(_nil) => {
                    vec.push(Token::Atom("nil".to_string()));
                }

                SExpr::SList(slist) => {
                    let ll = slist.get_linked_list();

                    vec = self.linked_list_to_vec_token(&vec, ll.clone());
                }

                _ => todo!(),
            }
        }

        SExpr::SForm(vec)
    }

    pub fn linked_list_to_vec_token(&self,
                                    vec: &Vec<Token>,
                                    mut ll: LinkedList<SExpr>) -> Vec<Token> {
        let mut vec = vec.clone();

        for _n in 0..ll.len() {
            let item = ll.pop_front().unwrap();

            match item {
                SExpr::Atom(atom) => {
                    let atom_string = atom.get_atom_string();

                    vec.push(Token::Atom(atom_string));
                }

                SExpr::Symb(symb) => {
                    let symb_string = symb.get_symbol_name();

                    vec.push(Token::Symb(Symbol { name: symb_string }));
                }

                SExpr::Nil(_nil) => {
                    vec.push(Token::Atom("nil".to_string()));
                }

                SExpr::SList(slist) => {
                    let ll = slist.get_linked_list();

                    vec = self.linked_list_to_vec_token(&vec, ll.clone());
                }

                _ => todo!(),
            }
        }

        vec.clone()
    }
}

impl RLList<QExpr> {
    pub fn new() -> RLList<QExpr> {
        let mut list = LinkedList::<QExpr>::new();

        list.push_front(QExpr::Symb(RLSymbol::new_with_str(")")));
        list.push_front(QExpr::Symb(RLSymbol::new_with_str("(")));

        let comma = None;

        RLList {
            list,
            comma,
        }
    }

    pub fn new_qlist(ll: &LinkedList<SExpr>) -> RLList<QExpr> {
        let mut list = LinkedList::<QExpr>::new();

        list.push_front(QExpr::Symb(RLSymbol::new_with_str(")")));

        let mut linked_list = ll.clone();

        while linked_list.is_empty().not() {
            if let Some(s_elem) = linked_list.pop_back() {
                list.push_front(sexpr_to_qexpr(s_elem));
            }
        }

        list.push_front(QExpr::Symb(RLSymbol::new_with_str("(")));

        let comma = None;

        RLList {
            list,
            comma,
        }
    }

    pub fn new_with_list(ll: &LinkedList<QExpr>) -> RLList<QExpr> {
        let list = ll.clone();

        let comma = None;

        RLList {
            list,
            comma,
        }
    }

    pub fn set_comma(&mut self, comma: &RLComma) {
        self.comma = Some(comma.clone());
    }

    pub fn get_comma(&self) -> Option<RLComma> {
        self.comma.clone()
    }

    pub fn has_comma(&self) -> bool {
        if let Some(_comma) = &self.comma {
            return true
        } else {
            return false;
        }
    }

    pub fn set_linked_list(&mut self, linked_list: &LinkedList<QExpr>) {
        self.list = linked_list.clone();

        self.list.push_back(QExpr::Symb(RLSymbol::new_with_str(")")));
        self.list.push_front(QExpr::Symb(RLSymbol::new_with_str("(")));
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn get_linked_list(&self) -> LinkedList<QExpr> {
        self.list.clone()
    }

    pub fn pop_back(&mut self) -> Option<QExpr> {
        self.list.pop_back();

        let option_item = self.list.pop_back();

        self.push_back(QExpr::Symb(RLSymbol::new_with_str(")")));

        option_item
    }

    pub fn pop_front(&mut self) -> Option<QExpr> {
        self.list.pop_front();

        let option_item = self.list.pop_front();

        self.push_front(QExpr::Symb(RLSymbol::new_with_str("(")));

        option_item
    }

    pub fn push_back(&mut self, qexpr: QExpr) {
        self.list.pop_back();

        self.list.push_back(qexpr);

        self.list.push_back(QExpr::Symb(RLSymbol::new_with_str(")")));
    }

    pub fn push_front(&mut self, qexpr: QExpr) {
        self.list.pop_front();

        self.list.push_front(qexpr);

        self.list.push_front(QExpr::Symb(RLSymbol::new_with_str("(")));
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn car(&mut self) -> Result<QExpr, RLError> {
        let mut linked_list = self.get_linked_list();

        if linked_list.len() == 2 {
            return Ok(QExpr::Nil(RLNil::new()));
        } else if linked_list.len() >= 3 {
            // remove "("
            linked_list.pop_front();

            if let Some(qexpr) = linked_list.pop_front() {
                return Ok(qexpr);
            } else {
                return Err(RLError::SimpleProgramError);
            }
        } else {
            return Err(RLError::SimpleProgramError);
        }
    }

    pub fn cdr(&mut self) -> Result<QExpr, RLError> {
        let mut linked_list = self.get_linked_list();

        println!("linked_list: {:?}", linked_list);
        println!("");

        if linked_list.len() == 2 {
            return Ok(QExpr::Nil(RLNil::new()));
        } else if linked_list.len() >= 3 {
            // remove "(" and ")"
            linked_list.pop_front();
            linked_list.pop_back();

            // remove first element
            linked_list.pop_front();

            if linked_list.len() == 0 {
                return Ok(QExpr::Nil(RLNil::new()));
            } else {
                self.set_linked_list(&linked_list);

                return Ok(QExpr::QList2(self.clone()));
            }
        } else {
            return Err(RLError::SimpleProgramError);
        }
    }

    pub fn concatenate_lists(&mut self, second_list: &QExpr) -> RLList<QExpr> {

        let mut first_ll = self.get_linked_list();

        first_ll.pop_front();
        first_ll.pop_back();

        let second_rllist = second_list.get_rllist();

        let mut second_ll = second_rllist.get_linked_list();

        second_ll.pop_front();
        second_ll.pop_back();

        // let mut result_list = Self::create_qlist();

        let mut result_list = LinkedList::<QExpr>::new();

        result_list.push_front(QExpr::Symb(RLSymbol::new_with_str(")")));

        while second_ll.is_empty().not() {
            if let Some(elem) = second_ll.pop_back() {

                // println!("elem: {:?}", elem);
                // println!("");

                result_list.push_front(elem);
            }
        }

        while first_ll.is_empty().not() {
            if let Some(elem) = first_ll.pop_back() {

                println!("elem: {:?}", elem);
                println!("");

                result_list.push_front(elem);
            }
        }

        result_list.push_front(QExpr::Symb(RLSymbol::new_with_str("(")));

        self.set_linked_list(&result_list);

        Self { list: self.list.clone(), comma: self.comma.clone() }
    }
}
