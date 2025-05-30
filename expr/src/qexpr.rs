// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;
// use std::ops::Not;

use crate::atom::RLAtom;
use crate::cons::RLCons;
use crate::QuoteTrait;
use crate::expr::{sexpr_to_qexpr};
use crate::list::RLList;
use crate::sexpr::SExpr;
use crate::symb::RLSymbol;
use crate::nil::RLNil;

use err::err::{RLError, TypeError};

#[derive(Debug, Clone)]
pub enum QExpr {
    Atom(RLAtom),

    Nil(RLNil),

    Symb(RLSymbol),

    QList2(RLList<QExpr>),
    QCons(RLCons<QExpr>),
}

impl QExpr {
    //// QList
    pub fn get_rllist(&self) -> RLList<QExpr> {
        match self {
            QExpr::QList2(rllist) => {
                return rllist.clone();
            }

            _ => unreachable!(),
        }
    }

    pub fn car(&mut self) -> Result<QExpr, RLError> {
        match self {
            QExpr::Nil(nil) => {
                Ok(QExpr::Nil(nil.clone()))
            }

            QExpr::Atom(atom) => {
                let atom_string = atom.get_atom_string();

                let err = TypeError::new(&atom_string, "LIST");

                return Err(RLError::TypeError(err));
            }

            QExpr::Symb(symb) => {
                let symb_name = symb.get_symbol_name();

                let err = TypeError::new(&symb_name, "LIST");

                return Err(RLError::TypeError(err));
            }

            QExpr::QList2(list) => {
                match list.car() {
                    Ok(res) => Ok(res),

                    Err(err) => Err(err),
                }
            }

            QExpr::QCons(cons) => {
                Ok(cons.car())
            }
        }
    }

    pub fn cdr(&mut self) -> Result<QExpr, RLError> {
        match self {
            QExpr::Nil(nil) => {
                Ok(QExpr::Nil(nil.clone()))
            }

            QExpr::Atom(atom) => {
                let atom_string = atom.get_atom_string();

                let err = TypeError::new(&atom_string, "LIST");

                return Err(RLError::TypeError(err));
            }

            QExpr::Symb(symb) => {
                let symb_name = symb.get_symbol_name();

                let err = TypeError::new(&symb_name, "LIST");

                return Err(RLError::TypeError(err));
            }

            QExpr::QList2(list) => {
                match list.cdr() {
                    Ok(res) => Ok(res),

                    Err(err) => Err(err)
                }
            }

            QExpr::QCons(cons) => {
                Ok(cons.cdr())
            }
        }
    }

    /*
    pub fn car(&self) -> Result<QExpr, RLError> {
        let nil_string: [String; 8] = ["nil".to_string(),
                                       "NIL".to_string(),
                                       "Nil".to_string(),
                                       "NIl".to_string(),
                                       "NiL".to_string(),
                                       "niL".to_string(),
                                       "nIL".to_string(),
                                       "nIl".to_string(),];

        let mut result_car = QExpr::Nil(RLNil::new());

        // is car a list ?
        let mut found_list = false;

        let mut paren_count = 0;

        // if car is a list, we store its members in this list
        let mut result_list = LinkedList::<QExpr>::new();

        match self {
            QExpr::Nil(nil) => {
                result_car = QExpr::Nil(nil.clone());
            }

            QExpr::QList(qlist) => {

                // pop "(" and ")" at begin and end of qlist
                qlist.pop_front();
                qlist.pop_back();

                /*
                println!("qlist: {:?}", qlist);
                println!("");

                println!("qlist len(): {:?}", qlist.len());
                println!("");
                */

                // pop the first elem of q_list
                let first_of_qlist = qlist.pop_front();

