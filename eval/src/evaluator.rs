// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{BTreeMap, LinkedList};
use std::ops::Not;

use crate::named_ds_bind::RLNamedDsBind;
use crate::zipper::RLZipper;

use cl::creator::CLCreator;

use env::block::RLBlock;
use env::env::RLEnvironment;
use env::env_trait::{EnvRef, EnvTrait};
use env::lambda::RLLambda;
use env::named_lambda::RLNamedLambda;
use env::result::RLResult;
use env::symb::RLEnvSymbol;
use env::var::RLVar;

use err::err::{RLError,
               UnboundVariableError,
               UndefinedFuncError};

use expr::atom::RLAtom;
use expr::expr::{qexpr_to_sexpr, sexpr_to_qexpr};
use expr::expr::Expr;
use expr::list::RLList;
use expr::nil::RLNil;
use expr::sexpr::SExpr;
use expr::symb::RLSymbol;

use pars::parser::RLParser;

use pars_symb::symbol::Symbol;
use pars_symb::token::Token;

pub struct RLEvaluator {
    sexpr: SExpr,

    env: EnvRef,

    cl_create: CLCreator,

    pub parser: RLParser,

    zipper: RLZipper,

    named_ds_bind: RLNamedDsBind,

    error: Option<RLError>,

    macro_mode: bool,
}

impl RLEvaluator {

    pub fn new() -> RLEvaluator {

        let env = RLEnvironment::new();

        let sexpr = SExpr::Nil(RLNil::new());

        let cl_create = CLCreator::new();

        let parser = RLParser::new();

        let zipper = RLZipper::new(&SExpr::Nil(RLNil::new()),
            0,
            0,
            None,
            None);

        let named_ds_bind = RLNamedDsBind::new();

        let error = None;

        let macro_mode = false;

        RLEvaluator {
            sexpr,
            env,
            cl_create,
            parser,
            zipper,
            named_ds_bind,
            error,
            macro_mode,
        }
    }

    pub fn init(&mut self) {
        let rc_binding = self.env.clone();
        let mut env_binding = rc_binding.borrow_mut();

        env_binding.init();

        self.cl_create.init(env_binding.get_mut_cl_package());

        self.parser.init();

        // drop(rc_binding);
        // drop(env_binding);
    }

     /*
     pub fn get_toplevel_sform(&self) -> SExpr {
     self.toplevel_sform.clone()
     }

    pub fn set_toplevel_sform(&mut self, mut vec: Vec<Token>) {
    let mut result_vec = Vec::<Token>::new();

        result_vec.push(Token::Symb(Symbol::new(")")));

        result_vec.append(&mut vec);

        result_vec.push(Token::Symb(Symbol::new("eval")));
        result_vec.push(Token::Symb(Symbol::new("(")));

        println!("result_vec: {:?}", result_vec);
        println!("");

        self.toplevel_sform = SExpr::SForm(result_vec.clone())
        }
      */

     ///////////////////////////////////////////////////////////
     /*
     Configure functions
      */
     ///////////////////////////////////////////////////////////

     fn set_macro_mode (&mut self, mode: bool) {
         self.macro_mode = mode;
     }

     pub fn configure(&mut self) {
         println!("configure - start");
         println!("");

         // self.set_toplevel_sform(self.parser.get_tokens());

         self.sexpr = self.parser.get_sexpr().clone();
         self.error = self.parser.get_error();

         println!("self.sexpr: {:?}", self.sexpr);
         println!("");

         println!("self.error: {:?}", self.error);
         println!("");



        
         let is_toplevel: bool;

         let mut is_macro = false;

         let eval_sexpr = self.get_sexpr();

         // realize env binding
         let rc_binding = self.env.clone();

         let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
             Rc::downgrade(&rc_binding);

         let binding = weak_binding.upgrade().unwrap();
         drop(rc_binding);

         let mut env_binding = binding.borrow_mut();

         is_toplevel = env_binding.is_toplevel();

         println!("toplevel is: {:?}", is_toplevel);
         println!("");

         match &self.sexpr {
             SExpr::Cons(symb, ll) => {
                 let symb_string = &*symb.name;

                 println!("{:?}", symb_string);
                 println!("");

                 if let Some(symb) = env_binding.get_symbol(
                     &symb_string.to_string()) {

                         is_macro = symb.get_is_macro();
                     }

                 if contains_sexpr_cons(ll.clone()) {
                     println!("contains a cons");
                     println!("");
                 }
             }

             _ => {}
         }

         drop(env_binding);
         drop(weak_binding);

         if is_toplevel &&
             is_macro {

                 let mut linked_list = LinkedList::<SExpr>::new();

                 linked_list.push_front(eval_sexpr.clone());

                 self.set_sexpr(SExpr::Cons(Symbol::new("eval"), linked_list));
             }




         self.zipper = RLZipper { sexpr: self.sexpr.clone(),
             final_sexpr: self.sexpr.clone(),
             depth_in_parent: 0,
             index_in_parent: 0,
             parent: None,
             children: None {},
         };

         self.build_zipper(self.sexpr.clone());

         self.substitute_var_atoms();

         println!("self.sexpr: {:?}", self.sexpr);
         println!("");

         println!("self.error: {:?}", self.error);
         println!("");

         println!("configure - end");
         println!("");
     }

     pub fn configure_with_sexpr(&mut self, sexpr: SExpr) {
         self.sexpr = self.build_sexpr_with_dyn_env(sexpr);

         println!("configure_with_sexpr - start");
         println!("");

         println!("after bsexpr: {}", self.sexpr);
         println!("");

         println!("self.zipper: {:?}", self.zipper);
         println!("");

         self.zipper =  RLZipper { sexpr: self.sexpr.clone(),
             // final_sexpr: self.zipper.final_sexpr.clone(),
             final_sexpr: self.sexpr.clone(),
             depth_in_parent: self.zipper.depth_in_parent,
             index_in_parent: self.zipper.index_in_parent,
             parent: self.zipper.parent.clone(),
             children: self.zipper.children.clone(),
         };

         self.build_zipper(self.sexpr.clone());

         self.substitute_var_atoms();

         println!("configure_with_sexpr - end");
         println!("");
     }

     pub fn configure_with_sexpr_macro(&mut self, sexpr: SExpr) {
         self.sexpr = sexpr;

         println!("configure_with_sexpr_macro - start");
         println!("");

         println!("self.sexpr: {:?}", self.sexpr);
         println!("");

         println!("self.error: {:?}", self.error);
         println!("");

         /*
         self.zipper =  RLZipper { sexpr: self.sexpr.clone(),
         final_sexpr: self.sexpr.clone(),
         depth_in_parent: 0,
         index_in_parent: 0,
         parent: None,
         children: None{},
         };
          */

         self.zipper =  RLZipper { sexpr: self.sexpr.clone(),
             // final_sexpr: self.zipper.final_sexpr.clone(),
             final_sexpr: self.sexpr.clone(),
             depth_in_parent: self.zipper.depth_in_parent,
             index_in_parent: self.zipper.index_in_parent,
             parent: self.zipper.parent.clone(),
             children: self.zipper.children.clone(),
         };

         self.build_zipper(self.sexpr.clone());

         /*
         self.substitute_var_atoms();

        println!("self.sexpr: {:?}", self.sexpr);
        println!("");

        println!("self.error: {:?}", self.error);
        println!("");
          */

         println!("Configure__with_sexpr_macro - end");
         println!("");
     }





     ///////////////////////////////////////////////////////////
     /*
     Build sexpr with dyn_env functions
      */
     ///////////////////////////////////////////////////////////

     fn get_var_from_dyn_env(&self, atom: String) -> Result<RLVar, RLError> {
         // realize env binding to environment
         let rc_binding = self.env.clone();

         let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
             Rc::downgrade(&rc_binding);

         let binding = weak_binding.upgrade().unwrap();
         drop(rc_binding);

         let mut env_binding = binding.borrow_mut();

         if let Some(var) =
            env_binding.get_curr_eval_dyn_env_var(&atom.to_uppercase()) {

                Ok(var)
            } else {
                let err = UnboundVariableError::new(&atom.to_uppercase());
                Err(RLError::UnboundVariableError(err))
            }
     }

     fn build_children_sexpr_with_dyn_env(&mut self,
         mut list: LinkedList<SExpr>) ->
        LinkedList<SExpr> {

            println!("Begin - bsexpr_children");
            println!("");

            println!("list: {:?}", list);
            println!("");

            let mut result_ll = LinkedList::<SExpr>::new();

            if contains_sexpr_cons(list.clone()) {
                loop {
                    match list.pop_front() {
                        Some(SExpr::Cons(sym, ll)) => {
                            println!("SEXpr::Cons {:?} - {:?}", sym, ll);
                            println!("");

                            // linked_list = ll.clone();
                            result_ll.push_back(
                                SExpr::Cons(sym,
                                    self.build_children_sexpr_with_dyn_env(ll)));
                        }

                        Some(SExpr::Atom(atom)) => {
                            println!("bsexpr_children: SExpr::Atom {:?}", atom);
                            println!("");

                            if let Ok(mut var) = self.get_var_from_dyn_env(
                                atom.to_string()) {

                                    println!("var: {:?}", var);
                                    println!("");

                                    println!("{}", var.var_to_sexpr());
                                    println!("");

                                    result_ll.push_back(var.var_to_sexpr());
                                } else {
                                    result_ll.push_back(SExpr::Atom(atom));
                                }
                        }

                        None => { break; }

                        _ => todo!(),
                    }
                }
                self.build_children_sexpr_with_dyn_env(result_ll.clone());
            } else {
                loop {
                    match list.pop_front() {
                        Some(SExpr::Atom(atom)) => {
                            println!("bsexpr: SExpr::Atom {:?}", atom);
                            println!("");

                            if let Ok(mut var) = self.get_var_from_dyn_env(
                                atom.to_string()) {

                                    println!("var: {:?}", var);
                                    println!("");

                                    println!("{}", var.var_to_sexpr());
                                    println!("");

                                    result_ll.push_back(var.var_to_sexpr());
                                } else {
                                    result_ll.push_back(SExpr::Atom(atom));
                                }
                        }

                        Some(SExpr::Nil(nil)) => {
                            result_ll.push_back(SExpr::Nil(nil));
                        }

                        None => { break; }

                        _ => todo!(),
                    }
                }
            }
            println!("End - bsexpr_children");
            println!("");

            result_ll
        }

