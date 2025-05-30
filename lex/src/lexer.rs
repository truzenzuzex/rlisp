// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::borrow::Cow;
use std::collections::HashMap;

use crate::symbol_creator::SymbolCreator;

use hash::hash::{RLHash, clone_hash_map};

use pars_symb::symbol::Symbol;
use pars_symb::token::{Token, make_token_hash_map};

use regex::Regex;

// #[derive(Debug)]
pub struct RLLexer {
    pub restored_token: Option<Token>,

    symbols: HashMap<String, Token, RLHash>,

    symbol_creator: SymbolCreator,

    pub tokens: Vec<Token>,
}

impl RLLexer {
    pub fn new() -> RLLexer {
        let restored_token = None;

        let symbols = make_token_hash_map();

        let symbol_creator = SymbolCreator::new();

        let tokens = Vec::<Token>::new();

        RLLexer { restored_token,
                  symbols,
                  symbol_creator,
                  tokens,
        }
    }

    pub fn init(&mut self) {
        self.symbol_creator.init(&mut self.symbols);

        /*
        self.symbols.insert("print".to_string(),
                            Token::Symb(Symbol::new("print")));
        */
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn add_symbol(&mut self, value: &String) {
        self.symbols.insert(value.to_string(),
                            Token::Symb(Symbol::new(value)));
    }

    pub fn delete_symbol(&mut self, key: &String) {
        self.symbols.remove(key);
    }

    pub fn show_all_symbols(&self) {
        for key in self.symbols.keys() {
            println!("{key}");
        }
    }

    pub fn parse(&mut self, input: &str) {
        // let restored_token = None;

        let mut strings: Vec<&str> = vec![];

        let mod_input: Cow<'_, str> = Cow::from(input);

        let quote1_re = Regex::new(r#"".+?""#).unwrap();

        for caps in quote1_re.captures_iter(&mod_input) {
            let s_val = caps.get(0).unwrap().as_str();
            strings.push(s_val);
        }

        strings.reverse();

        println!("lexer - strings: {:?}", strings);
        println!("");

        //
        let mod_input = quote1_re.replace_all(&mod_input, "\"\" dummy");
        println!("quote1_re: {}", mod_input);
        println!("");

        // #' function designator I
        let sharp_quote_func = Regex::new(r#"#'(?P<sqf>[^\(\s]+)"#).unwrap();
        let mod_input =
            sharp_quote_func.replace_all(&mod_input, " (function $sqf) ");
        println!("sharp_quote_func #'nvknfkef: {}", mod_input);
        println!("");

        // #' function designator II
        let sharp_quote_lambda =
            Regex::new(r#"#'(?P<sql>.?\(([^()]*|\(([^()]*|\(([^()]*|\(([^()]*|\(([^()]*|\([^()]*\))*\))*\))*\))*\))*\))"#).unwrap();
        let mod_input =
            sharp_quote_lambda.replace_all(&mod_input, " (function $sql) ");
        println!("sharp_quote_lambda #'(lambda () (+ )): {}", mod_input);
        println!("");

        // `(+ 1 2)
        let backquote_paren =
            Regex::new(r#"`(?P<bq_paren>.?\(([^()]*|\(([^()]*|\(([^()]*|\(([^()]*|\(([^()]*|\([^()]*\))*\))*\))*\))*\))*\))"#).unwrap();

        let mut matches_bq_paren: Vec<_> =
            backquote_paren.find_iter(&mod_input)
                           .map(|m| m.as_str())
                           .collect();

        let mod_input =
            backquote_paren.replace_all(&mod_input, "backquote_paren_pushed");

        /*
        for _n in 0..matches_bq_paren.len() {
            if let Some(item) = matches_bq_atom.pop() {
                println!("item: {:?}", item);
                println!("");
            }
        }
        */

        // `nvknfkef
        let backquote_atom = Regex::new(r#"`(?P<bq_atom>[^\(\s]+)"#).unwrap();

        let mut matches_bq_atom: Vec<_> =
            backquote_atom.find_iter(&mod_input)
                          .map(|m| m.as_str())
                          .collect();

        /*
        for _n in 0..matches_bq_atom.len() {
            if let Some(item) = matches_bq_atom.pop() {
                println!("item: {:?}", item);
                println!("");
            }
        }
        */

        let mod_input =
            backquote_atom.replace_all(&mod_input, "backquote_atom_pushed");

        /*
        let mod_input =
            backquote_atom.replace_all(&mod_input, " (backquote $q) ");

        println!("backquote_atom `nvknfkef: {}", mod_input);
        println!("");
        */

        let quote_paren =
            Regex::new(r#"'(?P<q_paren>.?\(([^()]*|\(([^()]*|\(([^()]*|\(([^()]*|\(([^()]*|\([^()]*\))*\))*\))*\))*\))*\))"#).unwrap();

        // let is_quote_paren = quote_paren.is_match(&mod_input);

        let mut mod_input =
            quote_paren.replace_all(&mod_input, " (quote $q_paren) ");
        println!("quote_paren '(()): {}", mod_input);
        println!("");

        // 'nvknfkef
        // let quote_atom = Regex::new(r#"'(?P<x>[^\(]+)"#).unwrap();

        let quote_atom = Regex::new(r#"'(?P<x>[^\(\s]+)"#).unwrap();

        let mod_input_quote_atom = mod_input.clone();

        /*
        if is_quote_paren {
            mod_input = quote_atom.replace_all(
                &mod_input_quote_atom, " ' $x ");
        } else {
        */
            mod_input = quote_atom.replace_all(
                &mod_input_quote_atom, " (quote $x) ");
        // }

        println!("quote_atom 'nvknfkef: {}", mod_input);
        println!("");



        let backquote_paren_pushed =
            Regex::new(r#"backquote_paren_pushed"#).unwrap();

        matches_bq_paren.reverse();

        let mod_input_for = mod_input.clone();

        let backquote_for = Regex::new(r#"`(?P<bq_for>.+?)"#).unwrap();
        let quote_for = Regex::new(r#"'(?P<q_for>.+?)"#).unwrap();
        let comma_for = Regex::new(r#",(?P<c_for>.+?)"#).unwrap();

        for _n in 0..matches_bq_paren.len() {
            if let Some(bq) = matches_bq_paren.pop() {
                println!("bq: {:?}", bq);
                println!("");

                let zero = backquote_paren.replace_all(&bq, "(backquote $bq_paren)");
                let first = backquote_for.replace_all(&zero, "` $bq_for");
                let second = quote_for.replace_all(&first, "' $q_for");
                let third = comma_for.replace_all(&second, ", $c_for");

                mod_input = backquote_paren_pushed.replace(
                    &mod_input_for, third);

                println!("mod_input in for: {:?}", mod_input);
                println!("");
            }
        }

        println!("backquote_paren_pushed: {}", mod_input);
        println!("");



        let backquote_atom_pushed =
            Regex::new(r#"backquote_atom_pushed"#).unwrap();

        matches_bq_atom.reverse();

        let mod_input_for2 = mod_input.clone();

        let backquote_for2 = Regex::new(r#"`(?P<bq_for2>.+?)"#).unwrap();
        let quote_for2 = Regex::new(r#"'(?P<q_for2>.+?)"#).unwrap();
        let comma_for2 = Regex::new(r#",(?P<c_for2>.+?)"#).unwrap();

        for _n in 0..matches_bq_atom.len() {
            if let Some(bq) = matches_bq_atom.pop() {
                println!("bq: {:?}", bq);
                println!("");

                let zero = backquote_atom.replace_all(&bq, "(backquote $bq_atom)");
                let first = backquote_for2.replace_all(&zero, "` $bq_for2");
                let second = quote_for2.replace_all(&first, "' $q_for2");
                let third = comma_for2.replace_all(&second, ", $c_for2");

                mod_input = backquote_atom_pushed.replace(
                    &mod_input_for2, third);
            }
        }

        println!("backquote_atom_pushed: {}", mod_input);
        println!("");



        let whitespace_paren_right = Regex::new(r#"(?P<y>.+?)\)"#).unwrap();
        let mod_input =
            whitespace_paren_right.replace_all(&mod_input, "$y ) ");
        println!("insert whitespace paren right: {:?}", mod_input);
        println!("");

        let whitespace_paren_left = Regex::new(r#"\((?P<z>.+?)"#).unwrap();
        let mod_input =
            whitespace_paren_left.replace_all(&mod_input, " ( $z");
        println!("insert whitespace paren left: {:?}", mod_input);
        println!("");

        let whitespace_paren_left2 = Regex::new(r#"\((?P<a>.+?)"#).unwrap();
        let mod_input =
            whitespace_paren_left2.replace_all(&mod_input, " ( $a");
        println!("insert whitespace paren left2: {:?}", mod_input);
        println!("");

        /*
        let nil_re    = Regex::new(r#"\((?P<n> +?)\)"#).unwrap();
        let mod_input = nil_re.replace_all(&mod_input, "()");
        println!("reduce whitespaces in nil: {}", mod_input);
        println!("");
        */

        let comma_at = Regex::new(r#", +@"#).unwrap();
        let mod_input =
            comma_at.replace_all(&mod_input, " ,@ ");
        println!("comma_at: {}", mod_input);
        println!("");

        println!("mod_input after regex: {:?}", mod_input);
        println!("");

        let mut hidden_token: Option<&str> = None;

        let symbols = clone_hash_map(&self.symbols);

        // let mut tokens = mod_input
        self.tokens = mod_input
            .split_ascii_whitespace()
            .map(move |c| if let Some(x) = hidden_token
                         { println!("lexer - hidden_token {:?}", x);
                           println!("");

                           hidden_token = None;
                           match x {
                           _ => Token::Atom(String::from(x)) }
                         } else if let Some(token) = symbols.get(c) {
                             token.clone()
                         } else {
                             match c {

                                 _ if c.starts_with("\"") => {
                                     hidden_token = strings.pop();
                                     Token::Symb(Symbol::new("nil"))
                                 }

                                 // _ if c.contains("dummy") =>
                                 // Token::Sym(Symbol::Nil),

                                 _ => {
                                     // println!("lexer - c: {:?}", c);
                                     // println!("");
                                 Token::Atom(String::from(c))
                                 }
                             }
                         }).collect::<Vec<_>>();

            self.tokens.reverse();
            // tokens.retain(|x| *x != Token::Symb(Symbol::Nil));
            self.tokens.retain(|x| *x !=
                Token::Symb(Symbol::new("nil")));

            println!("tokens: {:?}", self.tokens);
            println!("");

            // self.quote_tokens = self.tokens.clone();
    }

    /*
    fn reset(&mut self) -> Token {
        let token = self.tokens
                    .last()
                    .cloned()
                    .unwrap_or(Token::Eof);

        self.tokens.pop();

        self.restored_token = Some(token);

        self.restored_token.clone().unwrap()
    }

    fn reset2(&mut self) -> Token {
        self.restored_token = None;

        self.tokens
            .last()
            .cloned()
            .unwrap_or(Token::Eof)
    }
    */

    pub fn next(&mut self) -> Token {
        let token = self.tokens
                        .pop()
                        .unwrap_or(Token::Eof);

        self.restored_token = Some(token);

        self.restored_token.clone().unwrap()
    }

    pub fn peek(&mut self) -> Token {
        self.tokens
            .last()
            .cloned()
            // .copied()
            .unwrap_or(Token::Eof)
    }

    pub fn restore_token(&mut self) {
        self.tokens.push(self.restored_token.clone().unwrap())
    }

    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(token)
    }

    /*
    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
    */

    pub fn change_tokens(&mut self, tokens: &Vec<Token>) {
        self.tokens = tokens.clone()
    }
}





















        /*
        // backquote related
        // `,(+ 2 3)
        let backquote_comma_paren =
            Regex::new(r#"`,(?P<p>\(.+?\))"#).unwrap();
        let mod_input =
            backquote_comma_paren.replace_all(&mod_input, " (backquote , $p) ");
        println!("backquote_comma_paren `(+ 2 3): {}", mod_input);
        println!("");


        // `(+ 2 3)
        let backquote_paren =
            Regex::new(r#"`(?P<p>\(.+?\))"#).unwrap();

        let mod_input = backquote_paren.replace_all(
            &mod_input, " (backquote $p) ");

        // self.backquotes.push(mod_input.to_string());

        println!("backquote_paren `(+ 2 3): {}", mod_input);
        println!("");


        // `()
        let backquote_nil =
            Regex::new(r#"`(\(\))"#).unwrap();
        let mod_input =
            backquote_nil.replace_all(&mod_input, " (backquote ()) ");

        println!("backquote nil: `(): {}", mod_input);
        println!("");
        */




        /*
        // quote related
        // '(+ 2 (- 4 3))
        let qexpr2_re =
            Regex::new(r#"'(?P<q>\(.+?([^\)]+).+?\))"#).unwrap();
        let mod_input =
            qexpr2_re.replace_all(&mod_input, " (quote $q) ");

        // '(+ 2 (- 4 3) 5)
        let qexpr3_re =
            Regex::new(r#"'(?P<r>\(.+?([^\)]+).+?\).+\))"#).unwrap();
        let mod_input =
            qexpr3_re.replace_all(&mod_input, " (quote $r) ");
        */





        /*
        // Comma related
        // ,',nvknfkef
        let comma_quote_comma = Regex::new(r#",(?P<s>[^\(\s]+)"#).unwrap();
        let mod_input =
            comma_quote_comma.replace_all(
                &mod_input, " (comma /COMMA/QUOTE/COMMA/ $s) ");
        println!("comma_quote_comma ,',nvknfkef: {}", mod_input);
        println!("");
        */

        /*
        // ,@nvknfkef
        // let comma_at = Regex::new(r#",@(?P<s>[^\(\s]+)"#).unwrap();
        let comma_at = Regex::new(r#",@(?P<s>[^\(\s]+)"#).unwrap();
        let mod_input =
            comma_at.replace_all(&mod_input, " ,@ $s) ");
        println!("comma_at 'nvknfkef: {}", mod_input);
        println!("");
        */

        /*
        // ,(+ 2 3)
        let comma_paren =
            Regex::new(r#",(?P<r>\(.+?\))"#).unwrap();
        let mod_input =
            comma_paren.replace_all(&mod_input, " (comma ,$r) ");
        println!("comma_paren ,(+ 2 3): {}", mod_input);
        println!("");
        */

        /*
        // ,nvknfkef
        let comma = Regex::new(r#",(?P<t>[^\(\s]+)"#).unwrap();
        // let comma = Regex::new(r#",,(?P<t>[^\(\s]+)"#).unwrap();
        let mod_input =
            comma.replace_all(&mod_input, " , $t ");
        println!("comma ,nvknfkef: {}", mod_input);
        println!("");
        */


             /*
        // '()
        let quote_nil =
            Regex::new(r#"'(\(\))"#).unwrap();
        let mod_input =
            quote_nil.replace_all(&mod_input, " (quote ()) ");
        println!("quote_nil '(): {}", mod_input);
        println!("");

        // '((list 1 2) (list 3 4))
        let quote_paren1 =
            Regex::new(r#"(?<start_quote>'\()(?<middle>\(([^()]|(R))*\))(?<last>.+\))"#).unwrap();

        let mod_input =
            quote_paren1.replace(&mod_input, "(quote ($middle $last)");
        println!("quote_paren1 '((list 1 2) (list 3 4)): {:?}", mod_input);
        println!("");

        // '(+ 2 3)
        // let quote_paren2 =
        //     Regex::new(r#"'(?P<v>\(.+?\))"#).unwrap();

        let quote_paren2 =
            Regex::new(r#"'(?<var>\(.+?\))|(R)"#).unwrap();

        // let mod_input =
        //     quote_paren2.replace(&mod_input, "(quote $v)");
        let mod_input =
            quote_paren2.replace_all(&mod_input, "(quote $var)");
        println!("quote_paren2 '(+ 2 3): {:?}", mod_input);
        println!("");
        */
