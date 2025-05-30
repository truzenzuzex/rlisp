// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::HashMap;
use std::collections::LinkedList;

use crate::param_creator::RLParamCreator;
use crate::param_generator::{RLParamGenerator,
                             make_param_gens_hash_map};

use err::err::{RLError, ParseError};

use expr::atom::RLAtom;
use expr::list::RLList;
use expr::nil::RLNil;
use expr::sexpr::SExpr;
use expr::symb::RLSymbol;

use hash::hash::RLHash;

use lex::lexer::RLLexer;

use pars_symb::symbol::Symbol;
use pars_symb::token::Token;

pub struct RLParser {
    sexpr: SExpr,

    tokens: Vec<Token>,

    pub lexer: RLLexer,

    param_creator: RLParamCreator,

    param_gens: HashMap<String, RLParamGenerator, RLHash>,

    error: Option<RLError>,
}

impl RLParser {
    pub fn new() -> Self {
        let sexpr = SExpr::Nil(RLNil::new());

        let tokens = Vec::<Token>::new();

        let lexer = RLLexer::new();

        let param_creator = RLParamCreator::new();

        let param_gens = make_param_gens_hash_map();

        let error = None;

        RLParser { sexpr,
                   tokens,
                   lexer,
                   param_creator,
                   param_gens,
                   error,
        }
    }

