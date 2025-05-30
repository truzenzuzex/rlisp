// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::LinkedList;
use std::ops::Not;

use crate::atom::RLAtom;
use crate::cons::RLCons;
use crate::list::RLList;
use crate::qexpr::QExpr;
use crate::sexpr::SExpr;
use crate::symb::RLSymbol;

// use err::err::RLError;

#[derive(Debug, Clone)]
pub enum Expr {
    QExpr(QExpr),
    SExpr(SExpr),
}

impl Expr { }

/*
fn slist_to_scons_children(mut linked_list: LinkedList<SExpr>,
                           mut result: LinkedList<SExpr>) ->
    LinkedList<SExpr> {

    if let Some(sexpr) = linked_list.pop_front() {
        match sexpr {
            SExpr::Symb(symb) => {
                match &*symb.name {
                    "(" => { slist_to_scons_children(linked_list, result)
                    }

                    ")" => { slist_to_scons_children(linked_list, result)
                    }

                    _ => { slist_to_scons_children(linked_list, result)
                    // slist_to_scons_children(linked_list, result.push_back(SExpr::Cons(symb, slist_to_scons_children(linked_list, result)));
                    }
                }
            }

            SExpr::Atom(atom) => { result.push_back(SExpr::Atom(atom));
                                   slist_to_scons_children(linked_list, result)
            }

            SExpr::Nil => { result.push_back(SExpr::Nil);
                            slist_to_scons_children(linked_list, result)
            }

            _ => unreachable!(),
        }
    } else { return result; }
}

pub fn slist_to_scons(sexpr: SExpr) -> Result<SExpr, RLError> {
    let mut result = SExpr::Nil;

    match sexpr {
        SExpr::SList(mut list) => {
            // get rid of outer parens
            list.pop_front();
            list.pop_back();

            if let Some(SExpr::Symb(symb)) =
                list.pop_front() {

                let cons_ll = slist_to_scons_children(
                    list, LinkedList::<SExpr>::new());

                result = SExpr::Cons(symb, cons_ll);

                println!("{}", result);
                println!("");

            } else { // throw an RLError...
            }
        }

        _ => unreachable!(),
    }

    Ok(result)
}
*/

pub fn qexpr_to_sexpr(qexpr: QExpr) -> SExpr {
    match qexpr {
        QExpr::Atom(atom) => {
            let atom_string = atom.get_atom_string();
            SExpr::Atom(RLAtom::new(&atom_string))
        }

        QExpr::QCons(cons) => {
            let first = qexpr_to_sexpr(cons.car());

            let second = qexpr_to_sexpr(cons.cdr());

            let comma = cons.get_comma();

            SExpr::SCons(RLCons::<SExpr>::new_with_comma(first,
                                                         second,
                                                         comma))
        }

        QExpr::QList2(mut list) => {
            let mut s_list = LinkedList::<SExpr>::new();

            while list.is_empty().not() {
                let q_elem = list.pop_back().unwrap();
                s_list.push_front(qexpr_to_sexpr(q_elem));
            }
            SExpr::SList(RLList::<SExpr>::new_with_list(&s_list))
        }

        QExpr::Nil(nil) => SExpr::Nil(nil),

        QExpr::Symb(sym) => {
            let sym_name = sym.get_symbol_name();

            SExpr::Symb(RLSymbol::new_with_str(&sym_name))
        }
    }
}

pub fn sexpr_to_qexpr(sexpr: SExpr) -> QExpr {
    match sexpr {
        SExpr::Atom(atom) => {
            let string = atom.get_atom_string();
            QExpr::Atom(RLAtom::new(&string))
        }

        SExpr::SCons(cons) => {
            let first = sexpr_to_qexpr(cons.car());

            let second = sexpr_to_qexpr(cons.cdr());

            let comma = cons.get_comma();

            QExpr::QCons(RLCons::<QExpr>::new_with_comma(first,
                                                         second,
                                                         comma))
        }

        SExpr::Nil(nil) => QExpr::Nil(nil),

        SExpr::SList(slist) => {
            let mut q_ll = LinkedList::<QExpr>::new();

            let mut list = slist.get_linked_list();

            while list.is_empty().not() {
                let s_elem = list.pop_back().unwrap();
                q_ll.push_front(sexpr_to_qexpr(s_elem));
            }

            let qlist = RLList::<QExpr>::new_with_list(&q_ll);

            QExpr::QList2(qlist)
        }

        SExpr::Symb(sym) => {
            let sym_name = sym.get_symbol_name();

            QExpr::Symb(RLSymbol::new_with_str(&sym_name))
        }

        t => {
            // Dummy or Cons
            println!("OTHER: {:?}", t);
            unreachable!()
        }
    }
}

pub fn qexprll_to_sexprll(mut list: LinkedList<QExpr>) ->
    LinkedList<SExpr> {

    let mut sexprll = LinkedList::<SExpr>::new();

    while list.is_empty().not() {

        let item = list.pop_back().unwrap();

        sexprll.push_front(qexpr_to_sexpr(item));
 }

    sexprll
}

pub fn sexprll_to_qexprll(mut list: LinkedList<SExpr>) ->
    LinkedList<QExpr> {

    let mut qexprll = LinkedList::<QExpr>::new();

    while list.is_empty().not() {

        let item = list.pop_back().unwrap();

        qexprll.push_front(sexpr_to_qexpr(item));
 }

    qexprll
}
