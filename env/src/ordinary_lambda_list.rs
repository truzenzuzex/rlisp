// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::fmt;
use std::ops::Not;

use expr::list::RLList;
use expr::sexpr::SExpr;
use expr::symb::RLSymbol;

use err::err::RLError;

#[derive(Debug, Clone)]
pub struct RLOrdinaryLambdaList {
    // pub required_params: RLList<SExpr>,
    pub required_params: SExpr,

    // possible lambda-list keywords
    /*
    &allow-other-keys
    &key
    &rest
    &aux
    &optional
    */

    /*
    a_o_k_params: RLList,
    key_params: RLList,
    aux_params: RLList,
    optional_params: RLList,
    */

    // pub rest_params: RLList<SExpr>,
    pub rest_params: SExpr,
}

impl RLOrdinaryLambdaList {
    pub fn new() -> RLOrdinaryLambdaList {
        // let required_params = RLList::<SExpr>::new();
        let required_params = SExpr::SList(RLList::<SExpr>::new());

        // let rest_params  = RLList::<SExpr>::new();
        let rest_params = SExpr::SList(RLList::<SExpr>::new());

        Self {
            required_params,
            rest_params,
        }
    }

    pub fn set_required_params_with_rllist(&mut self,
                                           req_params: RLList<SExpr>) {
        self.required_params = SExpr::SList(req_params);
    }

    pub fn set_required_params_with_sexpr(&mut self, required_params: SExpr) {
        if let SExpr::SList(ref _slist) = required_params {
            self.required_params = required_params;
        }
    }

    pub fn get_req_params_as_option(&mut self) -> Option<SExpr> {
        Some(self.required_params.clone())
    }

    pub fn get_req_params_as_rlist(&self) -> RLList<SExpr> {
        if let SExpr::SList(slist) = &self.required_params {
            slist.clone()
        } else {
            RLList::<SExpr>::new()
        }
    }

    /*
    pub fn get_required_params_as_sexpr(&self) -> SExpr {
        SExpr::SList(self.required_params.clone())
    }
    */

    pub fn get_required_params(&self) -> SExpr {
        self.required_params.clone()
    }

    /*
    pub fn get_next_required_param(&mut self) -> Option<SExpr> {
        if let Some(param) = self.required_params.pop_front() {
            return Some(param);
        } else {
            return None;
        }
    }
    */


    pub fn set_rest_params_with_rllist(&mut self, rest_params: RLList<SExpr>) {
        self.rest_params = SExpr::SList(rest_params);
    }

    /*
    pub fn get_rest_params(&mut self) -> Option<SExpr> {
        if let Some(param) = self.rest_params.pop_front() {
            return Some(param);
        } else {
            return None;
        }
    }
    */

    pub fn parse_parameters(&mut self, sexpr: SExpr) -> Result<(), RLError> {
        let mut required_params = RLList::<SExpr>::new();

        let mut required_params_ll = required_params.get_linked_list();

        required_params_ll.pop_front();
        required_params_ll.pop_back();

        let mut rest_params = RLList::<SExpr>::new();

        let mut rest_params_ll = rest_params.get_linked_list();

        // let mut work_list: &mut RLList<SExpr> = &mut required_params;

        let mut work_list = &mut required_params_ll;

        println!("work_list: {:?}", work_list);
        println!("");

        println!("parameters_sexpr: {:?}", sexpr);
        println!("");

        // parameters: SList([Atom("x"), Symb(Symbol { name: "&rest" }), Atom("y")])

        match sexpr {
            SExpr::SList(slist) => {
                let mut list = slist.get_linked_list();

                if list.is_empty() {
                    self.set_required_params_with_rllist(
                        required_params);

                    println!("required_params: {:?}", self.required_params);
                    println!("");

                    return Ok(())
                }

                while list.is_empty().not() {
                    let item = list.pop_front();

                    match item {
                        Some(SExpr::Symb(symb)) => {

                            let symb_name = symb.get_symbol_name();

                            match &*symb_name {
                                /*
                                "&allow_other_keys" => {
                                }

                                "&key" => {
                                }
                                */

                                "&rest" => {
                                   work_list.push_front(
                                       SExpr::Symb(
                                           RLSymbol::new_with_str("(")));

                                    work_list.push_back(
                                        SExpr::Symb(
                                            RLSymbol::new_with_str(")")));

                                    work_list = &mut rest_params_ll;

                                    work_list.clear();
                                }

                                /*
                                "&aux" => {
                                }

                                "&optional" => {
                                }
                                */

                                &_ => unreachable!(),
                            }
                        }

                        Some(SExpr::Atom(atom)) => {
                            println!("atom: {:?}", atom);
                            println!("");

                            work_list.push_back(SExpr::Atom(atom));
                        }

                        Some(SExpr::SList(list)) =>  {
                            println!("slist: {:?}", list);
                            println!("");

                            work_list.push_back(SExpr::SList(list));
                        }

                        _ => unreachable!(),
                    }
                }
            }

            _ => unreachable!(),
        }

        work_list.push_front(SExpr::Symb(RLSymbol::new_with_str("(")));

        work_list.push_back(SExpr::Symb(RLSymbol::new_with_str(")")));

        required_params.set_linked_list(&required_params_ll);
        self.set_required_params_with_rllist(required_params);

        rest_params.set_linked_list(&rest_params_ll);
        self.set_rest_params_with_rllist(rest_params);

        println!("required_params_ll: {:?}", required_params_ll);
        println!("");

        println!("rest_params_ll: {:?}", rest_params_ll);
        println!("");

        Ok(())
    }
}

impl fmt::Display for RLOrdinaryLambdaList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*
        println!("in fmt for OrdinaryLambdaList");
        println!("self: {:?}", self);
        println!("");
        */

        match self {
           RLOrdinaryLambdaList {
               required_params: req_params,

               // possible lambda-list keywords
               /*
                  &allow-other-keys
                  &key
                  &rest
                  &aux
                  &optional
               */

               rest_params: _,
           } => {
               write!(f, "{}", req_params)
           }
        }
    }
}
