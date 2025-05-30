// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

// (list (+ 1 2) (+ 10 20)) -> (3 30)
// (cons (list 1 2) (list 3 4)) -> ((1 2) 3 4)

// (car (list (list 1 2) 3 4)) -> (1 2)
// (cons 1 (list (list 1 2) 3 4)) -> (1 (1 2) 3 4)

// (car (list (+ 1 2) (+ 10 20))) -> 3

// (eval (cdr '(cdr cdr (list 5 6 7)))) -> todo!()

// (cdr (list 2 3 (list 6 7))) -> (3 (6 7))
// (cdr (list 2 (list 5 6))) -> ((5 6))

// (macroexpand-1 '(defun foo (x) "doc" (print x)))
// (macroexpand-1 '(defmacro bla (x) "doc" `(print ,x)))

use std::collections::LinkedList;
use std::ops::Not;

use expr::expr::{qexprll_to_sexprll};

use expr::list::RLList;
use expr::sexpr::SExpr;

use pars_symb::symbol::Symbol;

#[derive(Debug, Clone)]
pub struct RLZipper {
    pub sexpr: SExpr,
    pub final_sexpr: SExpr,

    pub depth_in_parent: usize,
    pub index_in_parent: usize,

    pub parent: Option<Box<RLZipper>>,

    pub children: Option<Box<RLZipper>>,
}

impl RLZipper {
    pub fn new(sexpr: &SExpr,

               depth: usize,
               index: usize,

               parent: Option<Box<RLZipper>>,

               children: Option<Box<RLZipper>>) ->
        Self {

        let sexpr = sexpr.clone();
        let final_sexpr = sexpr.clone();

        let depth_in_parent = depth;
        let index_in_parent = index;

        let parent = parent;
        let children = children;

      Self { sexpr,
             final_sexpr,
             depth_in_parent,
             index_in_parent,
             parent, children }
    }

    pub fn child(&mut self,
                 new_sexpr: SExpr,
                 old_final_sexpr: SExpr,
                 new_depth: usize,
                 new_index: usize) -> RLZipper {
        // Return child RLZipper
        RLZipper {
            sexpr: new_sexpr.clone(),
            final_sexpr: old_final_sexpr.clone(),

            depth_in_parent: new_depth,
            index_in_parent: new_index,

            parent: Some(Box::new(self.clone())),

            children: None,
        }
    }