     #[allow(unused_assignments, unused_variables)]
     fn build_sexpr_with_dyn_env(&mut self, sexpr: SExpr) -> SExpr {

         let mut result = SExpr::Nil(RLNil::new());
         let mut result_ll = LinkedList::<SExpr>::new();

         let mut count = 0;

         println!("bsexpr - sexpr: {}", sexpr);
         println!("");

         match sexpr {
             SExpr::Cons(symb, ll) => {
                 println!("----------------------------------------------");
                 println!("Begin - bsexpr: First SExpr::Cons {:?} -- {:?}",
                     symb, ll);
                 println!("");

                 let mut s_list = ll.clone();

                 loop {
                     match s_list.pop_front() {
                         Some(SExpr::Cons(sym, list)) => {
                             count = count + 1;
                             println!("Child - {:?}", count);

                             println!("list: {:?}", list);
                             println!("");

                             result_ll.push_back(
                                 SExpr::Cons(sym,
                                     self.build_children_sexpr_with_dyn_env(
                                         list.clone())));
                         }

                         Some(SExpr::Atom(atom)) => {
                             println!("bsexpr: SExpr::Atom {:?}", atom);
                             println!("");

                             if let Ok(mut var) = self.get_var_from_dyn_env(
                                 atom.to_string()) {

                                     println!("var: {:?}", var);
                                     println!("");

                                     println!("{}", var.var_to_sexpr());
                                     println!("");

                                     result_ll.push_back(var.var_to_sexpr());
                                 } else {
                                     result_ll.push_back(SExpr::Atom(atom))
                                 };
                         }

                         Some(SExpr::SList(slist)) => {
                             println!("bsexpr: SExpr::SList {:?}", slist);
                             println!("");

                             let mut slist_ll = slist.get_linked_list();

                             let mut new_slist_ll = LinkedList::<SExpr>::new();

                             new_slist_ll.push_front(
                                 SExpr::Symb(RLSymbol::new_with_str(")")));

                             // remove ")"
                             slist_ll.pop_back();

                             loop {
                                 if let Some(item) = slist_ll.pop_back() {
                                     match item {
                                         SExpr::Atom(atom) => {
                                             if let Ok(mut var) =
                                                self.get_var_from_dyn_env(
                                                    atom.to_string()) {

                                                        println!("var: {:?}", var);
                                                        println!("");

                                                        println!("{}", var.var_to_sexpr());
                                                        println!("");

                                                        new_slist_ll.push_front(
                                                            var.var_to_sexpr());

                                                    } else {
                                                        new_slist_ll.push_front(
                                                            SExpr::Atom(atom))
                                                    };
                                         }

                                         SExpr::Symb(symb) => {
                                             if slist_ll.is_empty() {
                                                 new_slist_ll.push_front(
                                                     SExpr::Symb(symb));
                                                 break;
                                             } else {
                                                 new_slist_ll.push_front(
                                                     SExpr::Symb(symb));
                                             }
                                         }

                                         _ => {
                                             new_slist_ll.push_front(item);
                                         }
                                     }
                                 }
                             }

                             let new_slist =
                                 SExpr::SList(
                                     RLList::<SExpr>::new_with_list(
                                         &new_slist_ll));

                             result_ll.push_back(new_slist);
                         }

                         Some(SExpr::SForm(form)) => {
                             result_ll.push_back(SExpr::SForm(form))
                         }

                         Some(SExpr::SToken(token)) => {
                             result_ll.push_back(SExpr::SToken(token))
                         }

                         Some(SExpr::Symb(symb)) => {
                             result_ll.push_back(SExpr::Symb(symb))
                         }

                         None => {
                             println!("End - bsexpr: End reached");
                             println!("-----------------------------------");
                             println!("");

                             result = SExpr::Cons(symb.clone(), result_ll);
                             break;
                         }

                         t => {
                             println!("bsexpr: t {:?}", t);
                             println!("")
                         }
                     }
                 }
             }

             SExpr::Atom(atom) => {
                 let atom_string = atom.get_atom_string();

                 if let Ok(mut var) = self.get_var_from_dyn_env(atom_string) {

                     println!("var: {:?}", var);
                     println!("");

                     result = var.var_to_sexpr();
                 } else {
                     result = SExpr::Atom(atom.clone());
                 }
             }

             SExpr::Nil(nil) => {
                 result = SExpr::Nil(nil)
             }

             SExpr::SList(list) => {
                 result = SExpr::SList(list)
             }

             SExpr::Symb(symb) => {
                 result = SExpr::Symb(symb)
             }

             SExpr::QList(qlist) => {
                 result = SExpr::QList(qlist)
             }

             /*
             SExpr::Comma(atom, sexpr) => {
             result = SExpr::Comma(atom, sexpr)
             }
              */

             _ => todo!(),
         }

         result
     }





     ///////////////////////////////////////////////////////////
     /*
     Build zipper functions
      */
     ///////////////////////////////////////////////////////////

     pub fn build_children(&mut self,
         depth: usize,
         mut list: LinkedList<SExpr>) {
             println!("Begin - b_children {:?}", list);
             println!("");

             let current_depth = depth + 1;
             let mut current_index: i32 = -1;

             let mut linked_list = list.clone();

             if contains_sexpr_cons(list.clone()) {
                 loop {
                     match list.pop_front() {
                         Some(SExpr::Cons(sym, ll)) => {
                             println!("SEXpr::Cons {:?} - {:?}", sym, ll);
                             println!("");

                             current_index = current_index + 1;

                             println!("SEXpr::Cons current_index: {:?}",
                                 current_index);
                             println!("");

                             linked_list = ll.clone();

                             //initialize zipper.children
                             println!("self.zipper.children == None");
                             self.zipper.children =
                            Some(Box::new(RLZipper {
                                sexpr: SExpr::Cons(sym.clone(), ll.clone()),
                                final_sexpr: self.zipper.final_sexpr.clone(),
                                depth_in_parent: current_depth,
                                index_in_parent: current_index as usize,

                                parent: Some(Box::new(self.zipper.clone())),
                                children: None }));
                         }

                         None => { break; }

                         _ => { current_index = current_index + 1;
                             println!("_ current_index: {:?}", current_index)
                         }
                     }
                 }
                 self.build_children(current_depth,
                     linked_list);
             } else {
                 println!("End - b_children");
                 println!("");

                 return;
             }
         }

     pub fn build_zipper(&mut self, sexpr: SExpr) {
         let mut current_index = 0;
         let mut count = 0;

         match sexpr {
             SExpr::Cons(symb, ll) => {
                 println!("");
                 println!("----------------------------------------------");
                 println!("Begin - bz: First SExpr::Cons {:?} -- {:?}", symb, ll);
                 println!("");

                 let mut s_list = ll.clone();

                 loop {
                     match s_list.pop_front() {
                         Some(SExpr::Cons(sym, list)) => {
                             count = count + 1;
                             println!("Child - {:?}", count);

                             self.zipper = self.zipper.child(
                                 SExpr::Cons(sym.clone(), list.clone()),
                                 self.zipper.final_sexpr.clone(),
                                 0,
                                 current_index);
                             // parent);

                             println!("bz: self.children {:?}", self.zipper.children);
                             println!("");

                             current_index = current_index + 1;

                             println!("bz: sexpr: {:?}", self.zipper.sexpr);
                             println!("");

                             println!("bz: final_sexpr: {:?}",
                                 self.zipper.final_sexpr);
                             println!("");

                             println!("bz: index_in_parent: {:?}",
                                 self.zipper.index_in_parent);
                             println!("");

                             // if list contains SExpr::Cons
                             // go with functions which build up recursively
                             // all childs
                             self.build_children(0,
                                 list.clone());
                             println!("bz: self.zipper: {:?}", self.zipper);
                             println!("");

                             println!("bz: self.zipper.children: {:?}", self.zipper.children);
                             println!("");
                         }

                         /*
                         Some(SExpr::Atom(atom)) => {
                         current_index = current_index + 1;
                         println!("bz: SExpr::Atom {:?}", atom);
                         println!("");

                            let var = self.get_var_from_dyn_env(atom.to_string());
                            println!("var: {:?}", var);
                            println!("");
                            }
                          */

                         None => {
                             println!("End - bz: End reached");
                             println!("-----------------------------------");
                             println!("");
                             break;
                         }

                         t => {
                             current_index = current_index + 1;
                             println!("bz: t {:?}", t);
                             println!("")
                         }
                     }
                 }
             }

             SExpr::Atom(atom) => {
                 println!("evaluator::build_zipper - atom branch");
                 println!("");

                 self.zipper =
                    self.zipper.child(SExpr::Atom(atom.clone()),
                        self.zipper.final_sexpr.clone(),
                        0,
                        current_index);
                 // parent);
             }

             SExpr::Nil(nil) => {
                 println!("evaluator::build_zipper - nil branch");
                 println!("");

                 self.zipper =
                    self.zipper.child(SExpr::Nil(nil),
                        self.zipper.final_sexpr.clone(),
                        0,
                        current_index);
                 // parent);
             }

             SExpr::Symb(symb) => {
                 println!("evaluator::build_zipper - symb branch");
                 println!("");

                 self.zipper =
                    self.zipper.child(SExpr::Symb(symb),
                        self.zipper.final_sexpr.clone(),
                        0,
                        current_index);
                 // parent);
             }

             /*
             SExpr::QList(qlist) => {
             println!("evaluator::build_zipper - qlist branch");
             println!("");

                self.zipper =
                self.zipper.child(SExpr::QList(qlist),
                self.zipper.final_sexpr.clone(),
                0,
                current_index);
                // parent);
                   }
              */

             SExpr::SList(slist) => {
                 println!("evaluator::build_zipper - slist branch");
                 println!("");

                 self.zipper =
                    self.zipper.child(SExpr::SList(slist),
                        self.zipper.final_sexpr.clone(),
                        0,
                        current_index);
                 // parent);
             }
             _ => unreachable!(),
         }
     }





