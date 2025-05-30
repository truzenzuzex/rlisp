// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::data_control::DataControlFuncs;
use crate::eval_comp::EvalCompilationFuncs;

use crate::pack::PackageFuncs;
use crate::symb::SymbolFuncs;

use crate::list::ListFuncs;
use crate::math::MathFuncs;
use crate::string::StrFuncs;

use env::pack::RLPackage;

pub struct CLCreator {
    rdata_controlf: DataControlFuncs,
    reval_compf: EvalCompilationFuncs,

    rpackf: PackageFuncs,
    rsymbf: SymbolFuncs,

    rlf:   ListFuncs,
    rmf:   MathFuncs,
    rstrf: StrFuncs,
}

impl CLCreator {
    pub fn new() -> CLCreator {
        let rdata_controlf = DataControlFuncs::new();
        let reval_compf = EvalCompilationFuncs::new();

        let rpackf = PackageFuncs::new();
        let rsymbf = SymbolFuncs::new();

        let rlf   = ListFuncs::new();
        let rmf   = MathFuncs::new();
        let rstrf = StrFuncs::new();

        CLCreator {
            rdata_controlf,
            reval_compf,

            rpackf,
            rsymbf,

            rlf,
            rmf,
            rstrf
        }
    }

    pub fn init(&mut self, cl_pack: &mut RLPackage) {
        let cl_pack_hash = cl_pack.get_symbols_hash_map();

        self.rdata_controlf.init(cl_pack_hash);
        self.reval_compf.init(cl_pack_hash);

        self.rpackf.init(cl_pack_hash);
        self.rsymbf.init(cl_pack_hash);

        self.rlf.init(cl_pack_hash);
        self.rmf.init(cl_pack_hash);
        self.rstrf.init(cl_pack_hash);
    }
}