                if let Some(QExpr::Atom(ref atom)) =
                    first_of_qlist {

                    let atom_string = atom.get_atom_string();

                    for i in 0..nil_string.len() {
                        match nil_string.get(i) {
                            Some(nil_str) => {
                                if *nil_str == *atom_string {
                                    result_car = QExpr::Nil(RLNil::new());
                                    break;
                                } else {
                                    let atom_upp = atom_string.to_uppercase();

                                    result_car =
                                        QExpr::Atom(RLAtom::new(&atom_upp));
                                }
                            }
                            None => unreachable!(),
                        }
                    }
                }

                /*
                if let Some (QExpr::Nil) = first_of_qlist {
                    result_car = QExpr::Nil;
                }

                if let Some(QExpr::QList(ref list)) = first_of_qlist {
                    result_car = QExpr::QList(list.clone());
                }
                */

                if let Some(QExpr::Symb(ref symb)) = first_of_qlist {
                    let symb_name = symb.get_symbol_name();

                    match &*symb_name {
                        "(" => {
                            found_list = true;

                            paren_count = paren_count + 1;

                            result_list.push_back(
                                QExpr::Symb(
                                    RLSymbol::new_with_str("(")));
                        }

                        ")" => {
                            // throw an error
                        }

                        &_  => {
                            result_car = QExpr::Symb(symb.clone());
                        }
                    }
                }

