// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;
use std::collections::{BTreeMap, LinkedList};

use crate::atom::RLAtom;
use crate::cons::RLCons;
use crate::list::RLList;
use crate::nil::RLNil;
use crate::qexpr::QExpr;
use crate::symb::RLSymbol;
use crate::QuoteTrait;

use err::err::RLError;

use pars_symb::symbol::Symbol;
use pars_symb::token::Token;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Debug, Clone)]
pub enum SExpr {
    Atom(RLAtom),

    // Func(RLAtom),
    Lambda(RLAtom),

    Nil(RLNil),

    Symb(RLSymbol),

    SList(RLList<SExpr>),
    SCons(RLCons<SExpr>),

    Cons(Symbol, LinkedList<SExpr>),

    // type necessary for parsing purposes
    Dummy,

    // type necessary for zipper replacements?!
    QList(RLList<QExpr>),

    // types used for form* evaluations
    SForm(Vec<Token>),
    SToken(Token),

    // type used for macro replacements
    SBTreeMap(BTreeMap<String, SExpr>),
}

impl SExpr { }

impl QuoteTrait for SExpr {

    fn quote(sexpr: SExpr) -> Result<SExpr, RLError> {
        println!("in QuoteTrait sexpr - quote");

        match sexpr {
            SExpr::Cons(symb, ll) => {
                Ok(SExpr::Cons(symb, ll))
            }

            SExpr::SList(list) => {
                Ok(SExpr::SList(list))
            }

            SExpr::Atom(atom)  => {
                Ok(SExpr::Atom(atom))
            }

            SExpr::Nil(nil) => {
                Ok(SExpr::Nil(nil))
            }

            SExpr::Symb(symb) => {
                Ok(SExpr::Symb(symb))
            }

            _ => unreachable!()
        }
    }
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // println!("in fmt for SExpr");
        // println!("self: {:?}", self);

        match self {
            SExpr::Atom(a) => write!(f, "{}", a.get_atom_string()),

            // SExpr::Func(func) => write!(f, "{}", func),

            SExpr::Lambda(lambda) => write!(f, "{}", lambda),

            SExpr::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest.iter() {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }

            SExpr::QList(qlist) => {
                // println!("Formatter for SExpr::QList: {:?}", qlist);
                // println!("");

                let list = qlist.get_linked_list();

                let mut iter = list.iter().peekable();

                for sexpr in list.clone() {
                    iter.next();

                    let _ = match sexpr {
                        QExpr::Symb(s) => {
                            let s_name = s.get_symbol_name();

                            match &*s_name {
                                "(" => {
                                    write!(f, "{}", s_name)
                                }

                                ")" => {
                                    Ok(if let None = iter.peek() {
                                        write!(f, "{}", s_name)?
                                    } else {
                                        write!(f, "{}", s_name)?
                                    })
                                }

                                _ => { write!(f, "{} ",
                                    s_name.to_uppercase()) }
                            }
                        }

                        QExpr::QList2(s_list) => {
                            if let Some(QExpr::Atom(_atom)) = iter.peek() {
                                write!(f, "{} ",
                                    QExpr::QList2(s_list.clone()))
                            } else {
                                write!(f, "{}",
                                    QExpr::QList2(s_list.clone())) }
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

            SExpr::Nil(nil) => write!(f, "{}", nil),

            SExpr::Dummy => write!(f, ""),

            SExpr::SList(slist) => {
                let list = slist.get_linked_list();

                // list.push_back(SExpr::Symb(RLSymbol::new_with_str(")")));
                // list.push_front(SExpr::Symb(RLSymbol::new_with_str("(")));

                // println!("list: {:?}", list);
                // println!("");

                let mut iter = list.iter().peekable();

                for sexpr in list.clone() {
                    iter.next();

                    let _ = match sexpr {
                        SExpr::Symb(symb) => {
                            let symb_name = symb.get_symbol_name();

                            match &*symb_name {
                                "(" => {
                                    write!(f, "{}", symb_name)
                                }

                                ")" => {
                                    Ok(if let None = iter.peek() {
                                        write!(f, "{}", symb_name)?
                                    } else {
                                        if let Some(SExpr::Symb(s)) =
                                            iter.peek() {

                                            let s_name = s.get_symbol_name();

                                            match &*s_name {
                                                "(" => {

                                                write!(f, "{} ", symb)? }

                                                ")" => { write!(f, "{}", symb)? }

                                                _ => write!(f, "{} ", symb)?,
                                            }
                                        } else {
                                            write!(f, "{} ", symb_name)?
}
                                    })
                                }

                                "'" => {
                                    write!(f, "{}", symb_name)
                                }

                                "`" => {
                                    write!(f, "{}", symb_name)
                                }

                                "," => {
                                    write!(f, "{}", symb_name)
                                }

                                _ => {
                                      Ok(if let None = iter.peek() {
                                        write!(f, "{}", symb_name
                                                           .to_uppercase())?
                                    } else {
                                        if let Some(SExpr::Symb(s)) =
                                            iter.peek() {

                                            let s_name = s.get_symbol_name();

                                            match &*s_name {

                                                "(" => {
                                                    write!(f, "{} ",
                                                        symb_name
                                                           .to_uppercase())?
                                                }

                                                ")" => {
                                                    write!(f, "{}",
                                                        symb_name
                                                           .to_uppercase())?
                                                }

                                                _ => {
                                                    write!(f, "{} ",
                                                        symb_name
                                                           .to_uppercase())?
                                                }
                                            }
                                        } else {
                                            write!(f, "{} ",
                                                symb_name.to_uppercase())?
                                        }
                                    })
                                    /*
                                    write!(f, "{} ", symb_name
                                                         .to_uppercase())
                                   */
                                }
                            }
                        }

                        SExpr::SList(s_list) => {
                            if let Some(SExpr::Atom(_atom)) = iter.peek() {
                                write!(f, "{} ", SExpr::SList(s_list.clone()))
                            } else { write!(f, "{}", SExpr::SList(s_list.clone())) }
                        }

                        SExpr::Atom(a) => {
                            let atom_string = a.get_atom_string();

                            if let Some(SExpr::Symb(s)) = iter.peek() {
                                let s_name = s.get_symbol_name();

                                match &*s_name {
                                    "(" => { write!(f, "{} ", atom_string)? }

                                    ")" => {
                                        write!(f, "{}", atom_string)?
                                    }

                                    _ => {
                                        write!(f, "{} ", atom_string)?
                                    }
                                }
                            }

                            if let Some(SExpr::Atom(_)) = iter.peek() {
                                write!(f, "{} ", atom_string)?
                            }

                            if let Some(SExpr::SList(_list)) = iter.peek() {
                                write!(f, "{} ", atom_string)
                            } else { Ok(()) }
                        }

                        SExpr::Nil(nil) => write!(f, "{}", nil),

                        _ => write!(f, ""),
                    };
                }
                Ok(())
            }

            SExpr::SForm(form) =>
                write!(f, "{:?}", form),

            SExpr::SToken(token) =>
                write!(f, "{:?}", token),

            SExpr::SBTreeMap(btm) =>
                write!(f, "{:?}", btm),

            SExpr::SCons(cons) => {
                let first = cons.car();

                let second = cons.cdr();

                write!(f, "({} . {})", first, second)
            }

            SExpr::Symb(symb) => write!(f, "{}", symb),
        }
    }
}

/////////////////////////////////////////////////////////////