     ///////////////////////////////////////////////////////////
     /*
     Substitution of var exprs
      */
     ///////////////////////////////////////////////////////////

     pub fn is_keyword(&self, keyword: &String) -> bool {
         // realize env binding to environment
         let rc_binding = self.env.clone();

         let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
             Rc::downgrade(&rc_binding);

         let binding = weak_binding.upgrade().unwrap();
         drop(rc_binding);

         let env_binding = binding.borrow_mut();

         if env_binding.is_keyword(keyword.clone()) {
             drop(env_binding);
             return true;
         } else {
             drop(env_binding);
             return false;
         }
     }

     fn get_var_from_curr_package(&self, atom: String) ->
        Result<RLResult, RLError> {

            // realize env binding to environment
            let rc_binding = self.env.clone();

            let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
                Rc::downgrade(&rc_binding);

            let binding = weak_binding.upgrade().unwrap();
            drop(rc_binding);

            let mut env_binding = binding.borrow_mut();

            let curr_pack = env_binding.get_mut_current_package();

            if let Some(var_symb) = curr_pack.get_symbol(&atom.to_uppercase()) {
                println!("var_symb: {:?}", var_symb);
                println!("");

                let rl_dyn_var = var_symb.dyn_var.clone().unwrap();

                drop(env_binding);
                drop(weak_binding);

                Ok(RLResult::VarRes(rl_dyn_var))
            } else {
                let err = UnboundVariableError::new(&atom.to_uppercase());
                Err(RLError::UnboundVariableError(err))
            }
        }

     fn get_last_var_from_curr_package(&self, atom: String) ->
        Result<RLResult, RLError> {

            let result: RLResult;

            // realize env binding to environment
            let rc_binding = self.env.clone();

            let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
                Rc::downgrade(&rc_binding);

            let binding = weak_binding.upgrade().unwrap();
            drop(rc_binding);

            let mut env_binding = binding.borrow_mut();

            let curr_pack = env_binding.get_mut_current_package();

            if let Some(var_symb) = curr_pack.get_symbol(&atom.to_uppercase()) {
                println!("var_symb: {:?}", var_symb);
                println!("");

                let rl_dyn_var = var_symb.dyn_var.clone().unwrap();

                let rl_var = rl_dyn_var.get_var();

                match rl_var {
                    /*
                    Some(RLVar::SAtomVar(SExpr::Func(atom))) => {
                    let atom_string = atom.get_atom_string()
                    .to_lowercase();

                    if let Some(symbol) = curr_pack.get_symbol(&atom_string) {

                        let named_lambda = symbol.get_named_lambda()
                        .expect("REASON");

                        result = RLResult::FuncRes(named_lambda.clone());

                    } else {
                    let err = UnboundVariableError::new(
                    &atom_string.to_uppercase());

                        return Err(RLError::UnboundVariableError(err))
                        }
                        }
                     */

                    Some(RLVar::SAtomVar(SExpr::Lambda(atom))) => {
                        let atom_string = atom.get_atom_string();

                        println!("we are here!!");
                        println!("");

                        if let Some(lambda) = curr_pack.get_lambda(&atom_string) {
                            result = RLResult::LambdaRes(lambda.clone());

                        } else {
                            let err = UnboundVariableError::new(
                                &atom_string.to_uppercase());

                            return Err(RLError::UnboundVariableError(err))
                        }
                    }

                    _ => {
                        result = RLResult::VarRes(rl_dyn_var);
                    }
                }

                drop(env_binding);
                drop(weak_binding);

                Ok(result)

            } else {
                let err = UnboundVariableError::new(&atom.to_uppercase());
                Err(RLError::UnboundVariableError(err))
            }
        }

     pub fn substitute_var_atoms(&mut self) {
         println!("Begin - substitute_var_atoms");
         println!("");

         match &self.zipper.final_sexpr {
             SExpr::Cons(symb, ll) => {
                 let mut linked_list = ll.clone();
                 let mut result_ll = LinkedList::<SExpr>::new();

                 println!("Begin - substitute_var_atoms");
                 println!("");

                 match &*symb.name {
                     "backquote" |
                    "defparameter" |
                    "quote" => {
                        self.zipper.final_sexpr = SExpr::Cons(symb.clone(),
                            ll.clone());
                    }

                     _ => {
                         for _n in 0..linked_list.len() {
                             let item = linked_list.pop_front().unwrap();

                             println!("item: {:?}", item);
                             println!("");

                             if let SExpr::Atom(ref atom) = item {
                                 let atom_string = atom.get_atom_string();

                                 if self.is_keyword(&atom_string) {
                                     result_ll.push_back(item);

                                     // number atom
                                 } else if let Ok(_f64_atom) =
                            atom_string.parse::<f64>() {

                                result_ll.push_back(item);

                                // parse atom string
                            } else {

                                let string_atom =
                                    atom_string.parse::<String>().unwrap();

                                // string atom
                                if string_atom.starts_with("\"") &&
                                    string_atom.ends_with("\"") {

                                        result_ll.push_back(item);
                                    } else {
                                        // var atom
                                        match self.get_var_from_curr_package(
                                            atom_string) {

                                                Ok(RLResult::VarRes(mut var)) => {

                                                    println!("var: {:?}", var);
                                                    println!("");

                                                    let sexpr: SExpr = var.var_to_sexpr();

                                                    result_ll.push_back(sexpr);
                                                }

                                                Err(err) => {
                                                    self.error = Some(err);
                                                }

                                                _ => unreachable!(),
                                            }
                                    }
                            }
                             } else if let SExpr::Symb(ref symb) = item {
                                 println!("item is a symb");
                                 println!("");

                                 let symb_string = symb.get_symbol_name();

                                 match self.get_var_from_curr_package(
                                     symb_string) {

                                         Ok(RLResult::VarRes(mut var)) => {

                                             println!("var: {:?}", var);
                                             println!("");

                                             let sexpr: SExpr = var.var_to_sexpr();

                                             result_ll.push_back(sexpr);
                                         }

                                         Err(_err) => {
                                             result_ll.push_back(item);
                                         }

                                         _ => unreachable!(),
                                     }
                             } else {
                                 result_ll.push_back(item);
                             }
                         } // for

                         self.zipper.final_sexpr = SExpr::Cons(symb.clone(),
                             result_ll);
                     }
                 }
             }

             SExpr::Atom(atom) => {
                 let atom_string = atom.get_atom_string();

                 if self.is_keyword(&atom_string) {
                     // keyword atom
                     self.sexpr = SExpr::Atom(atom.clone());

                 } else if let Ok(_f64_atom) = atom_string.parse::<f64>() {
                     // number atom
                     self.sexpr = SExpr::Atom(atom.clone());

                 } else {
                     // string atom
                     let string_atom = atom_string.parse::<String>().unwrap();

                     // rlstring atom
                     if string_atom.starts_with("\"") &&
                         string_atom.ends_with("\"") {

                             self.sexpr = SExpr::Atom(atom.clone());
                         } else {
                             match self.get_var_from_curr_package(atom_string) {

                                 Ok(RLResult::VarRes(mut var)) => {
                                     println!("var: {:?}", var);
                                     println!("");

                                     let sexpr: SExpr = var.var_to_sexpr();

                                     self.sexpr = sexpr;
                                 }

                                 Err(err) => {
                                     self.error = Some(err);
                                 }
                                 _ => unreachable!(),
                             }
                         }
                 }
             }

             SExpr::Symb(symb) => {
                 let symb_name = symb.get_symbol_name();

                 match self.get_var_from_curr_package(symb_name.clone()) {

                     Ok(RLResult::VarRes(mut var)) => {
                         println!("var: {:?}", var);
                         println!("");

                         let sexpr: SExpr = var.var_to_sexpr();

                         self.sexpr = sexpr;
                     }

                     Err(err) => {
                         self.error = Some(err);
                     }

                     _ => unreachable!(),
                 }
             }

             SExpr::Nil(_nil) => {}

             _ => todo!(),
         }

         println!("End - substitute_var_atoms");
         println!("");
     }