                if found_list {

                    loop {

                        let elem = qlist.pop_front();

                        if let Some(QExpr::Atom(ref atom)) = elem {

                            let mut found_nil = false;

                            let atom_string = atom.get_atom_string();

                            for i in 0..nil_string.len() {
                                match nil_string.get(i) {
                                    Some(nil_str) => {
                                        if *nil_str == *atom_string {
                                            result_list.push_back(
                                                QExpr::Nil(RLNil::new()));

                                            found_nil = true;
                                            break;
                                         }
                                    }
                                    None => unreachable!(),
                                }
                            } // for

                            if found_nil.not() {
                                let atom_upp = atom_string.to_uppercase();

                                result_list.push_back(
                                    QExpr::Atom(RLAtom::new(&atom_upp)));
                            }
                        }

                        if let Some(QExpr::QList(ref list)) = elem {
                            result_list.push_back(
                                QExpr::QList(list.clone()));
                        }

                        if let Some(QExpr::Symb(ref symb)) = elem {
                            let symb_name = symb.get_symbol_name();

                            match &*symb_name {
                                "(" => {
                                    paren_count = paren_count + 1;

                                    result_list.push_back(
                                        QExpr::Symb(
                                            RLSymbol::new_with_str("(")));
                                }

                                ")" => {
                                    paren_count = paren_count - 1;

                                    if paren_count == 0 {
                                        println!("paren_count == 0");
                                        println!("");

                                        result_list.push_back(
                                            QExpr::Symb(
                                                RLSymbol::new_with_str(")")));

                                        break;

                                    } else {

                                        result_list.push_back(
                                            QExpr::Symb(
                                                RLSymbol::new_with_str(")")));
                                    }
                                }

                                _ => {
                                    result_list.push_back(QExpr::Symb(
                                        symb.clone()));
                                }
                            }
                        }

                        if let None = elem {
                            /*
                            if paren_count == 0 {
                                break;
                            } else {
                                // throw an error
                            }
                            */

                            break;
                        }
                    } // loop
                } // if
            }

            QExpr::Cons(first, _second) => {
                result_car = *first.clone();
            }

            QExpr::Atom(atom) => {
                let atom_string = atom.get_atom_string();

                let err = TypeError::new(&atom_string, "LIST");

                return Err(RLError::TypeError(err));
            }

            QExpr::Symb(symb) => {
                let symb_name = symb.get_symbol_name();

                let err = TypeError::new(&symb_name, "LIST");

                return Err(RLError::TypeError(err));
            }

            _ => unreachable!(),
        }

        if result_list.len() > 0 {
            result_car = QExpr::QList(result_list.clone());
            return Ok(result_car);
        } else {
            return Ok(result_car);
        }
    }
    */

    /*
    pub fn cdr(&mut self) -> Result<QExpr, RLError> {

        // result of cdr is a list, we store its members in this list
        // let mut result_list = LinkedList::<QExpr>::new();

        let result_cdr: QExpr;

        // is car a list ?
        let mut found_list = false;

        let mut paren_count = 0;

        match self {
            QExpr::Nil(nil) => {
                result_cdr = QExpr::Nil(nil.clone());
            }

            QExpr::QList(qlist) => {

                // pop "(" and ")" at begin and end of qlist
                qlist.pop_front();
                qlist.pop_back();

                /*
                println!("qlist: {:?}", qlist);
                println!("");

                println!("qlist len(): {:?}", qlist.len());
                println!("");
                */

                // pop the first elem of q_list
                let first_of_qlist = qlist.pop_front();

                if let Some(QExpr::Symb(ref symb)) = first_of_qlist {
                    let symb_name = symb.get_symbol_name();

                    match &*symb_name {
                        "(" => {
                            found_list = true;

                            paren_count = paren_count + 1;
                        }

                        ")" => {
                            // throw an error
                        }

                        &_  => { }
                    }
                }

                if found_list {

                    loop {

                        let elem = qlist.pop_front();

                        if let Some(QExpr::Symb(ref symb)) = elem {
                            let symb_name = symb.get_symbol_name();

                            match &*symb_name {
                                "(" => {
                                    paren_count = paren_count + 1;
                                }

                                ")" => {
                                    paren_count = paren_count - 1;

                                    if paren_count == 0 {
                                        println!("paren_count == 0");
                                        println!("");

                                        break;
                                    }
                                }

                                _ => {}
                            }
                        }

                        if let None = elem {
                            break;
                        }
                    } // loop
                } // if


                qlist.push_front(QExpr::Symb(RLSymbol::new_with_str("(")));

                qlist.push_back(QExpr::Symb(RLSymbol::new_with_str(")")));

                result_cdr = QExpr::QList(qlist.clone());
            }

            QExpr::Cons(_first, second) => {
                result_cdr = *second.clone();
            }

            QExpr::Atom(atom) => {
                let atom_string = atom.get_atom_string();

                let err = TypeError::new(&atom_string, "LIST");

                return Err(RLError::TypeError(err));
            }

            QExpr::Symb(symb) => {
                let symb_name = symb.get_symbol_name();

                let err = TypeError::new(&symb_name, "LIST");

                return Err(RLError::TypeError(err));
            }

            _ => unreachable!(),
        }

        Ok(result_cdr)
    }
    */

    //// Cons
    pub fn new_cons(pair: (QExpr, QExpr)) -> QExpr {
        match pair {
            (QExpr::Nil(_), QExpr::Nil(_)) => {
                let mut rllist = RLList::<QExpr>::new();

                rllist.push_front(QExpr::Nil(RLNil::new()));

                return QExpr::QList2(rllist);
            }

            (QExpr::Nil(nil), QExpr::Atom(atom)) => {
                let rlcons = RLCons::<QExpr>::new_cons(QExpr::Nil(nil),
                                                       QExpr::Atom(atom));

                return QExpr::QCons(rlcons);
            }

            (QExpr::Atom(atom), QExpr::Nil(_)) => {
                let mut rllist = RLList::<QExpr>::new();

                rllist.push_front(QExpr::Atom(atom));

                return QExpr::QList2(rllist);
            }

            (QExpr::Atom(atom1), QExpr::Atom(atom2)) => {
                let rlcons = RLCons::<QExpr>::new_cons(QExpr::Atom(atom1),
                                                       QExpr::Atom(atom2));

                return QExpr::QCons(rlcons);
            }

            (QExpr::Nil(nil), QExpr::QList2(mut list)) => {
                let mut linked_list = list.get_linked_list();

                linked_list.pop_front();
                linked_list.pop_back();

                linked_list.push_front(QExpr::Nil(nil));

                list.set_linked_list(&linked_list);

                return QExpr::QList2(list);
            }

            (QExpr::QList2(list), QExpr::Nil(_)) => {
                let mut rllist = RLList::<QExpr>::new();

                rllist.push_front(QExpr::QList2(list));

                return QExpr::QList2(rllist);
            }

            (QExpr::Atom(atom), QExpr::QList2(mut list)) => {
                let mut linked_list = list.get_linked_list();

                linked_list.pop_front();
                linked_list.pop_back();

                linked_list.push_front(QExpr::Atom(atom));

                list.set_linked_list(&linked_list);

                return QExpr::QList2(list);
            }

            (QExpr::QList2(list), QExpr::Atom(atom)) => {
                let rlcons = RLCons::<QExpr>::new_cons(QExpr::QList2(list),
                                                       QExpr::Atom(atom));

                return QExpr::QCons(rlcons);
            }

            (QExpr::QList2(list1), QExpr::QList2(mut list2)) => {
                let mut linked_list = list2.get_linked_list();

                linked_list.pop_front();
                linked_list.pop_back();

                linked_list.push_front(QExpr::QList2(list1));

                list2.set_linked_list(&linked_list);

                return QExpr::QList2(list2);
            }

            _ => todo!(),
        }
    }
}

