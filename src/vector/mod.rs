// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

//! Indexed array mapped trie, supporting vectors and lists.

use std::fmt;
use std::io;
use std::cmp::Ordering;
use memory::*;
use dispatch::*;
use value::*;
use handle::Handle;
use transducer::Process;

pub mod guide;
use self::guide::Guide;
pub mod conj;
use self::conj::unaliased_root;
pub mod pop;
pub mod nth;
pub mod meta;
pub mod assoc;
pub mod eq;
pub mod tear_down;
pub mod reduce;
pub mod iter;
pub mod util;
use self::util::*;
#[cfg(test)]
use fuzz;

/// Defines branching factor.
///
/// Can be 4, 5 or 6, making for sixteen, thirty-two or sixty-four way branching.
pub const BITS: u32 = 4; // one of 4, 5, 6
pub const ARITY: u32 = 1 << BITS;
pub const TAIL_CAP: u32 = ARITY;
pub const MASK: u32 = ARITY - 1;

pub static VECTOR_SENTINEL: u8 = 0;

/// Vector dispatch.
pub struct Vector {
    prism: Unit,
}

impl Vector {
    pub fn new() -> Unit {
        let guide = {
            let s = Segment::new(6);
            let prism = s.line_at(0);
            prism.set(0, mechanism::prism::<Vector>());
            let mut g = Guide::hydrate_top_bot(prism, 0, 0);
            g.is_compact_bit = 0x1;
            g
        };
        guide.store().segment().unit()
    }

    pub fn new_value() -> Value {
        Vector::new().handle().value()
    }
}

impl Dispatch for Vector {
    fn tear_down(&self, prism: AnchoredLine) {
        tear_down::tear_down(prism);
    }

    fn unaliased(&self, prism: AnchoredLine) -> Unit {
        unaliased_root(Guide::hydrate(prism)).segment().unit()
    }
}

impl Identification for Vector {
    fn type_name(&self) -> &'static str {
        "Vector"
    }

    fn type_sentinel(&self) -> *const u8 {
        (& VECTOR_SENTINEL) as *const u8
    }
}

impl Distinguish for Vector {
    fn hash(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        if guide.has_hash() {
            return guide.hash;
        }
        use random::{PI, cycle_abc};
        struct Pointer {
            pub ptr: *mut u64,
        }
        impl Process for Pointer {
            fn inges(&mut self, stack: &mut [Box<Process>], v: &Value) -> Option<Value> {
                let h = v.hash() as u64;
                unsafe {
                    *self.ptr = cycle_abc(34, *self.ptr + h);
                }
                None
            }
            fn last_call(&mut self, stack: &mut [Box<Process>]) -> Value {
                Handle::nil().value()
            }
        }

        let mut y = cycle_abc(7, PI[321] + guide.count as u64);
        let mut procs: [Box<Process>; 1] = [Box::new(Pointer { ptr: (&mut y) as *mut u64 })];
        let _ = reduce::reduce(prism, &mut procs);
        let h = cycle_abc(210, y) as u32;
        guide.set_hash(h).store().hash
    }
    fn eq(&self, prism: AnchoredLine, other: Unit) -> bool {
        // basic checks
        // if immediate, false
        // if vector, structural compare
        // if list, iterate pairwise
        // if eduction, iterate pairwise
        // compare structurally down tree
        // like tandem tear_down's
        unimplemented!()
    }
    fn cmp(&self, prism: AnchoredLine, other: Unit) -> Ordering {
        // cast other to vector, compare pairwise
        unimplemented!()
    }
}

impl Aggregate for Vector {
    fn count(&self, prism: AnchoredLine) -> u32 {
        let guide = Guide::hydrate(prism);
        guide.count
    }
    fn empty(&self, prism: AnchoredLine) -> Unit {
        Vector::new()
    }
    fn conj(&self, prism: AnchoredLine, x: Unit) -> Unit {
        conj::conj(prism, x)
    }
    fn meta(&self, prism: AnchoredLine) -> *const Unit {
        meta::meta(prism)
    }
    fn with_meta(&self, prism: AnchoredLine, m: Unit) -> Unit {
        meta::with_meta(prism, m)
    }
    fn pop(&self, prism: AnchoredLine) -> (Unit, Unit) {
        pop::pop(prism)
    }
}