     ///////////////////////////////////////////////////////////
     /*
     eval functions
      */
     ///////////////////////////////////////////////////////////

     fn eval_children(&mut self) ->
        Result<RLResult, RLError> {

            println!("begin - eval_children!");

            match &self.zipper.children.clone().unwrap().sexpr {
                SExpr::Cons(symb, _ll) => {
                    println!("cons occurred");

                    let children_sexpr = self.zipper
                        .children
                        .clone()
                        .unwrap()
                        .sexpr
                        .clone();

                    println!("eval_children - children_sexpr:");
                    println!("{:?}", children_sexpr);
                    println!("");

                    let result = self.eval_rl_symbol(symb.clone(),
                        children_sexpr);

                    // if let Some(ref _parent) = self.zipper.parent {
                    println!("parent is there");
                    println!("");

                    match result {
                        Ok(RLResult::ExprRes(ref expr)) => {
                            match expr {
                                Expr::QExpr(qexpr) =>
                                self.zipper.sexpr =
                                    qexpr_to_sexpr(qexpr.clone()),

                                Expr::SExpr(sexpr) =>
                                self.zipper.sexpr =
                                    sexpr.clone(),
                            }
                        }

                        Ok(RLResult::QExprRes(ref qexpr)) => {
                            self.zipper.sexpr =
                            qexpr_to_sexpr(qexpr.clone());
                        }

                        Ok(RLResult::BlockRes(ref _block)) =>
                        todo!(),

                        Ok(RLResult::SExprRes(ref sexpr)) =>
                        self.zipper.sexpr = sexpr.clone(),

                        Ok(RLResult::NilRes(ref nil)) =>
                        self.zipper.sexpr = SExpr::Nil(nil.clone()),

                        Ok(RLResult::BoolRes(_bool)) =>
                        todo!(),

                        Ok(RLResult::NumRes(f64)) =>
                        self.zipper.sexpr =
                            SExpr::Atom(RLAtom::new(&f64.to_string())),

                        Ok(RLResult::StrRes(ref str)) =>
                        self.zipper.sexpr =
                            SExpr::Atom(RLAtom::new(str)),

                        Ok(RLResult::StringRes(ref rlstring)) =>
                        self.zipper.sexpr =
                            SExpr::Atom(RLAtom::new(&rlstring.get())),

                        Ok(RLResult::EnvRes(_env)) =>
                        todo!(),

                        Ok(RLResult::FuncRes(ref func)) =>
                        self.zipper.sexpr =
                            SExpr::Atom(RLAtom::new(&func.get_name())),

                        Ok(RLResult::LambdaRes(ref lambda)) =>
                        self.zipper.sexpr =
                            SExpr::Lambda(RLAtom::new(&lambda.get_id())),

                        Ok(RLResult::SymbolRes(_symb)) =>
                        todo!(),

                        Ok(RLResult::SymbRes(ref symb)) =>
                        self.zipper.sexpr =
                            SExpr::Symb(RLSymbol::new_with_symb(&symb)),

                        Ok(RLResult::VarRes(_var)) =>
                        todo!(),

                        Ok(RLResult::ReturnFromRes(_)) =>
                        todo!(),

                        Ok(RLResult::NamedDsBindRes(_ds)) =>
                        todo!(),

                        Ok(RLResult::MacroRes(_func)) =>
                        todo!(),

                        Err(err) => { println!("errrrr: {:?}", err);
                            unreachable!()
                        }
                    }
                    // }
                    println!("self.zipper.sexpr: {:?}", self.zipper.sexpr);
                    println!("self.zipper.final_sexpr: {:?}",
                        self.zipper.final_sexpr);
                    println!("");

                    self.zipper.parent_of_children();

                    println!("self.zipper.sexpr: {:?}", self.zipper.sexpr);
                    println!("self.zipper.final_sexpr: {:?}",
                        self.zipper.final_sexpr);
                    println!("");

                    return self.eval();
                    // return result;
                    // result
                }
                _ => unreachable!(),
            }
        }

     pub fn eval(&mut self) -> Result<RLResult, RLError> {
         println!("");
         println!("Begin - Eval");
         println!("");

         println!("zipper: {:?}", self.zipper);
         println!("");

         println!("zipper.sexpr: {:?}", self.zipper.sexpr);
         println!("");

         println!("zipper.final_sexpr: {:?}", self.zipper.final_sexpr);
         println!("");

         // self.substitute_var_atoms();

         // avoid deep evaluation of a certain sexpr
         match &self.zipper.final_sexpr {
             SExpr::Cons(symb, _ll) => {
                 match &*symb.name {
                     "backquote" |
                    // "block" |
                    // "progn" |
                    // "defparameter" |
                    "quote" => {
                        return self.eval_rl_symbol(symb.clone(),
                            self.zipper.final_sexpr.clone());
                    }

                     // "block" |
                     // "comma" |
                     // "defparameter" |
                     /*
                     "defun" => {
                     return self.eval_rl_symbol(symb.clone(),
                     self.zipper.sexpr.clone());
                     }
                      */

                     _ => { }
                 }
             }

             _ => { }
         }

         // continue with deep evaluation...
         if let Some(ref mut pr) = self.zipper.parent {

             pr.final_sexpr = self.zipper.final_sexpr.clone();
         }

         if let Some(ref mut child) = self.zipper.children {
             println!("prior - zipper.children.sexpr: {:?}", child.sexpr);
             println!("");
             println!("prior - zipper.children.final_sexpr: {:?}",
                 child.final_sexpr);
             println!("");

             child.final_sexpr = self.zipper.final_sexpr.clone();

             if let Some(ref mut pr) = child.parent {
                 pr.sexpr = self.zipper.sexpr.clone();
                 pr.final_sexpr = self.zipper.final_sexpr.clone();
             }

             println!("post - zipper.children.sexpr: {:?}", child.sexpr);
             println!("");

             println!("post - zipper.children.final_sexpr: {:?}",
                 child.final_sexpr);
             println!("");

             println!("post - zipper.children.parent.sexpr: {:?}",
                 child.parent
                 .clone()
                 .unwrap()
                 .sexpr);
             println!("");

             println!("post - zipper.children.parent.final_sexpr: {:?}",
                 child.parent
                 .clone()
                 .unwrap()
                 .final_sexpr);
             println!("");

         } else {
             println!("no zipper.children!");
             println!("");
         }

         if let Some(_child) = &self.zipper.children {
             // let _ = self.eval_children();
             let _ = return self.eval_children();
         }

         if let Some(err) = &self.error {
             println!("before error......");
             return Err(err.clone());
         } else if let None = self.zipper.parent {
             match &self.zipper.final_sexpr {
                 SExpr::Cons(symb, _ll) => {
                     println!("no parent -> before symbol match");
                     println!("");

                     self.eval_rl_symbol(symb.clone(),
                         self.zipper.final_sexpr.clone())
                 }
                 _ => panic!("no parent - last SExpr-option"),
             }
         } else {
             match &self.zipper.sexpr {
                 SExpr::Cons(symb, _ll) => {
                     println!("parent -> before symbol match");
                     println!("");

                     let result = self.eval_rl_symbol(symb.clone(),
                         self.zipper.sexpr.clone());

                     if let Some(ref _parent) = self.zipper.parent {
                         match result {
                             Ok(RLResult::BlockRes(ref _block)) =>
                                todo!(),

                             Ok(RLResult::ExprRes(ref expr)) => {
                                 match expr {
                                     Expr::QExpr(qexpr) =>
                                        self.zipper.sexpr =
                                            qexpr_to_sexpr(qexpr.clone()),

                                     Expr::SExpr(sexpr) =>
                                        self.zipper.sexpr = sexpr.clone(),
                                 }
                             }

                             Ok(RLResult::QExprRes(ref qexpr)) => {
                                 println!("in QExprRes");
                                 self.zipper.sexpr =
                                    qexpr_to_sexpr(qexpr.clone());

                                 println!("self.zipper.sexpr: {:?}",
                                     self.zipper.sexpr);
                             }

                             Ok(RLResult::SExprRes(ref sexpr)) =>
                                self.zipper.sexpr = sexpr.clone(),

                             Ok(RLResult::NilRes(ref nil)) =>
                                self.zipper.sexpr = SExpr::Nil(nil.clone()),

                             Ok(RLResult::BoolRes(_bool)) =>
                                todo!(),

                             Ok(RLResult::NumRes(f64)) =>
                                self.zipper.sexpr =
                                    SExpr::Atom(RLAtom::new(&f64.to_string())),

                             Ok(RLResult::StrRes(ref str)) =>
                                self.zipper.sexpr =
                                    SExpr::Atom(RLAtom::new(str)),

                             Ok(RLResult::StringRes(ref rlstring)) =>
                                self.zipper.sexpr =
                                    SExpr::Atom(RLAtom::new(&rlstring.get())),

                             Ok(RLResult::EnvRes(_env)) =>
                                todo!(),

                             Ok(RLResult::FuncRes(ref func)) => {
                                 if let Some(lambda) = func.get_lambda() {
                                     self.zipper.sexpr =
                                        SExpr::Lambda(
                                            RLAtom::new(&lambda.get_id()));
                                 } else {
                                     self.zipper.sexpr =
                                        SExpr::Atom(
                                            RLAtom::new(&func.get_name()));
                                 }
                                 /*
                                 SExpr::Symb(
                                 RLSymbol::new_with_str(
                                 &func.get_name()));
                                  */
                             }

                             Ok(RLResult::LambdaRes(ref lambda)) => {
                                 println!("in LambdaRes");
                                 println!("");

                                 self.zipper.sexpr =
                                    /*
                                    SExpr::Atom(
                                    RLAtom::new(
                                    &lambda.get_id())),
                                     */

                                    SExpr::Lambda(
                                        RLAtom::new(
                                            &lambda.get_id()))
                             }

                             Ok(RLResult::SymbolRes(_symb)) =>
                                todo!(),

                             Ok(RLResult::SymbRes(ref symb)) =>
                                self.zipper.sexpr =
                                    SExpr::Symb(RLSymbol::new_with_symb(&symb)),

                             Ok(RLResult::VarRes(_var)) =>
                                todo!(),

                             Ok(RLResult::ReturnFromRes(_)) =>
                                todo!(),

                             Ok(RLResult::NamedDsBindRes(_ds)) =>
                                todo!(),

                             Ok(RLResult::MacroRes(_func)) =>
                                todo!(),

                             Err(ref err) => {
                                 println!("err: {:?}", err);
                                 println!("");
                             }
                         }
                     }
                     self.zipper.parent();
                     return self.eval();
                 }

                 SExpr::Atom(atom) => {
                     println!("atom-atom");
                     return Ok(RLResult::SExprRes(SExpr::Atom(atom.clone())))
                 }

                 SExpr::Nil(nil) => {
                     println!("nil-nil");
                     return Ok(RLResult::NilRes(nil.clone()))
                 }

                 SExpr::Symb(symb) => {
                     println!("symb-symb");
                     if self.macro_mode {
                         Ok(RLResult::SymbRes(symb.get_symbol()))
                     } else {
                        
                         println!("bla");
                         let item =
                             self.get_last_var_from_curr_package(
                                 symb.to_string());

                         println!("item: {:?}", item);
                         println!("");
                         item
                        
                     }
                 }

                 /*
                 SExpr::QList(qlist) => {
                 println!("qlist-qlist");
                 return Ok(RLResult::SExprRes(SExpr::QList(qlist.clone())))
                 }
                  */

                 SExpr::SList(slist) => {
                     println!("slist-qlist");
                     // return Ok(RLResult::SExprRes(
                     // SExpr::SList(slist.clone())))
                     return Ok(RLResult::QExprRes(
                         sexpr_to_qexpr(SExpr::SList(slist.clone()))))
                 }

                 _ => unreachable!(),
             }
         }
         // }
     }

