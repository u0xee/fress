// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! A cohesive fressian library for rust

pub mod dispatch;
pub mod fuzz;
pub mod keyword;
pub mod list;
//pub mod map;
pub mod memory;
pub mod rational;
pub mod set;
pub mod sorted_map;
pub mod sorted_set;
pub mod string;
pub mod transducer;
pub mod symbol;
pub mod value;
pub mod vector;

pub use value::Value;

#[cfg(test)]
mod tests {
    use super::*;

}
