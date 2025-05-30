// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::hash_map::HashMap;

use crate::data_control_params::DataControlParams;
use crate::eval_comp_params::EvalCompParams;
use crate::list_params::ListParams;
use crate::param_generator::RLParamGenerator;
use crate::string_params::StringParams;


use hash::hash::RLHash;

pub struct RLParamCreator {

    data_control_params: DataControlParams,
    eval_comp_params: EvalCompParams,
    list_params: ListParams,
    string_params: StringParams,
}

impl RLParamCreator {
    pub fn new() -> RLParamCreator {
        let data_control_params = DataControlParams::new();

        let eval_comp_params = EvalCompParams::new();

        let list_params = ListParams::new();

        let string_params = StringParams::new();

        // let param_gens_hash = make_param_gens_hash_map();

        RLParamCreator {
            data_control_params,
            eval_comp_params,
            list_params,
            string_params,
            // param_gens_hash,
        }
    }

    pub fn init(&mut self,
                param_gens: &mut HashMap<String, RLParamGenerator, RLHash>) {
        self.data_control_params.init(param_gens);
        self.eval_comp_params.init(param_gens);
        self.list_params.init(param_gens);
        self.string_params.init(param_gens);
    }
}