     pub fn run_closure(&mut self,
         sexpr: SExpr,
         symbol: &mut RLEnvSymbol) ->
        Result<RLResult, RLError> {

            let mut result: Result<RLResult, RLError> =
                Ok(RLResult::NilRes(RLNil::new()));

            let mut named_lambda = symbol.named_lambda.clone().unwrap();

            // second env binding to deliver env as a parameter
            // of the RLSymbol/RLNamedLambda closure
            let rc_binding = self.env.clone();

            let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
                Rc::downgrade(&rc_binding);

            let binding = weak_binding.upgrade().unwrap();
            drop(rc_binding);

            let mut env_binding = binding.borrow_mut();

            // env_binding.run_closure(sexpr, closure)
            // env_binding.run_closure_rl_symbol(sexpr, symbol)

            if let Some(_closure) = named_lambda.closure {
                println!("in if let sexpr...");

                result = match named_lambda.closure {
                    Some(ref mut f)  => f(sexpr),
                    _ => panic!("Closure not found."),
                };

            } else if let Some(_closure) = named_lambda.closure_env {
                // result = match named_lambda.closure_env {
                result = env_binding.run_closure_rl_symbol(sexpr, symbol);
                /*
                {
                Some(ref mut f)  => f(sexpr, env_binding),
                _ => panic!("Closure_env not found."),
                };
                 */
            }

            println!("before end, run_closure...");
            println!("");

            match result {
                Ok(res)  => {
                    if let env::result::RLResult::SExprRes(sexpr) = res {
                        println!("run_closure - SExprRes");
                        println!("");

                        drop(env_binding);

                        // self.configure_with_sexpr(sexpr.clone());
                        // self.eval()

                        match sexpr {
                            SExpr::SForm(ref _sform) => {
                                self.eval_sform(sexpr)
                            }

                            _ => {
                                self.configure_with_sexpr(sexpr.clone());
                                self.eval()
                            }
                        }

                        /*
                        } else if let env::result::RLResult::QExprRes(qexpr) = res {
                        println!("TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT");
                        println!("");

                    drop(env_binding);

                    println!("qexpr: {:?}", qexpr);
                    println!("");

                    let sexpr = qexpr_to_sexpr(qexpr);
                    println!("new_sexpr: {:?}", sexpr );
                    println!("");

                    self.configure_with_sexpr(sexpr);

                    self.eval()
                         */

                    } else {
                        return Ok(res)
                    }
                }
                Err(err) => return Err(err),
            }
        }