    pub fn inject_in_sexpr(&self,
                           mut lhs_items: usize,
                           parent_sexpr: SExpr,
                           index: usize,
                           mut current_depth: usize,
                           depth: usize,
                           rs_symb: Symbol,
                           mut rs_ll: LinkedList<SExpr>) -> SExpr {
        println!("Begin - inject_in_sexpr");

        println!("parent_sexpr: {:?}", parent_sexpr);

        println!("current_depth: {:?}", current_depth);
        println!("depth: {:?}", depth);
        println!("");

        println!("rs_symb: {:?}", rs_symb);
        println!("");

        while lhs_items > 0 {
            rs_ll.pop_front();
            lhs_items = lhs_items - 1;
        }
        println!("rs_ll: {:?}", rs_ll);
        println!("");

        println!("self.sexpr: {:?}", self.sexpr);
        println!("");

        let mut lhs_rs_ll = LinkedList::<SExpr>::new();
        let mut cons_reached: bool = false;

        let mut intermediate_sexpr: SExpr;

        let mut result_ll = rs_ll;

        let mut current_index = 0;

        match parent_sexpr {
            SExpr::Cons(symb, ll) => {
                println!("match parent_sexpr");
                println!("");

                if contains_sexpr_cons(ll.clone()).not() {
                    println!("before return, ll: {:?}", ll);
                    println!("");
                    return SExpr::Cons(symb, ll);
                }

                let mut iter = ll.iter();

                let len_ll = ll.len();

                println!("len_ll: {:?}", len_ll);
                let mut count_ll = 0;

                while count_ll <= len_ll {
                    println!("start while count_ll <= len_ll");
                    println!("");

                    println!("current_depth: {:?}", current_depth);
                    println!("depth: {:?}", depth);

                    println!("current_index: {:?}", current_index);
                    println!("index: {:?}", index);
                    println!("");

                    let item = iter.next();

                    count_ll = count_ll + 1;

                    if current_depth == depth &&
                       depth == 0 &&
                       current_index == index {
                           if let SExpr::Cons(_s, _l) = &self.sexpr {
                               result_ll.pop_back();
                           }

                           while lhs_rs_ll.is_empty().not() {
                                result_ll.push_back(
                                    lhs_rs_ll.pop_front().unwrap());
                           }

                           result_ll.push_back(self.sexpr.clone());
                           println!("current_depth 0 - result_ll: {:?}", result_ll);
                           println!("");

                           return SExpr::Cons(symb, result_ll);
                    }

                    println!("item: {:?}", item);
                    println!("");

                    match item {
                        Some(SExpr::Cons(sym, list)) => {
                            current_depth = current_depth + 1;

                            cons_reached = true;

                            if current_depth < depth {
                                println!("prior rec - current_depth: {:?}",
                                    current_depth);
                                println!("");

                                intermediate_sexpr =
                                    SExpr::Cons(sym.clone(), list.clone());

                                while lhs_rs_ll.is_empty().not() {
                                    result_ll.push_front(
                                        lhs_rs_ll.pop_back().unwrap());
                                        lhs_items = lhs_items + 1;
                                }

                                println!("lhs_rs_ll: {:?}", lhs_rs_ll);
                                println!("");

                                let depth0_sexpr =
                                    self.inject_in_sexpr(
                                        lhs_items,
                                        intermediate_sexpr,
                                        index,
                                        current_depth,
                                        depth,
                                        sym.clone(),
                                        result_ll.clone());

                                 result_ll.push_back(depth0_sexpr);

                                 println!("!= depth - result_ll: {:?}", result_ll);
                                 println!("");

                                 current_index = current_index + 1;
                            }

                            if current_depth == depth &&
                               current_index == index {
                                println!("------------------------");
                                println!("current_depth: {:?}", current_depth);
                                println!("current_index: {:?}", current_index);

                                println!("self.sexpr: {:?}", self.sexpr);
                                println!("");

                                if let SExpr::Cons(_s, _l) = &self.sexpr {
                                    result_ll.pop_back();
                                }

                                while lhs_rs_ll.is_empty().not() {
                                    result_ll.push_back(
                                    lhs_rs_ll.pop_front().unwrap());
                                }

                                result_ll.push_back(self.sexpr.clone());

                                println!("result_ll: {:?}", result_ll);
                                println!("");
                            }

                            if current_depth == depth &&
                               current_index != index {
                                    println!("current_index != index");
                                    println!("");
                                result_ll.push_back(
                                    SExpr::Cons(sym.clone(), list.clone()));

                                println!("result_ll: {:?}", result_ll);
                                current_depth = current_depth - 1;
                                current_index = current_index + 1;
                            }

                        }

                        Some(SExpr::SList(s_list)) => {
                            if cons_reached {
                                result_ll.push_back(
                                    SExpr::SList(s_list.clone()));
                            } else {
                                lhs_rs_ll.push_back(
                                    SExpr::SList(s_list.clone()));
                            }

                            current_index = current_index + 1;
                        }

                        Some(SExpr::Atom(atom)) => {
                            if cons_reached {
                                result_ll.push_back(
                                    SExpr::Atom(atom.clone()));
                            } else {
                                lhs_rs_ll.push_back(
                                    SExpr::Atom(atom.clone()));
                            }

                            current_index = current_index + 1;
                            println!("ATOM_ATOM");
                            println!("result_ll: {:?}", result_ll);
                            println!("lhs_rs_ll: {:?}", lhs_rs_ll);
                            println!("");
                        }

                        Some(SExpr::Nil(nil)) => {
                            if cons_reached {
                                result_ll.push_back(SExpr::Nil(nil.clone()));
                            } else {
                                lhs_rs_ll.push_back(SExpr::Nil(nil.clone()));
                            }

                            current_index = current_index + 1;
                        }

                        None => { break; }

                        /*
                        Some(qexpr_to_sexpr(QExpr::QList(qlist))) => {
                            if cons_reached {
                                result_ll.push_back(
                                    qexpr_to_sexpr(QExpr::QList(qlist.clone())));
                            } else {
                                lhs_rs_ll.push_back(
                                    SExpr::SList(qlist.clone()));
                            }

                            current_index = current_index + 1;
                        }
                        */

                        Some(SExpr::QList(qlist)) => {
                            println!("Attention - qlist: {:?}", qlist);
                            println!("");

                            let list = qlist.get_linked_list();

                            if cons_reached {
                                result_ll.push_back(
                                    SExpr::QList(qlist.clone()));
                            } else {
                                lhs_rs_ll.push_back(
                                    SExpr::SList(
                                        RLList::<SExpr>::new_with_list(
                                            &qexprll_to_sexprll(
                                                list.clone()))));
                            }
                            current_index = current_index + 1;
                        }

                        Some(SExpr::Symb(symb)) => {
                            if cons_reached {
                                result_ll.push_back(
                                    SExpr::Symb(symb.clone()));
                            } else {
                                lhs_rs_ll.push_back(
                                    SExpr::Symb(symb.clone()));
                            }

                            current_index = current_index + 1;
                        }

                        _ => todo!(),
                    }
                }
                return SExpr::Cons(rs_symb, result_ll);
            }

            _ => unreachable!(),
        }
    }