impl QuoteTrait for QExpr {

    fn quote(sexpr: SExpr) -> Result<QExpr, RLError> {
        println!("in QuoteTrait qexpr - quote");

        let result_qexpr: QExpr;

        match sexpr {
            SExpr::SList(list) => {
                result_qexpr = sexpr_to_qexpr(SExpr::SList(list));
            }

            SExpr::QList(list) => {
                result_qexpr = sexpr_to_qexpr(SExpr::QList(list));
            }

            _ => unreachable!()
        }
        Ok(result_qexpr)
    }
}

#[allow(unused_assignments)]
impl fmt::Display for QExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // println!("in fmt for QExpr");
        // println!("self: {:?}", self);
        match self {
            QExpr::Atom(s) => { // println!("atom");
                                write!(f, "{}", s) }

            QExpr::Symb(s)  => write!(f, "{}", s),

            QExpr::Nil(nil) => write!(f, "{}", nil),

            QExpr::QList2(qlist) => {
                let list = qlist.get_linked_list();

                let mut iter = list.iter().peekable();

                for qexpr in list.clone() {
                    iter.next();

                    let _ = match qexpr {
                        QExpr::Symb(s) => {
                            let s_name = s.get_symbol_name();

                            match &*s_name {
                                "(" => {
                                    write!(f, "{}", s_name)
                                }

                                ")" => {
                                    write!(f, "{}", s_name)
                                }

                                "`" => {
                                    write!(f, "{}", s_name)
                                }

                                "," => {
                                    write!(f, "{}", s_name)
                                }

                                _ => { write!(f, "{} ", s_name.to_uppercase())
                                }
                            }
                        }

                        QExpr::QList2(q_list) => {
                            if let Some(QExpr::Atom(_atom)) = iter.peek() {
                                write!(f, "{} ", QExpr::QList2(q_list.clone()))
                            } else { write!(f, "{}", QExpr::QList2(q_list.clone())) }
                        }

                        QExpr::Atom(a) => {
                            if let Some(QExpr::Symb(s)) = iter.peek() {
                                let s_name = s.get_symbol_name();

                                match &*s_name {
                                    "(" => { write!(f, "{} ", a)? }

                                    ")" => { write!(f, "{}", a)? }

                                      _ => write!(f, "{} ", a)?,
                                }
                            }

                            if let Some(QExpr::Atom(_)) = iter.peek() {
                                write!(f, "{} ", a)?
                            }

                            if let Some(QExpr::QList2(_list)) = iter.peek() {
                                write!(f, "{} ", a)
                            } else { Ok(()) }
                        }

                        QExpr::Nil(nil) => write!(f, "{}", nil),

                        _ => write!(f, ""),
                    };
                }
                Ok(())
            }

            QExpr::QCons(cons) => {
                let first = cons.car();

                let second = cons.cdr();

                write!(f, "({} . {})", first, second)
            }
        }
    }
}