impl Sequential for Vector {
    fn nth(&self, prism: AnchoredLine, idx: u32) -> *const Unit {
        nth::nth(prism, idx).line().star()
    }
}

fn key_into_idx(k: Unit) -> u32 {
    // TODO need general conversion to int
    let i: u32 = k.into();
    i >> 4
}

impl Associative for Vector {
    fn assoc(&self, prism: AnchoredLine, k: Unit, v: Unit) -> (Unit, Unit) {
        let idx: u32 = key_into_idx(k);
        assoc::assoc(prism, idx, v)
    }
}

impl Reversible for Vector {}
impl Sorted for Vector {}
impl Notation for Vector {
    fn debug(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        let guide= Guide::hydrate(prism);
        let hash = if !guide.has_hash() { "".to_string() } else {
            format!(" #x{:X}", guide.hash)
        };
        let meta = if !guide.has_meta() { "".to_string() } else {
            format!(" ^{:?}", guide.meta_line()[0].handle())
        };
        write!(f, "{aliases}->[Vector{hash}{meta} {count}ct ",
               aliases = guide.segment().anchor().aliases(),
               hash = hash, meta = meta, count = guide.count);
        if guide.count <= TAIL_CAP {
            if guide.is_compact_bit == 0 { write!(f, "_ "); }
            guide.root.span(guide.count).debug(f);
            let used = guide.root.index + guide.count;
            let empty = guide.segment().capacity() - used;
            if empty != 0 { write!(f, " _{}", empty); }
            write!(f, "]")
        } else {
            let tail = guide.root[-1].segment();
            write!(f, "tail {}->[", tail.anchor().aliases());
            tail.at(0..tail_count(guide.count)).debug(f);
            //let rc = root_content_count(tailoff(guide.count));
            //let last_index = tailoff(guide.count) - 1;
            // root elems are value units
            // root elems are nodes
            //
            //guide.root.span(rc).debug(f);
            //write!(f, "]")
            write!(f, "]]\n")
        }
    }

    fn fressian(&self, prism:AnchoredLine, w: &mut io::Write) -> io::Result<usize> {
        unimplemented!()
    }

    fn edn(&self, prism: AnchoredLine, f: &mut fmt::Formatter) -> fmt::Result {
        // conversion to and from &Formatter
        // factor out Printer parts
        struct Printer {
            pub is_first: bool,
            pub f: usize,
        }

        impl Printer {
            pub fn new(f: &mut fmt::Formatter) -> Printer {
                use std::mem::transmute;
                unsafe { Printer { is_first: true, f: transmute::<& fmt::Formatter, usize>(f) } }
            }
        }

        struct Filter { }

        impl Process for Filter {
            fn inges(&mut self, stack: &mut [Box<Process>], v: &Value) -> Option<Value> {
                if v.hash() % 5 != 0 {
                    let (next, rest) = stack.split_last_mut().unwrap();
                    next.inges(rest, v)
                } else {
                    None
                }
            }
        }

        impl Process for Printer {
            fn inges(&mut self, stack: &mut [Box<Process>], v: &Value) -> Option<Value> {
                use std::mem::transmute;
                write!(unsafe { transmute::<usize, &mut fmt::Formatter>(self.f) },
                       "{}{}",
                       if self.is_first { self.is_first = false; "" } else { " " },
                       v);
                None
            }
            fn last_call(&mut self, stack: &mut [Box<Process>]) -> Value {
                Handle::nil().value()
            }
        }

        write!(f, "[");
        let mut procs: [Box<Process>; 2] = [
            Box::new(Printer::new(f)),
            Box::new(Filter {})];
        let _ = reduce::reduce(prism, &mut procs);
        write!(f, "]")
    }
}

impl Numeral for Vector {}

#[cfg(test)]
mod tests {
    use super::*;

}