     // symbol evaluation without hard-coded-symbols of example-functions
     pub fn eval_rl_symbol(&mut self,
         symb: Symbol,
         sexpr: SExpr) ->
        Result<RLResult, RLError> {

            println!("Begin eval_rl_symbol");
            println!("");

            println!("symb: {:?}", symb);
            println!("");

            println!("sexpr: {:?}", sexpr);
            println!("");

            // realize env binding to get the appropriate RLSymbol
            let rc_binding = self.env.clone();

            let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
                Rc::downgrade(&rc_binding);

            let binding = weak_binding.upgrade().unwrap();
            drop(rc_binding);

            let mut env_binding = binding.borrow_mut();

            /*
            let next_env_binding = &mut *env_binding;
            let cl_user_pack = next_env_binding.get_mut_cl_user_package();
            println!("pack name is: {}", cl_user_pack.get_name());
             */

            /*
            println!("strong count: {}",
            Rc::strong_count(rc_binding));
            println!("weak count: {}",
            Rc::weak_count(rc_binding));
            println!("");
             */

            // println!("{:?}", env_binding.lex_env);
            // println!("");

            let mut symbol = env_binding.get_symbol(&symb.name)
                .unwrap()
                .clone();

            let home_pack = symbol.get_pack_name();
            println!("home_pack: {}", home_pack);
            println!("");

            // let named_lambda = symbol.get_named_lambda();

            /*
            let closure_binding =
            <Option<Box<for<'a> fn(expr::sexpr::SExpr,
            &'a mut RLEnvironment) ->
            Result<SimpleResult, RLError>>> as Clone>::clone(
            &env_binding.get_symbol(&symb.name).unwrap()
            .named_lambda
            .as_mut()
            .unwrap()
            .closure_env);
            // .unwrap().clone();
             */

            /*
            let closure_binding =
            <Option<for<'a>fn(expr::sexpr::SExpr,
            &'a mut RLEnvironment) ->
            Result<SimpleResult, RLError>>>::clone(
            &env_binding.get_symbol(&symb.name).unwrap()
            .named_lambda
            .as_mut()
            .unwrap()
            .closure_env);

        let closure = closure_binding.unwrap();
             */

            drop(env_binding);
            drop(weak_binding);

            if let None = symbol.named_lambda {
                let err = UndefinedFuncError::new(&symb.name);
                return Err(RLError::UndefinedFuncError(err))
            } else {
                // match env_binding.run_closure_rl_symbol(sexpr, symbol) {
                match self.run_closure(sexpr, &mut symbol) {
                    Ok(res)  => {
                        match res {
                            /*
                            RLResult::StrRes(ref str) => {
                            println!("in RLResult::StrRes");
                            println!("");

                            return Ok(RLResult::StrRes(str.to_string()))
                            }
                             */

                            RLResult::BlockRes(ref block) => {
                                self.eval_block(block.clone())
                            }

                            RLResult::ReturnFromRes(ref block) => {
                                // self.push_chain_block(block.clone());
                                self.eval_return_from(block.clone())
                            }

                            RLResult::FuncRes(ref func) => {
                                if symb.name.eq("funcall") {
                                    self.eval_func(func.clone())

                                } else {
                                    if home_pack.eq("COMMON-LISP") {
                                        return Ok(res)
                                    } else {
                                        if let None = func.get_block() {
                                            Ok(RLResult::FuncRes(func.clone()))
                                        } else {
                                            self.eval_func(func.clone())
                                        }
                                    }
                                }
                            }

                            RLResult::LambdaRes(ref lambda) => {
                                /*
                                if symb.name.eq("function") |
                                symb.name.eq("lambda") {
                                return Ok(RLResult::LambdaRes(lambda.clone()))
                                } else {
                                self.eval_lambda(lambda.clone())
                                }
                                 */

                                if symb.name.eq("funcall") {
                                    self.eval_lambda(lambda.clone())

                                } else {
                                    println!("LambdaRes - Other");
                                    println!("");

                                    return Ok(RLResult::LambdaRes(lambda.clone()))
                                }
                            }

                            RLResult::NamedDsBindRes(ref block) => {
                                println!("self_macro_mode: {:?}", self.macro_mode);
                                println!("");

                                if self.macro_mode.not() {
                                    self.eval_ds(block.clone())
                                } else {
                                    Ok(RLResult::NamedDsBindRes(block.clone()))
                                }
                            }

                            RLResult::MacroRes(ref func) => {
                                if home_pack.eq("COMMON-LISP") {
                                    return Ok(res)
                                } else {
                                    println!("func: {:?}", func);
                                    println!("");

                                    self.eval_macro(func.clone())
                                }
                            }

                            _ => {
                                match &*symb.name {
                                    "quote" => {
                                        match res {
                                            RLResult::ExprRes(Expr::QExpr(qexpr)) =>
                                        return Ok(RLResult::QExprRes(qexpr)),

                                            RLResult::ExprRes(Expr::SExpr(sexpr)) =>
                                        return Ok(RLResult::SExprRes(sexpr)),

                                            _ => unreachable!(),
                                        }
                                    }

                                    _ => {
                                        return Ok(res)
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => return Err(err),
                }
            }
        }





     ///////////////////////////////////////////////////////////
     /*
     Eval RLResult-object functions
      */
     ///////////////////////////////////////////////////////////

     fn search_for_return_from(&self, mut progn_ll: LinkedList<SExpr>) ->
        String {

            let mut result = "".to_string();

            for _n in 0..progn_ll.len() {
                let form = progn_ll.pop_front().unwrap();

                match form {
                    SExpr::SForm(mut sform) => {
                        if sform.is_empty().not() {
                            println!("sform is not empty");
                            println!("sform: {:?}", sform);
                            println!("");

                            for _n in 0..sform.len() {
                                if let Some(Token::Symb(symb)) =
                                sform.pop() {

                                    match &*symb.name {
                                        "return-from" => {
                                            println!("in return_from");
                                            println!("sform: {:?}", sform);
                                            println!("");

                                            if let Some(Token::Atom(atom)) =
                                            sform.pop() {

                                                result = atom.clone()
                                                    .to_uppercase();
                                            }
                                        }

                                        &_ => {}
                                    }
                                }
                            }

                        } else {}
                    }

                    _ => {}
                }
            }

            result
        }

     fn eval_sform(&mut self, sexpr: SExpr) -> Result<RLResult, RLError> {
         println!("Begin eval_sform");
         println!("");

         // let mut result = RLResult::NilRes(RLNil::new());

         if let SExpr::SForm(form) = sexpr {
             self.parser.change_tokens(&form);

             let parse_result = self.parser.parse_to_sexpr();

             println!("parse_result: {:?}", parse_result);
             println!("");

             println!("self.sexpr: {:?}", self.sexpr);
             println!("");

             match parse_result {
                 Ok(ref res)  => {
                     self.parser.set_sexpr(res.clone());
                 }

                 Err(ref err) => {
                     self.parser.set_error(err.clone());
                 }
             }

             if self.macro_mode {
                 println!("Switch to configure_with_sexpr_macro");
                 println!("");
                 self.configure_with_sexpr_macro(parse_result?);
             } else {
                 self.configure_with_sexpr(parse_result?);
             }

             println!("End eval_sform");
             println!("");

             match self.eval() {
                 Ok(res) => Ok(res),

                 Err(err) => return Err(err),
             }

         } else {
             return Err(RLError::SimpleProgramError);
         }
     }

     // fn eval_block(&mut self, mut block: RLBlock, macro_mode: bool) ->
     fn eval_block(&mut self, mut block: RLBlock) ->
        Result<RLResult, RLError> {

            println!("Begin eval_block");
            println!("");

            println!("macro_mode is: {:?}", self.macro_mode);
            println!("");

            println!("BLOCK_YY: {:?}", block);
            println!("");

            // realize env binding
            let rc_binding = self.env.clone();

            let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
                Rc::downgrade(&rc_binding);

            let binding = weak_binding.upgrade().unwrap();
            drop(rc_binding);

            let mut env_binding = binding.borrow_mut();

            env_binding.block_chain_push(&block);

            println!("BLOCK - replace_curr_eval_dyn_env");
            println!("");

            let hash_map = block.get_dyn_env_block();
            env_binding.replace_curr_eval_dyn_env(&hash_map);

            let mut progn_ll = block.get_progn_ll();

            println!("block progn_ll: {:?}", progn_ll);
            println!("");

            let block_name_of_return_from =
                self.search_for_return_from(progn_ll.clone());

            println!("block_name_of_return_from: {:?}",
                block_name_of_return_from);
            println!("");

            let mut block_chain_retained = env_binding.block_chain_retain(
                &block_name_of_return_from);

            println!("RETAINED_block_chain: {:?}", block_chain_retained);
            println!("");

            if block_chain_retained.len() == 1 {
                let mut last_block = env_binding.block_chain_pop().unwrap();

                if last_block.get_name().eq(&block_name_of_return_from) {
                    last_block.set_return_from(true)
                }

                env_binding.block_chain_push(&last_block);
            }

            if block_chain_retained.len() > 1 {
                let id_last_block_retained = block_chain_retained.pop()
                    .unwrap()
                    .get_id();

                let block_chain = env_binding.get_ref_mut_block_chain();

                block_chain.retain_mut(
                    |x| if x.get_id().eq(&id_last_block_retained) &&
                        x.get_name().eq(&block_name_of_return_from) {

                            x.set_return_from(true);
                            true
                        } else {
                            true
                        });

                for _n in 0..block_chain_retained.len() {
                    let id_block = block_chain_retained.pop()
                        .unwrap()
                        .get_id();

                    block_chain.retain_mut(
                        |x| if x.get_id().eq(&id_block) &&
                            x.get_name().eq(&block_name_of_return_from) {

                                x.set_return_from(false);
                                true
                            } else {
                                true
                            });
                }
            }

            drop(env_binding);
            drop(weak_binding);

            let mut result = RLResult::NilRes(RLNil::new());

            if progn_ll.len() > 0 {
                for _n in 0..progn_ll.len() {

                    let sexpr = progn_ll.pop_front().unwrap();

                    match sexpr {
                        SExpr::SForm(form) => {

                            println!("SFORM");
                            println!("");

                            self.parser.change_tokens(&form);

                            let parse_result = self.parser.parse_to_sexpr();

                            println!("parse_result: {:?}", parse_result);
                            println!("");

                            println!("self.sexpr: {:?}", self.sexpr);
                            println!("");

                            match parse_result {
                                Ok(ref res)  => {

                                    /*
                                    if contains_function(&res) {
                                    println!("is function");
                                    println!("");

                                    self.set_sexpr(&res);

                                    match self.eval() {
                                    // Ok(RLResult::FuncRes(res)) => {
                                       Ok(RLResult::SExprRes(res)) => {
                                       self.parser.set_sexpr(
                                       SExpr::res.clone());
                                       }

                                        Err(err) => return Err(err),

                                        _ => { }
                                        }
                                        }
                                     */

                                    self.parser.set_sexpr(res.clone());
                                }

                                Err(ref err) => {
                                    self.parser.set_error(err.clone());
                                }
                            }

                            if self.macro_mode {
                                println!("Switch to configure_with_sexpr_macro");
                                println!("");
                                self.configure_with_sexpr_macro(parse_result?);
                            } else {
                                self.configure_with_sexpr(parse_result?);
                            }

                            match self.eval() {
                                Ok(res) => {
                                    println!("res-slistform: {:?}", res);
                                    println!("");

                                    // realize env binding
                                    let rc_binding = self.env.clone();

                                    let weak_binding:
                                        std::rc::Weak<RefCell<dyn EnvTrait>> =
                                        Rc::downgrade(&rc_binding);

                                    let binding =
                                        weak_binding.upgrade().unwrap();

                                    drop(rc_binding);

                                    let mut env_binding = binding.borrow_mut();

                                    let mut return_from = false;

                                    if let Some(curr_block) =
                                    env_binding.get_block_by_id(
                                        &block.get_id()) {

                                            return_from = curr_block.get_return_from();
                                        }

                                    drop(env_binding);
                                    drop(weak_binding);

                                    if return_from {
                                        return Ok(res);
                                    } else {
                                        result = res;
                                    }
                                }

                                Err(err) => return Err(err),
                            }
                        }

                        SExpr::SToken(token) => {
                            self.parser.change_tokens(&vec!(token));

                            let parse_result = self.parser.parse_to_sexpr();

                            match parse_result {
                                Ok(ref res)  => {
                                    self.parser.set_sexpr(res.clone());
                                }

                                Err(ref err) => {
                                    self.parser.set_error(err.clone());
                                }
                            }

                            self.configure_with_sexpr(parse_result?);

                            match self.eval() {
                                Ok(res) => {
                                    println!("res-token: {:?}", res);
                                    println!("");

                                    // realize env binding
                                    let rc_binding = self.env.clone();

                                    let weak_binding:
                                        std::rc::Weak<RefCell<dyn EnvTrait>> =
                                        Rc::downgrade(&rc_binding);

                                    let binding =
                                        weak_binding.upgrade().unwrap();

                                    drop(rc_binding);

                                    let mut env_binding = binding.borrow_mut();

                                    let mut return_from = false;

                                    if let Some(curr_block) =
                                    env_binding.get_block_by_id(
                                        &block.get_id()) {

                                            return_from = curr_block.get_return_from();
                                        }

                                    drop(env_binding);
                                    drop(weak_binding);

                                    if return_from {
                                        return Ok(res);
                                    } else {
                                        result = res;
                                    }
                                }

                                Err(err) => return Err(err),
                            }
                        }

                        _ => {
                            println!("NO SLIST");
                            println!("");

                            self.configure_with_sexpr(sexpr);

                            match self.eval() {
                                Ok(res)  => {
                                    println!("res-other: {:?}", res);
                                    println!("");

                                    result = res
                                }

                                Err(err) => return Err(err),
                            }
                        }
                    }
                }

                // realize env binding
                let rc_binding = self.env.clone();

                let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
                    Rc::downgrade(&rc_binding);

                let binding = weak_binding.upgrade().unwrap();
                drop(rc_binding);

                let mut env_binding = binding.borrow_mut();

                env_binding.block_chain_pop();

                env_binding.release_curr_eval_dyn_env();

                drop(env_binding);
                drop(weak_binding);

                Ok(result)
            } else { // progn_ll.len = 0 -> NilRes

                // realize env binding
                let rc_binding = self.env.clone();

                let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
                    Rc::downgrade(&rc_binding);

                let binding = weak_binding.upgrade().unwrap();
                drop(rc_binding);

                let mut env_binding = binding.borrow_mut();

                env_binding.block_chain_pop();

                env_binding.release_curr_eval_dyn_env();

                drop(env_binding);
                drop(weak_binding);

                Ok(result)
            }
        }

     fn eval_return_from(&mut self, mut block: RLBlock) ->
        Result<RLResult, RLError> {

            println!("begin eval_return-from");
            println!("");

            // realize env binding
            let rc_binding = self.env.clone();

            let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
                Rc::downgrade(&rc_binding);

            let binding = weak_binding.upgrade().unwrap();
            drop(rc_binding);

            let mut env_binding = binding.borrow_mut();

            env_binding.block_chain_push(&block);

            println!("RETURN-FROM - replace_curr_eval_dyn_env");
            println!("");

            let hash_map = block.get_dyn_env_block();
            env_binding.replace_curr_eval_dyn_env(&hash_map);

            if let Some(block) = env_binding.block_chain_pop() {

                drop(env_binding);
                drop(weak_binding);

                let mut progn_ll = block.get_progn_ll();

                println!("block progn_ll: {:?}", progn_ll);
                println!("");

                let mut result = RLResult::NilRes(RLNil::new());

                if progn_ll.len() > 0 {
                    for _n in 0..progn_ll.len() {

                        let sexpr = progn_ll.pop_front().unwrap();

                        match sexpr {
                            SExpr::SForm(form) => {

                                println!("SFORM");
                                println!("");

                                self.parser.change_tokens(&form);

                                let parse_result = self.parser.parse_to_sexpr();

                                match parse_result {
                                    Ok(ref res)  => {
                                        self.parser.set_sexpr(res.clone());
                                    }

                                    Err(ref err) => {
                                        self.parser.set_error(err.clone());
                                    }
                                }

                                self.configure_with_sexpr(parse_result?);

                                match self.eval() {
                                    Ok(res) => {
                                        println!("res-slistform: {:?}", res);
                                        println!("");

                                        // realize env binding
                                        let rc_binding = self.env.clone();

                                        let weak_binding:
                                            std::rc::Weak<RefCell<dyn EnvTrait>> =
                                            Rc::downgrade(&rc_binding);

                                        let binding =
                                            weak_binding.upgrade().unwrap();

                                        drop(rc_binding);

                                        let mut env_binding = binding.borrow_mut();

                                        let mut return_from = false;

                                        if let Some(curr_block) =
                                    env_binding.get_block_by_id(
                                        &block.get_id()) {

                                            return_from = curr_block.get_return_from();
                                        }

                                        // env_binding.release_curr_eval_dyn_env();

                                        drop(env_binding);
                                        drop(weak_binding);

                                        if return_from {
                                            return Ok(res);
                                        } else {
                                            result = res;
                                        }
                                    }

                                    Err(err) => return Err(err),
                                }
                            }

                            SExpr::SToken(token) => {
                                self.parser.change_tokens(&vec!(token));

                                let parse_result = self.parser.parse_to_sexpr();

                                match parse_result {
                                    Ok(ref res)  => {
                                        self.parser.set_sexpr(res.clone());
                                    }

                                    Err(ref err) => {
                                        self.parser.set_error(err.clone());
                                    }
                                }

                                self.configure_with_sexpr(parse_result?);

                                match self.eval() {
                                    Ok(res) => {
                                        println!("res-token: {:?}", res);
                                        println!("");

                                        // realize env binding
                                        let rc_binding = self.env.clone();

                                        let weak_binding:
                                            std::rc::Weak<RefCell<dyn EnvTrait>> =
                                            Rc::downgrade(&rc_binding);

                                        let binding =
                                            weak_binding.upgrade().unwrap();

                                        drop(rc_binding);

                                        let mut env_binding = binding.borrow_mut();

                                        let mut return_from = false;

                                        if let Some(curr_block) =
                                    env_binding.get_block_by_id(
                                        &block.get_id()) {

                                            return_from = curr_block.get_return_from();
                                        }

                                        // env_binding.release_curr_eval_dyn_env();

                                        drop(env_binding);
                                        drop(weak_binding);

                                        if return_from {
                                            return Ok(res);
                                        } else {
                                            result = res;
                                        }
                                    }

                                    Err(err) => return Err(err),
                                }
                            }

                            _ => {}

                        }
                    }
                    Ok(result)
                } else {
                    Ok(result)
                }
            } else {
                return Err(RLError::SimpleProgramError);
            }
        }

     fn eval_func(&mut self, func: RLNamedLambda) ->
        Result<RLResult, RLError> {

            println!("Begin eval_func");
            println!("");

            /*
            // realize env binding
               let rc_binding = self.env.clone();

        let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
        Rc::downgrade(&rc_binding);

        let binding = weak_binding.upgrade().unwrap();
        drop(rc_binding);

        let mut env_binding = binding.borrow_mut();

        println!("EVAL_FUNC - replace_curr_eval_dyn_env");
        println!("");

        // env_binding.replace_curr_eval_dyn_env(func.get_ref_dyn_env());

        drop(env_binding);
        drop(weak_binding);
             */

            // let sexpr = func.get_body();

            if let None = func.get_block() {
                let lambda = func.get_lambda().expect("REASON");

                self.eval_lambda(lambda.clone())
            } else {
                let block = func.get_block().expect("REASON");

                self.eval_block(block)
            }
        }

     fn eval_lambda(&mut self, mut lambda: RLLambda) ->
        Result<RLResult, RLError> {

            println!("Begin eval_lambda");
            println!("");

            // realize env binding
            let rc_binding = self.env.clone();

            let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
                Rc::downgrade(&rc_binding);

            let binding = weak_binding.upgrade().unwrap();
            drop(rc_binding);

            let mut env_binding = binding.borrow_mut();

            println!("EVAL_LAMBDA - replace_curr_eval_dyn_env");
            println!("");

            env_binding.replace_curr_eval_dyn_env(lambda.get_ref_dyn_env_lambda());

            drop(env_binding);
            drop(weak_binding);

            let sexpr = lambda.get_body();

            self.configure_with_sexpr(sexpr);

            println!("In eval_lambda - self.sexpr: {}", self.sexpr);
            println!("");

            // env_binding.release_curr_eval_dyn_env();

            match self.eval() {
                Ok(res)  => return Ok(res),
                Err(err) => return Err(err),
            }
        }

     fn eval_ds(&mut self, block: RLBlock) ->
        Result<RLResult, RLError> {

            println!("Begin eval_ds");
            println!("");

            println!("block: {:?}", block);
            println!("");

            self.set_macro_mode(true);

            let progn_ll = block.get_progn_ll();

            self.named_ds_bind.set_expr_ll(&progn_ll);

            self.named_ds_bind.set_block(&block);

            self.named_ds_bind.build_macro_sexpr();

            // realize env binding
            let rc_binding = self.env.clone();

            let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
                Rc::downgrade(&rc_binding);

            let binding = weak_binding.upgrade().unwrap();
            drop(rc_binding);

            let env_binding = binding.borrow_mut();

            let mut sexpr = self.named_ds_bind.do_macro_replacements(env_binding);

            println!("sexpr: {:?}", sexpr);
            println!("");

            if let Ok(SExpr::SBTreeMap(ref mut sbtm)) = sexpr {
                let mut new_sbtm = BTreeMap::<String, SExpr>::new();

                for _n in 0..sbtm.len() {
                    if let Some((key, sexpr)) = sbtm.pop_first() {
                        println!("key: {:?}", key);
                        println!("sexpr: {:?}", sexpr);
                        println!("");

                        match self.eval_sform(sexpr) {
                            Ok(res)  => {
                                if let Ok(sexpr) =
                                RLResult::get_sexpr_from_res(res) {

                                    new_sbtm.insert(key, sexpr);
                                }
                            }

                            Err(err) => return Err(err),
                        }
                    }
                }

                println!("new_sbtm: {:?}", new_sbtm);
                println!("");

                sexpr = Ok(self.named_ds_bind.replace_scons(new_sbtm.clone()));
            }

            // drop(env_binding);
            drop(weak_binding);

            match sexpr {
                Ok(sexpr) => {

                    // self.configure_with_sexpr_macro(sexpr);

                    self.set_macro_mode(false);

                    return Ok(RLResult::SExprRes(sexpr));
                }

                Err(err) => {
                    return Err(err);
                }
            }

            /*
            match self.eval() {
            Ok(res)  => {
            self.set_macro_mode(false);

                return Ok(res);
                }

            Err(err) => return Err(err),
            }
             */
        }

     fn eval_macro(&mut self, rlmacro: RLNamedLambda) ->
        Result<RLResult, RLError> {

            println!("Begin eval_macro");
            println!("");

            self.set_macro_mode(true);

            let block = rlmacro.get_block().expect("REASON");

            /*
            let mut toplevel_sform = SExpr::SForm(Vec::<Token>::new());

        if let Some(SExpr::SForm(vec)) = block.get_progn_ll().pop_front() {
        toplevel_sform = SExpr::SForm(vec);
        }
             */

            let old_block = block.clone();

            println!("rlmacro - block: {:?}", block);
            println!("");

            if let Ok(RLResult::NamedDsBindRes(ds_block)) =
            self.eval_block(block) {

                println!("ds_block: {:?}", ds_block);
                println!("");

                self.named_ds_bind.set_expr_ll(&ds_block.get_progn_ll());

                self.named_ds_bind.set_block(&old_block);

                // self.named_ds_bind.set_toplevel_sform(&toplevel_sform);

                self.named_ds_bind.build_macro_sexpr();

                // realize env binding
                let rc_binding = self.env.clone();

                let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
                    Rc::downgrade(&rc_binding);

                let binding = weak_binding.upgrade().unwrap();
                drop(rc_binding);

                let env_binding = binding.borrow_mut();

                let mut sexpr =
                    self.named_ds_bind.do_macro_replacements(env_binding);

                println!("after do_macro_replacements ----------------");
                println!("sexpr: {:?}", sexpr);
                println!("");
                println!("--------------------------------------------");
                println!("");

                if let Ok(SExpr::SBTreeMap(ref mut sbtm)) = sexpr {

                    let mut new_sbtm = BTreeMap::<String, SExpr>::new();

                    for _n in 0..sbtm.len() {
                        if let Some((key, sexpr)) = sbtm.pop_first() {
                            println!("key: {:?}", key);
                            println!("sexpr: {:?}", sexpr);
                            println!("");

                            self.configure_with_sexpr_macro(sexpr);

                            match self.eval() {
                                Ok(res)  => {
                                    if let Ok(sexpr) =
                                    RLResult::get_sexpr_from_res(res) {

                                        new_sbtm.insert(key, sexpr);
                                    }
                                }

                                Err(err) => return Err(err),
                            }
                        }
                    }
                    println!("new_sbtm: {:?}", new_sbtm);
                    println!("");

                    sexpr = Ok(self.named_ds_bind.replace_scons(
                        new_sbtm.clone()));
                }

                // drop(env_binding);
                drop(weak_binding);

                self.set_macro_mode(false);

                println!("End eval_macro");
                println!("");

                Ok(RLResult::SExprRes(sexpr?))
            } else {
                return Err(RLError::SimpleProgramError);
            }
        }

     /*
     fn eval_macro(&mut self, rlmacro: RLNamedLambda) ->
     Result<RLResult, RLError> {

        println!("Begin eval_macro");
        println!("");

        self.set_macro_mode(true);

        /*
        // realize env binding
           let rc_binding = self.env.clone();

        let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
        Rc::downgrade(&rc_binding);

        let binding = weak_binding.upgrade().unwrap();
        drop(rc_binding);

        let env_binding = binding.borrow_mut();

        println!("EVAL_MACRO - replace dyn_env");
        println!("");

        // env_binding.replace_curr_eval_dyn_env(rlmacro.get_ref_dyn_env());

        drop(env_binding);
        drop(weak_binding);
      */

        let block = rlmacro.get_block().expect("REASON");

        /*
        let mut toplevel_sform = SExpr::SForm(Vec::<Token>::new());

        if let Some(SExpr::SForm(vec)) = block.get_progn_ll().pop_front() {
        toplevel_sform = SExpr::SForm(vec);
        }
      */

        let old_block = block.clone();

        println!("rlmacro - block: {:?}", block);
        println!("");

        if let Ok(RLResult::NamedDsBindRes(ds_block)) =
        self.eval_block(block) {

            println!("ds_block: {:?}", ds_block);
            println!("");

            self.named_ds_bind.set_expr_ll(&ds_block.get_progn_ll());

            self.named_ds_bind.set_block(&old_block);

            // self.named_ds_bind.set_toplevel_sform(&toplevel_sform);

            self.named_ds_bind.build_macro_sexpr();

            // realize env binding
               let rc_binding = self.env.clone();

            let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
            Rc::downgrade(&rc_binding);

            let binding = weak_binding.upgrade().unwrap();
            drop(rc_binding);

            let env_binding = binding.borrow_mut();

            let mut sexpr =
            self.named_ds_bind.do_macro_replacements(env_binding);

            println!("after do_macro_replacements ----------------");
            println!("sexpr: {:?}", sexpr);
            println!("");
            println!("--------------------------------------------");
            println!("");

            if let Ok(SExpr::SBTreeMap(ref mut sbtm)) = sexpr {

                let mut new_sbtm = BTreeMap::<String, SExpr>::new();

                for _n in 0..sbtm.len() {
                if let Some((key, sexpr)) = sbtm.pop_first() {
                println!("key: {:?}", key);
                println!("sexpr: {:?}", sexpr);
                println!("");

                        self.configure_with_sexpr_macro(sexpr);

                        match self.eval() {
                        Ok(res)  => {
                        if let Ok(sexpr) =
                        RLResult::get_sexpr_from_res(res) {

                                    new_sbtm.insert(key, sexpr);
                                    }
                                    }

                            Err(err) => return Err(err),
                            }
                            }
                            }
                            println!("new_sbtm: {:?}", new_sbtm);
                            println!("");

                sexpr = Ok(self.named_ds_bind.replace_scons(
                new_sbtm.clone()));
                }

            // drop(env_binding);
               drop(weak_binding);

              // realize env binding
                 let rc_binding = self.env.clone();

            let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
            Rc::downgrade(&rc_binding);

            let binding = weak_binding.upgrade().unwrap();
            drop(rc_binding);

            let env_binding = binding.borrow_mut();

            // env_binding.release_curr_eval_dyn_env();
               let is_toplevel = env_binding.is_toplevel();

            println!("toplevel is: {:?}", is_toplevel);
            println!("");

            drop(env_binding);
            drop(weak_binding);

            match sexpr {
            Ok(mut sexpr) => {
            println!("in eval_ds - sexpr: {:?}", sexpr);
            println!("");

                    if is_toplevel {
                    match sexpr {
                    SExpr::Atom(atom) => {
                    self.configure_with_sexpr_macro(
                    SExpr::Atom(atom))
                    }

                            SExpr::SList(ref mut slist) => {

                                let cons = slist.slist_to_cons();

                                println!("cons: {:?}", cons);
                                println!("");

                                let sform = slist.slist_to_sform();
                                println!("sform: {:?}", sform);
                                println!("");

                                self.configure_with_sexpr_macro(cons);

                            }
                            _ => unreachable!(),
                            }
                            } else {
                            self.configure_with_sexpr_macro(sexpr);
                            }
                            }

                Err(err) => {
                self.error = Some(err);
                }
                }

            match self.eval() {
            Ok(res)  => {
            self.set_macro_mode(false);

                   return Ok(res);
                   }

               Err(err) => {
               if let Some(self_err) = &self.error {

                       return Err(self_err.clone());
                       } else {
                       return Err(err);
                       }
                       }
                       }
                       } else {
                       return Err(RLError::SimpleProgramError);
                       }
    
      */



     ///////////////////////////////////////////////////////////
     /*
     Utility functions
      */
     ///////////////////////////////////////////////////////////

     pub fn get_sexpr(&self) -> &SExpr {
        &self.sexpr
    }

    pub fn set_sexpr(&mut self, sexpr: SExpr) {
        self.sexpr = sexpr.clone();
    }

    pub fn reset(&mut self) {
        self.sexpr = SExpr::Nil(RLNil::new());

        self.parser.reset();

        // realize env binding
        let rc_binding = self.env.clone();

        let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
            Rc::downgrade(&rc_binding);

        let binding = weak_binding.upgrade().unwrap();
        drop(rc_binding);

        let mut env_binding = binding.borrow_mut();

        env_binding.block_chain_clear();

        drop(env_binding);
        drop(weak_binding);
    }

    pub fn update_symbols(&mut self) {
        println!("begin update_symbols");
        println!("");

        // realize env binding to environment
        let rc_binding = self.env.clone();

        let weak_binding: std::rc::Weak<RefCell<dyn EnvTrait>> =
            Rc::downgrade(&rc_binding);

        let binding = weak_binding.upgrade().unwrap();
        drop(rc_binding);

        let mut env_binding = binding.borrow_mut();

        let curr_pack = env_binding.get_mut_current_package();

        let curr_hash_map = curr_pack.get_symbols_hash_map();

        for key in curr_hash_map.keys() {
            self.parser.lexer.add_symbol(&key.to_lowercase());
        }

        // self.parser.lexer.show_all_symbols();

        println!("end update_symbols");
        println!("");
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