    pub fn init(&mut self) {
        self.lexer.init();
        self.param_creator.init(&mut self.param_gens);
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    #[allow(unused_assignments)]
    pub fn parse_to_sexpr(&mut self) -> Result<SExpr, RLError> {

        println!("parse_to_sexpr - start");
        println!("");

        self.error = None;

        /*
        println!("begin - lexer:");
        self.lexer.show_all_symbols();
        println!("");
        */

        self.tokens = self.lexer.get_tokens();

        println!("self.lexer.tokens: {:?}", self.lexer.tokens);
        println!("");

        let mut lhs = match self.lexer.next() {
            Token::Atom(atom) => {
                match &*atom {
                    "()"  | "nil" |
                    "NIL" | "Nil" |
                    "NIl" | "NiL" |
                    "niL" | "nIL" |
                    "nIl" => SExpr::Nil(RLNil::new()),

                    _ => SExpr::Atom(RLAtom::new(&atom)),
                }
            }

            Token::Symb(symb) => {
                match &*symb.name {
                    "(" => SExpr::Dummy,

                    ")" => {
                        let err = ParseError::new(
                            "First", pars_symb::token::Token::Symb(symb));
                          return Err(RLError::ParseError(err))
                    }

                    _ => {
                        SExpr::Symb(RLSymbol::new_with_symb(&symb))
                    }
                }
            }

            Token::Eof => {
                let err = ParseError::new("First", pars_symb::token::Token::Eof);
                return Err(RLError::ParseError(err))
            }
        };

        loop {
            let sym = match self.lexer.peek() {
                Token::Eof => break,
                Token::Symb(sym) => sym,
                // Token::Atom(atom) => break,
                t => {
                    let err = ParseError::new("Symbol", t);
                    return Err(RLError::ParseError(err))
                }
            };

            let mut list: LinkedList<SExpr> = LinkedList::new();

            // lexer.next(); // upon symbol

            match self.lexer.next() {  // upon symbol
                Token::Symb(symb) => {
                    match &*symb.name {
                        "block" => {
                            println!("parser: in block");

                            let param_gen: &mut RLParamGenerator =
                                &mut self.param_gens.get("block")
                                                    .unwrap()
                                                    .clone();

                            match param_gen.run_closure(&mut self.lexer) {
                                Ok(ll) => {
                                    list = param_gen.build_param_list(ll);

                                    println!("block-list: {:?}", list);
                                    println!("");
                                }

                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        }

                        "backquote" => {
                            println!("parser: in backquote");

                            let param_gen: &mut RLParamGenerator =
                                &mut self.param_gens.get("backquote")
                                                    .unwrap()
                                                    .clone();

                            match param_gen.run_closure(&mut self.lexer) {
                                Ok(ll) => {
                                    list = param_gen.build_param_list(ll);
                                }

                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        }

                        "concatenate" => {
                            println!("parser: in concatenate");

                            /*
                            let param_gen: &mut RLParamGenerator =
                                &mut self.param_gens.get("concatenate")
                                                    .unwrap()
                                                    .clone();

                            match param_gen.run_closure(&mut self.lexer) {
                                Ok(_linked_list) => {
                                    // list = linked_list;
                                    list = param_gen.build_param_list();
                                }

                                Err(err) => {
                                    return Err(err);
                                }
                            }
                            */
                        }

                        "defun" => {
                            println!("parser: in defun");

                            let param_gen: &mut RLParamGenerator =
                                &mut self.param_gens.get("defun")
                                                    .unwrap()
                                                    .clone();

                            match param_gen.run_closure(&mut self.lexer) {
                                Ok(ll) => {
                                    list = param_gen.build_param_list(ll);
                                }

                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        }

                        "defmacro" => {
                            println!("parser: in defmacro");

                            let param_gen: &mut RLParamGenerator =
                                &mut self.param_gens.get("defmacro")
                                                    .unwrap()
                                                    .clone();

                            match param_gen.run_closure(&mut self.lexer) {
                                Ok(ll) => {
                                    list = param_gen.build_param_list(ll);
                                }

                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        }

                        "defparameter" => {
                            println!("parser: in defparameter");
                        }

                        "defvar" => {
                            println!("parser: in defvar");
                        }

                        "eval" => {
                            println!("parser: in eval");

                            /*
                            let param_gen: &mut RLParamGenerator =
                                &mut self.param_gens.get("eval")
                                                    .unwrap()
                                                    .clone();

                            match param_gen.run_closure(&mut self.lexer) {
                                Ok(ll) => {
                                    list = param_gen.build_param_list(ll);
                                }

                                Err(err) => {
                                    return Err(err);
                                }
                            }
                            */
                        }

                        "function" => {
                            println!("parser: in function");

                            /*
                            let param_gen: &mut RLParamGenerator =
                                &mut self.param_gens.get("function")
                                                    .unwrap()
                                                    .clone();

                            match param_gen.run_closure(&mut self.lexer) {
                                Ok(ll) => {
                                    list = param_gen.build_param_list(ll);
                                }

                                Err(err) => {
                                    return Err(err);
                                }
                            }
                            */
                        }

                        "lambda" => {
                            println!("parser: in lambda");

                            let param_gen: &mut RLParamGenerator =
                                &mut self.param_gens.get("lambda")
                                                    .unwrap()
                                                    .clone();

                            match param_gen.run_closure(&mut self.lexer) {
                                Ok(ll) => {
                                    list = param_gen.build_param_list(ll);
                                }

                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        }

                        "progv" => {
                            println!("parser: in progv");

                            let param_gen: &mut RLParamGenerator =
                                &mut self.param_gens.get("progv")
                                                    .unwrap()
                                                    .clone();

                            match param_gen.run_closure(&mut self.lexer) {
                                Ok(ll) => {
                                    list = param_gen.build_param_list(ll);
                                }

                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        }

                        "quote" => {
                            println!("parser: in quote");

                            let param_gen: &mut RLParamGenerator =
                                &mut self.param_gens.get("quote")
                                                    .unwrap()
                                                    .clone();

                            match param_gen.run_closure(&mut self.lexer) {
                                Ok(ll) => {
                                    list = param_gen.build_param_list(ll);
                                }

                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        }

                        "return-from" => {
                            println!("parser: in return-from");

                            let param_gen: &mut RLParamGenerator =
                                &mut self.param_gens.get("return-from")
                                                    .unwrap()
                                                    .clone();

                            match param_gen.run_closure(&mut self.lexer) {
                                Ok(ll) => {
                                    list = param_gen.build_param_list(ll);
                                }

                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        }

                        _ => {}
                   } // match &*symb.symbol {
                } // Token::Symb(symb)

                _ => { }
            }; // match self.lexer.next()

            println!("after qlist loop");
            println!("");

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

            loop {
                println!("in inner loop...");

                let mut paren_count = 1;

                match self.lexer.next() {
                    Token::Atom(atom) => {
                        println!("loop after qlist, option Atom...");
                        match &*atom {
                            "()"  | "nil" |
                            "NIL" | "Nil" |
                            "NIl" | "NiL" |
                            "niL" | "nIL" |
                            "nIl" => {
                                list.push_back(SExpr::Nil(RLNil::new()));
                            }

                            _ => {
                                list.push_back(
                                    SExpr::Atom(RLAtom::new(&atom)));
                            }
                        }
                    }

                    Token::Symb(symb) => {
                        match &*symb.name {
                            "(" => {
                                println!("loop after qlist, option (");
                                println!("");

                                paren_count = paren_count + 1;

                                self.lexer.restore_token();

                                let inner_sexpr = self.parse_to_sexpr();

                                match inner_sexpr {
                                    Ok(_) => {
                                        println!("INNER_SEXPR: {:?}",
                                            inner_sexpr);
                                        println!("");

                                        list.push_back(inner_sexpr.unwrap())
                                    }

                                    Err(err) => {
                                        return Err(err);
                                    }
                                }
                            }

                            ")" => {
                               println!("loop after qlist, token::Symb, matched )...");
                               self.lexer.restore_token();
                               break;
                            }

                            _ => {
                                println!("loop after qlist, option ( _ =>");
                                println!("{}", &*symb.name);
                                println!("");

                                list.push_back(
                                    SExpr::Symb(RLSymbol::new_with_symb(
                                        &Symbol {name: symb.name.clone() })));

                                /*
                                let err = ParseError::new("Inner",
                                    pars_symb::token::Token::Symb(symb));
                                return Err(RLError::ParseError(err))
                                */
                            }
                        }
                    }

                    Token::Eof => {
                        println!("in inner loop, option t...");

                        self.lexer.restore_token();
                        match self.lexer.peek() {
                            /*
                            Token::Symb(Symbol::ParenRight) => { break }
                             _ => {
                                 let err = ParseError::new("Inner", t);
                                 return Err(RLispError::ParseError(err))
                             }
                             */

                             Token::Symb(symb) => {
                                 match &*symb.name {
                                     ")" =>  {
                                        println!("loop after qlist, option t, matched )...");
                                        println!("");

                                        break;
                                     }

                                     _ => {
                                         let err = ParseError::new("Inner",
                                         pars_symb::token::Token::Symb(symb));
                                         return Err(RLError::ParseError(err))
                                     }
                                 }
                             }

                             Token::Eof => {
                                 println!("EOF-EOF");
                                 println!("");

                                 /*
                                 let err = ParseError::new("Inner",
                                     pars_symb::token::Token::Eof);
                                 return Err(RLError::ParseError(err))
                                 */

                                 break;
                             }

                             t => {
                                 println!("inner loop, option t _ =>");

                                 let err = ParseError::new("Inner", t);
                                 return Err(RLError::ParseError(err))
                             }
                        };
                    }
                };
            } // inner loop

            println!("left inner loop...");
            println!("");

            println!("sym: {:?}", sym);
            println!("");
            println!("list: {:?}", list);
            println!("");

            if list.is_empty() {
                let mut lhs_list = LinkedList::<SExpr>::new();

                lhs_list.push_front(
                    SExpr::Symb(RLSymbol::new_with_str(")")));

                lhs_list.push_front(
                    SExpr::Symb(RLSymbol::new_with_symb(&sym)));

                lhs_list.push_front(
                    SExpr::Symb(RLSymbol::new_with_str("(")));

                lhs = SExpr::SList(RLList::<SExpr>::new_with_list(&lhs_list));
            } else {
                lhs = SExpr::Cons(sym, list.clone());
            }

            // lhs = SExpr::Cons(sym, list.clone());

            println!("lhs is: {}", lhs);
            println!("");

            match self.lexer.next() {
                Token::Eof => break,

                Token::Symb(symb) => {
                    match &*symb.name {
                        ")" => break,
                          _ => {
                              let err = ParseError::new("Last",
                                  pars_symb::token::Token::Symb(symb));
                              return Err(RLError::ParseError(err))
                        }
                    }
                }

                Token::Atom(atom) => {
                    println!("last - option t");
                    let err = ParseError::new("Last",
                        pars_symb::token::Token::Atom(atom));
                    return Err(RLError::ParseError(err))
                }
            };
        } // outer loop

        println!("second lhs is: {:?}", lhs);
        println!("");

        println!("parse_to_sexpr - end");
        println!("");

        Ok(lhs)
    }

    pub fn parse(&mut self, input: &str) -> Result<SExpr, RLError> {
        self.lexer.parse(input);

        self.parse_to_sexpr()
    }

    pub fn parse_silent(&mut self, input: &str) {
        self.lexer.parse(input);

        let parse_result = self.parse_to_sexpr();

        println!("parse_silent - self.error: {:?}", self.error);

        match parse_result {
            Ok(res)  => self.sexpr = res,
            Err(err) => self.error = Some(err),
        }
    }

    pub fn set_sexpr(&mut self, sexpr: SExpr) {
        self.sexpr = sexpr.clone()
    }

    pub fn get_sexpr(&self) -> SExpr {
        self.sexpr.clone()
    }

    pub fn set_error(&mut self, error: RLError) {
        self.error = Some(error)
    }

    pub fn get_error(&self) -> Option<RLError> {
        self.error.clone()
    }

    pub fn reset(&mut self) {
        self.sexpr = SExpr::Nil(RLNil::new());
    }

    pub fn change_tokens(&mut self, tokens: &Vec<Token>) {
        self.lexer.change_tokens(tokens)
    }
}

////////////////////////////////////////////////////////////
