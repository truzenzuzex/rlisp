// This file is part of the rlisp package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::collections::hash_map::HashMap;

use env::symb::RLEnvSymbol;

use hash::hash::RLHash;

pub struct PackageFuncs {
}

impl PackageFuncs {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&mut self,
                _cl_pack_hash: &mut HashMap<String, RLEnvSymbol, RLHash>) {
    }
}

/*
*modules*
*package*
defpackage
do-all-symbols
do-external-symbols
do-symbols
export
find-all-symbols
find-package
find-symbol
import
in-package
intern
list-all-packages
make-package
package-name
package-nicknames
package-shadowing-symbols
package-use-list
package-used-by-list
provide
rename-package
require
shadow
shadowing-import
unexport
unintern
unuse-package
use-package
*/