    pub fn parent_of_children(&mut self) {

        println!("");
        println!("---------------------------------------------");
        println!("Begin - zipper.parent_of_children");
        println!("");

        println!("sexpr: {:?}", self.sexpr);
        println!("");

        let depth = self.children
                        .clone()
                        .unwrap()
                        .depth_in_parent;

        println!("depth_in_parent: {:?}", depth);
        println!("");

        let index = self.children
                        .clone()
                        .unwrap()
                        .index_in_parent;

        println!("index_in_parent: {:?}", index);
        println!("self.index_in_parent: {:?}", self.index_in_parent);
        println!("");

        let sexpr = self.children
                        .clone()
                        .unwrap()
                        .sexpr;

        println!("children_sexpr: {:?}", sexpr);
        println!("");

        // Destructure the parent RLZipper
        let RLZipper {
            sexpr:       mut parent_sexpr,
            final_sexpr: mut parent_final_sexpr,

            depth_in_parent: parent_depth_in_parent,
            index_in_parent: parent_index_in_parent,

            parent: parent_parent,
            children: mut parent_children,
        } = *self.children
                 .clone()
                 .unwrap()
                 .parent
                 .clone()
                 .unwrap();

        println!("Before - parent_sexpr: {:?}", parent_sexpr);
        println!("");

        let result_symb: Symbol;
        let result_ll = LinkedList::<SExpr>::new();

        match parent_sexpr {
            SExpr::Cons(ref symb, ref _ll) => { result_symb = symb.clone(); }
            _ => unreachable!(),
        }

        println!("depth: {:?}", depth);
        println!("");

        parent_sexpr = self.inject_in_sexpr(0,
                                            parent_sexpr,
                                            index,
                                            0,
                                            depth,
                                            result_symb,
                                            result_ll);

        println!("After - parent_sexpr: {:?}", parent_sexpr);
        println!("");

        if let Some(ref mut child) = parent_children {
            println!("Before - parent_children_sexpr: {:?}", child.sexpr);
            println!("");

            let children_symb: Symbol;
            let children_ll = LinkedList::<SExpr>::new();

            match child.sexpr {
                SExpr::Cons(ref symb, ref _ll) => {
                    children_symb = symb.clone();
                }

                _ => unreachable!(),
            }

            println!("TTT child.depth: {:?}", child.depth_in_parent);
            println!("");

            child.sexpr = self.inject_in_sexpr(0,
                                               child.sexpr.clone(),
                                               child.index_in_parent,
                                               0,

                                               child.depth_in_parent,

                                               children_symb,
                                               children_ll);

            println!("After - parent_children_sexpr: {:?}", child.sexpr);
            println!("");
        }

        println!("Before - parent_final_sexpr: {:?}", parent_final_sexpr);
        println!("");

        let result_final_symb: Symbol;
        let result_final_ll = LinkedList::<SExpr>::new();

        println!("WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW");
        let mut depths_parent_final_sexpr = vec![];

        get_depths(parent_final_sexpr.clone(),
                  0,
                  &mut depths_parent_final_sexpr);
        println!("DEPTHS: {:?}", depths_parent_final_sexpr);

        let max_depth_ref = depths_parent_final_sexpr.iter().max().unwrap();
        let max_depth = *max_depth_ref;

        println!("{:?}", max_depth);
        println!("");

        println!("WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW");
        println!("");

        match parent_final_sexpr {
            SExpr::Cons(ref symb, ref _ll) => { result_final_symb = symb.clone(); }
            _ => unreachable!(),
        }

        parent_final_sexpr = self.inject_in_sexpr(0,
                                                  parent_final_sexpr,
                                                  index,
                                                  0,
                                                  max_depth,
                                                  result_final_symb,
                                                  result_final_ll);

        println!("After - parent_final_sexpr: {:?}", parent_final_sexpr);
        println!("");

        println!("End - zipper.parent_of_children");
        println!("");

        // Return a new RLZipper focused on the parent.
        *self = RLZipper {
            sexpr: parent_sexpr,
            final_sexpr: parent_final_sexpr,

            depth_in_parent: parent_depth_in_parent,
            index_in_parent: parent_index_in_parent,

            parent: parent_parent,
            children: parent_children,
        }
    }

