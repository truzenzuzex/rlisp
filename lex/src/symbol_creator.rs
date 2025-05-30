// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::hash_map::HashMap;

use crate::cl_symbs::CLSymbs;
use crate::data_control_symbs::DataControlSymbs;
use crate::eval_comp_symbs::EvalCompSymbs;
use crate::list_symbs::ListSymbs;
use crate::math_symbs::MathSymbs;
use crate::symb_symbs::SymbSymbs;

use hash::hash::RLHash;

use pars_symb::token::Token;

pub struct SymbolCreator {
    cl_symbs: CLSymbs,
    data_control_symbs: DataControlSymbs,
    eval_comp_symbs: EvalCompSymbs,
    list_symbs: ListSymbs,
    math_symbs: MathSymbs,
    symb_symbs: SymbSymbs,
}

impl SymbolCreator {
    pub fn new() -> SymbolCreator {
        let cl_symbs           = CLSymbs::new();
        let data_control_symbs = DataControlSymbs::new();
        let eval_comp_symbs    = EvalCompSymbs::new();
        let list_symbs         = ListSymbs::new();
        let math_symbs         = MathSymbs::new();
        let symb_symbs         = SymbSymbs::new();

        SymbolCreator {
            cl_symbs,
            data_control_symbs,
            eval_comp_symbs,
            list_symbs,
            math_symbs,
            symb_symbs,
        }
    }

    pub fn init(&mut self,
                symbols: &mut HashMap<String, Token, RLHash>) {
        self.cl_symbs.init(symbols);
        self.data_control_symbs.init(symbols);
        self.eval_comp_symbs.init(symbols);
        self.list_symbs.init(symbols);
        self.math_symbs.init(symbols);
        self.symb_symbs.init(symbols);
    }
}
