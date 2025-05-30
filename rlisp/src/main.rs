// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::io::{BufRead, stdin, stdout, Write};

// use eval::evaluator::{RLEvaluator, downcast_result};
use eval::evaluator::RLEvaluator;

fn show_repl_intro() {
    print!("RLisp> ");
    stdout()
        .flush()
        .expect("Failed to flush");
}

fn main() {
    // let mut parser = RLParser::new();
    // let mut rl = RLEvaluator::new_with_parser(&parser);
    let mut rl = RLEvaluator::new();
    rl.init();

    println!("RLisp Version 0.0.1");
    println!("Press Crtl+c to exit");
    println!("");

    show_repl_intro();

    for line in stdin().lock().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            show_repl_intro();
        } else {
            // let parser = RLParser::new();
            // &parser.parse_silent(&line);
            rl.update_symbols();
            rl.reset();
            rl.parser.parse_silent(&line);
            rl.configure();

            // let mut rl = RLEvaluator::new_with_parser(&parser);
            // rl.init();

            /*
            let mut sexpr: SExpr = SExpr::Nil;
            let mut parse_err: Option<RLError> = None;

            match parser.parse(&line) {
                Ok(expr) => sexpr = expr,
                Err(err) => parse_err = Some(err),
            };

            // println!("{}", sexpr);

            if let Some(ref err) = parse_err {
                eprintln!("{}", err);
                } else {
                    let mut rl = RLEvaluator::new_with_sexpr(&sexpr);
                    // println!("{:?}", rl.get_sexpr());
            */

            // continue with rl.eval()
            println!("main -> sexpr is: {:?}", rl.get_sexpr());

            match rl.eval() {
                Ok(res) => println!("{}", res),
                Err(err) => eprintln!("{}", err),
            };

            // }
            show_repl_intro();
        } // else
    } // for
} // main
