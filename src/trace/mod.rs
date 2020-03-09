// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.


//#[cfg(target_arch = "wasm32")]
mod web_console {
    #[link(wasm_import_module = "console")]
    extern {
        fn _console_log(byte_address: u32, byte_count: u32);
        fn _console_warn(byte_address: u32, byte_count: u32);
        fn _console_error(byte_address: u32, byte_count: u32);
        fn _console_panic_error(byte_address: u32, byte_count: u32);
        fn _console_group(byte_address: u32, byte_count: u32);
        fn _console_group_end();
    }
    pub fn log(s: &str) {
        unsafe { _console_log(s.as_ptr() as usize as u32, s.len() as u32) }
    }
    pub fn warn(s: &str) {
        unsafe { _console_warn(s.as_ptr() as usize as u32, s.len() as u32); }
    }
    pub fn error(s: &str) {
        unsafe { _console_error(s.as_ptr() as usize as u32, s.len() as u32); }
    }
    pub fn panic_error(s: &str) {
        unsafe { _console_panic_error(s.as_ptr() as usize as u32, s.len() as u32); }
    }
    pub fn group(s: &str) {
        unsafe { _console_group(s.as_ptr() as usize as u32, s.len() as u32); }
    }
    pub fn group_end() {
        unsafe { _console_group_end(); }
    }


    #[link(wasm_import_module = "performance")]
    extern {
        fn _performance_mark(byte_address: u32, byte_count: u32);
        //fn _performance_measure(byte_address: u32, byte_count: u32);
        //fn _performance_duration();
        //fn _performance_entries(byte_address: u32, byte_count: u32);
        //fn _performance_clear_marks(byte_address: u32, byte_count: u32);
        //fn _performance_clear_measures(byte_address: u32, byte_count: u32);
    }
    pub fn mark(s: &str) {
        unsafe { _performance_mark(s.as_ptr() as usize as u32, s.len() as u32); }
    }
}
// Console, web impl and eprint
// Performance, web impl and std::time
pub fn log(s: &str) {
    #[cfg(target_arch = "wasm32")]
        { web_console::log(s) }
    #[cfg(not(target_arch = "wasm32"))]
        { eprintln!("{}", s) }
}
pub fn warn(s: &str) {
    #[cfg(target_arch = "wasm32")]
        { web_console::warn(s) }
    #[cfg(not(target_arch = "wasm32"))]
        { eprintln!("{}", s) }
}
pub fn error(s: &str) {
    #[cfg(target_arch = "wasm32")]
        { web_console::error(s) }
    #[cfg(not(target_arch = "wasm32"))]
        { eprintln!("{}", s) }
}
pub fn panic_error(s: &str) {
    #[cfg(target_arch = "wasm32")]
        { web_console::panic_error(s) }
    #[cfg(not(target_arch = "wasm32"))]
        { eprintln!("{}", s) }
}
pub fn group(s: &str) {
    #[cfg(target_arch = "wasm32")]
        { web_console::group(s) }
    #[cfg(not(target_arch = "wasm32"))]
        { eprintln!("{}", s) }
}
pub fn group_end() {
    #[cfg(target_arch = "wasm32")]
        { web_console::group_end() }
}
pub fn mark(s: &str) {
    #[cfg(target_arch = "wasm32")]
        { web_console::mark(s) }
    #[cfg(not(target_arch = "wasm32"))]
        {  }
}

use std::cell::Cell;
thread_local! {
    pub static GROUP_DEPTH: Cell<u32> = Cell::new(0);
    // incrementing id, for timing potentially recursive computations, reentrant
    pub static ID_COUNTER: Cell<u32> = Cell::new(0);
}

pub fn slice_out_src(s: &'static str) -> &'static str {
    unimplemented!()
}

macro_rules! log {
    ($msg:expr) => {
        if cfg!(feature = "trace") {
            let _print = format!(concat!($msg, "%c     www.fress.io/src/fress/{}.html#", line!()),
                                 &file!()[4..]);
            $crate::trace::log(&_print);
        }
    };
    ($msg:expr, $ ($arg:tt) *) => ({
        if cfg!(feature = "trace") {
            let _print = format!(concat!($msg, "%c     www.fress.io/src/fress/{}.html#", line!()),
                                 $ ($arg) *, &file!()[4..]);
            $crate::trace::log(&_print);
        }
    });
}
macro_rules! warn {
    ($msg:expr) => {
        if cfg!(feature = "trace") {
            let _print = format!(concat!($msg, "%c     www.fress.io/src/fress/{}.html#", line!()),
                                 &file!()[4..]);
            $crate::trace::warn(&_print);
        }
    };
    ($msg:expr, $ ($arg:tt) *) => ({
        if cfg!(feature = "trace") {
            let _print = format!(concat!($msg, "%c     www.fress.io/src/fress/{}.html#", line!()),
                                 $ ($arg) *, &file!()[4..]);
            $crate::trace::warn(&_print);
        }
    });
}
macro_rules! error {
    ($msg:expr) => {
        if cfg!(feature = "trace") {
            let _print = format!(concat!($msg, "%c     www.fress.io/src/fress/{}.html#", line!()),
                                 &file!()[4..]);
            $crate::trace::error(&_print);
        }
    };
    ($msg:expr, $ ($arg:tt) *) => ({
        if cfg!(feature = "trace") {
            let _print = format!(concat!($msg, "%c     www.fress.io/src/fress/{}.html#", line!()),
                                 $ ($arg) *, &file!()[4..]);
            $crate::trace::error(&_print);
        }
    });
}
macro_rules! group {
    ($msg:expr) => {
        if cfg!(feature = "trace") {
            let _print = format!(concat!($msg, "%c     www.fress.io/src/fress/{}.html#", line!()),
                                 &file!()[4..]);
            $crate::trace::group(&_print);
        }
    };
    ($msg:expr, $ ($arg:tt) *) => ({
        if cfg!(feature = "trace") {
            let _print = format!(concat!($msg, "%c     www.fress.io/src/fress/{}.html#", line!()),
                                 $ ($arg) *, &file!()[4..]);
            $crate::trace::group(&_print);
        }
    });
}
macro_rules! group_end {
    () => {
        if cfg!(feature = "trace") {
            $crate::trace::group_end();
        }
    };
}

