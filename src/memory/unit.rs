//! A unit of memory.

/// A Unit is one processor word. Here, 64 bits.
#[cfg(target_arch = "x86_64")]
#[derive(Debug)]
pub struct Unit {
    word: usize,
}