    pub fn parent(&mut self) {
        println!("");
        println!("Begin - zipper.parent");
        println!("");

        let depth = self.depth_in_parent;

        println!("depth_in_parent: {:?}", depth);
        println!("");

        let index = self.index_in_parent;

        println!("index_in_parent: {:?}", index);
        println!("");

        // Destructure the parent Zipper
        let RLZipper {
            sexpr: parent_sexpr,
            final_sexpr: mut parent_final_sexpr,

            depth_in_parent: parent_depth_in_parent,
            index_in_parent: parent_index_in_parent,

            parent: parent_parent,
            children: parent_children,
        } = *self.parent.clone().unwrap();

        println!("Parent, Before - parent_final_sexpr: {:?}", parent_final_sexpr);
        println!("");


        println!("parent_sexpr: {:?}", parent_sexpr);
        println!("");

        println!("");
        println!("");

        println!("parent_children: {:?}", parent_children);
        println!("");

        let result_final_symb: Symbol;
        let result_final_ll = LinkedList::<SExpr>::new();

        println!("parent_final_sexpr: {:?}", parent_final_sexpr);
        println!("");

        match parent_final_sexpr {
            SExpr::Cons(ref symb, ref _ll) => {
                result_final_symb = symb.clone();
            }

            /*
            SExpr::Symb(ref symb) => {
                println!("SYMB-SYMB-SYMB");
                result_final_symb = symb.clone();
            }
            */

            /*
            SExpr::Atom(ref atom) => {
                println!("ATOM-ATOM-ATOM");
                result_final_symb = Symbol::new("block".to_string());
            }
            */

            _ => unreachable!(),
        }

        println!("YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY");
        let mut depths_parent_final_sexpr = vec![];

        get_depths(parent_final_sexpr.clone(),
                  0,
                  &mut depths_parent_final_sexpr);
        println!("DEPTHS: {:?}", depths_parent_final_sexpr);

        let max_depth_ref = depths_parent_final_sexpr.iter().max().unwrap();
        let max_depth = *max_depth_ref;

        println!("{:?}", max_depth);
        println!("");

        println!("YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY");
        println!("");

        parent_final_sexpr = self.inject_in_sexpr(0,
                                                  parent_final_sexpr,
                                                  index,
                                                  0,
                                                  max_depth,
                                                  result_final_symb,
                                                  result_final_ll);

        println!("After - parent_final_sexpr: {:?}", parent_final_sexpr);
        println!("");

        println!("End - zipper.parent");
        println!("");

        // Return a new RLZipper focused on the parent.
        *self = RLZipper {
            sexpr: parent_sexpr,
            final_sexpr: parent_final_sexpr,

            depth_in_parent: parent_depth_in_parent,
            index_in_parent: parent_index_in_parent,

            parent: parent_parent,
            children: parent_children,
        }
    }
}

