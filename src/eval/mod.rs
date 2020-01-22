// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use value::Value;
use ::{read, is_aggregate, hash_map, hash_set, list, vector, nil, tru, fals};
use right_into;
use transduce::Process;

pub mod structure;
pub mod compile;

// eval global context map!
pub fn eval(v: Value) -> Value {
    let (structured, notes) = structure::structure(v).expect("Error during structure");
    vector().conj(structured).conj(notes)
}

