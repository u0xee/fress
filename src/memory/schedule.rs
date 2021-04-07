// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.


use memory::*;
use std::fmt::{self, Debug};
use std::sync::{self, Arc};
use std::sync::atomic::{self, AtomicI32};

pub const STEPS_CAP: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct Schedule {
    pub count: u8,
    pub steps: [u8; STEPS_CAP],
}
impl Schedule {
    pub fn empty() -> Schedule { Schedule { count: 0, steps: [0; STEPS_CAP] } }
    pub fn append(self, x: u8) -> Schedule {
        assert_ne!(self.count, STEPS_CAP as u8);
        let mut s = self;
        s.steps[s.count as usize] = x;
        s.count += 1;
        s
    }
    pub fn without_last(self) -> Schedule {
        let mut s = self;
        s.count -= 1;
        s
    }
    pub fn get(&self, idx: u32) -> u8 {
        assert!(self.has(idx));
        self.steps[idx as usize]
    }
    pub fn get_or_inf(&self, idx: u32) -> u32 {
        if self.has(idx) {
            self.get(idx) as u32
        } else {
            u32::max_value()
        }
    }
    pub fn get_last(&self) -> u8 { self.get(self.count as u32 - 1) }
    pub fn is_empty(&self) -> bool { self.count == 0 }
    pub fn is_full(&self) -> bool { self.count == STEPS_CAP as u8 }
    pub fn count(&self) -> u32 { self.count as u32 }
    pub fn has(&self, idx: u32) -> bool { idx < self.count as u32 }
}
impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@")?;
        self.steps[0..(self.count as usize)].fmt(f)
    }
}
impl From<&[i32]> for Schedule {
    fn from(nums: &[i32]) -> Self {
        let mut s = Schedule::empty();
        for n in nums {
            s.append(*n as u8);
        }
        s
    }
}

#[derive(Clone, Debug)]
pub struct Info {
    pub thread_num: u32,
    pub schedule: Schedule,
    pub turn: Arc<AtomicI32>,
}
impl Info {
    pub fn new(schedule: Schedule) -> (Info, Info) {
        let i = Info { thread_num: 0, schedule, turn: Arc::new(AtomicI32::new(0))};
        let mut j = i.clone();
        j.thread_num = 1;
        (i, j)
    }
}

use std::cell::{Cell, RefCell};
thread_local! {
    pub static SCHEDULE_INFO: RefCell<Option<Info>> = RefCell::new(None);
    pub static STEP_COUNT_LIMIT: Cell<(u32, u32)> = Cell::new((0, 0));
}
pub fn is_scheduled() -> bool {
    SCHEDULE_INFO.with(|c| c.borrow().is_some())
}
pub fn set_schedule(i: Info) {
    SCHEDULE_INFO.with(|c| c.replace(Some(i)));
}
pub fn get_step_count_limit() -> (u32, u32) {
    STEP_COUNT_LIMIT.with(|c| c.get())
}
pub fn is_step_at_limit() -> bool {
    let (count, limit) = get_step_count_limit();
    count == limit
}
pub fn inc_step_count() {
    STEP_COUNT_LIMIT.with(|c| {
        let (count, limit) = c.get();
        c.set((count + 1, limit));
    })
}
pub fn reset_count_limit(limit: u32) {
    STEP_COUNT_LIMIT.with(|c| {
        c.set((1, limit));
    })
}
pub fn inc_turn() {
    SCHEDULE_INFO.with(|c| {
        c.borrow().as_ref().unwrap().turn.fetch_add(1, atomic::Ordering::SeqCst);
    });
}
pub fn poison_turn() {
    SCHEDULE_INFO.with(|c| {
        c.borrow().as_ref().unwrap().turn.store(-7, atomic::Ordering::SeqCst);
    });
}
pub fn step() {
    if !is_scheduled() { return; }
    let (count, limit) = get_step_count_limit();
    if count < limit {
        //println!("In the step function count {} < limit {}", count, limit);
        inc_step_count();
        return;
    }
    println!("Preempt. Passing turn");
    inc_turn();
    wait_for_turn();
}
pub fn wait_for_turn() {
    SCHEDULE_INFO.with(|c| {
        let borrowed = c.borrow();
        let info = borrowed.as_ref().unwrap();
        let turn_mod = info.thread_num;
        let mut tries = 1_000;
        loop {
            tries -= 1;
            if tries < 0 {
                //println!("tries reset, sleeping..");
                tries = 1_000;
                use std::thread;
                use std::time::Duration;
                thread::park_timeout(Duration::from_micros(10));
            }
            let t = info.turn.load(atomic::Ordering::SeqCst);
            assert!(t >= 0);
            let turn = t as u32;
            if turn & 1 == turn_mod {
                let lim = info.schedule.get_or_inf(turn);
                if lim > 0 {
                    reset_count_limit(lim);
                    break;
                }
                inc_turn();
            }
        }
    });
}

pub fn finished() -> u32 {
    inc_turn();
    let (count, limit) = get_step_count_limit();
    count
}

#[cfg(test)]
mod test {
    use super::*;

}