pub fn get_depths(sexpr: SExpr,
                  mut count: usize,
                  result_depths: &mut Vec<usize>) {
    match sexpr {
        SExpr::Cons(_symb, ll) => {

            let mut s_list = ll.clone();

            loop {
                match s_list.pop_front() {
                    Some(SExpr::Cons(sym, list)) =>  {
                        count = count + 1;
                            get_depths(SExpr::Cons(sym, list),
                                      count,
                                      result_depths);
                    }

                    None => {
                        result_depths.push(count);
                        break;
                    }

                    _ => {}
                }
            }
        }
        _ => {}
    }
}

pub fn contains_sexpr_cons(mut list: LinkedList<SExpr>) -> bool {
    let mut result: bool = false;

    while list.is_empty().not() {
        let item = list.pop_front();
        match item {
            Some(SExpr::Cons(_symb, _ll)) => {
                result = true;
                return result;
            }
            _ => { result = false; }
        }
     }
     result
}




/*
bz: self.zipper:
RLZipper {
    sexpr:
        Cons(Symbol { name: "list" }, [Atom("3"), Atom("4")]),
    final_sexpr:
        Cons(Symbol { name: "concatenate" }
            [Cons(Symbol { name: "quote" },
                [Symb(Symbol { name: "list" })]),
             Cons(Symbol { name: "list" }, [Atom("1"), Atom("2")]),
             Cons(Symbol { name: "list" }, [Atom("3"), Atom("4")])]),
    depth_in_parent: 0,
    index_in_parent: 2,
    parent:
        Some(RLZipper {
            sexpr:
                Cons(Symbol { name: "list" }, [Atom("1"), Atom("2")]),
            final_sexpr:
                Cons(Symbol { name: "concatenate" },
                    [Cons(Symbol { name: "quote" },
                        [Symb(Symbol { name: "list" })]),
                     Cons(Symbol { name: "list" },
                         [Atom("1"), Atom("2")]),
                     Cons(Symbol { name: "list" },
                         [Atom("3"), Atom("4")])]),
            depth_in_parent: 0,
            index_in_parent: 1,
            parent:
                Some(RLZipper {
                    sexpr:
                        Cons(Symbol { name: "quote" },
                            [Symb(Symbol { name: "list" })]),
                    final_sexpr:
                        Cons(Symbol { name: "concatenate" },
                            [Cons(Symbol { name: "quote" },
                                [Symb(Symbol { name: "list" })]),
                             Cons(Symbol { name: "list" },
                                 [Atom("1"), Atom("2")]),
                             Cons(Symbol { name: "list" },
                                 [Atom("3"), Atom("4")])]),
                    depth_in_parent: 0,
                    index_in_parent: 0,
                    parent:
                        Some(RLZipper {
                            sexpr:
                                Cons(Symbol { name: "concatenate" },
                                    [Cons(Symbol { name: "quote" },
                                         [Symb(Symbol { name: "list" })]),
                                     Cons(Symbol { name: "list" },
                                         [Atom("1"), Atom("2")]),
                                     Cons(Symbol { name: "list" },
                                         [Atom("3"), Atom("4")])]),
                            final_sexpr:
                                Cons(Symbol { name: "concatenate" },
                                    [Cons(Symbol { name: "quote" },
                                         [Symb(Symbol { name: "list" })]),
                                     Cons(Symbol { name: "list" },
                                         [Atom("1"), Atom("2")]),
                                     Cons(Symbol { name: "list" },
                                         [Atom("3"), Atom("4")])]),
                            depth_in_parent: 0,
                            index_in_parent: 0,
                            parent: None,
                            children: None }),
                    children: None }),
            children: None }),
    children: None }
*/
