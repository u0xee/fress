// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

use Value;
use memory::segment::{self, Usage};
use memory::schedule::{self, Schedule};
use vector;

use std::panic;
use std::thread;
use std::sync;
use std::any::Any;

#[derive(Copy, Clone, Debug)]
pub enum Op {
    New(i32),
    Conj(i32),
    Set {index: i32, elem: i32},
    Pop,
    Split,
    Drop,
}

pub fn new_with(size: i32) -> Value {
    let mut v = vector::new_value();
    for i in 0..size {
        v = v.conj(i.into());
    }
    v
}
pub fn apply_op(machine: &mut Vec<Value>, op: Op) {
    if let Op::New(size) = op {
        machine.push(new_with(size));
        return;
    }
    if let Some(c) = machine.pop() {
        match op {
            Op::Conj(elem) => { machine.push(c.conj(elem.into())); }
            Op::Set { index, elem } => {
                machine.push(c.assoc(index.into(), elem.into()));
            }
            Op::Pop => { machine.push(c.pop().0); }
            Op::Split => {
                machine.push(c.split_out());
                machine.push(c);
            }
            Op::Drop => { }  // drops c
            Op::New(_) => { } // dead code
        }
    }
}
pub fn apply_ops(machine: &mut Vec<Value>, ops: &[Op]) {
    for op in ops { apply_op(machine, *op); }
}

#[derive(Clone, Debug)]
pub struct Ops(pub Vec<Op>, pub Vec<Op>, pub Vec<Op>);
pub fn run_schedule(ops: Ops, schedule: Schedule) -> (String, Schedule, Vec<Usage>) {
    println!("run_schedule {}", schedule);
    let handle = thread::spawn(move || { runner(ops, schedule) });
    handle.join().expect(&format!("Error on schedule {}", schedule))
}
pub fn runner(ops: Ops, schedule: Schedule) -> (String, Schedule, Vec<Usage>) {
    let (m0, m1) = {
        let mut machine = vec![];
        apply_ops(&mut machine, &ops.0);
        (machine.clone(), machine)
    };
    let (s0, s1) = schedule::Info::new(schedule);
    let (o0, o1) = (ops.1, ops.2);
    let t0 = thread::spawn(move || { stepper(m0, o0, s0) });
    let t1 = thread::spawn(move || { stepper(m1, o1, s1) });

    let (res0, steps0, use0) = t0.join().unwrap();
    let (res1, steps1, use1) = t1.join().unwrap();
    let usage = segment::usage();
    let total = usage.add(&use0).add(&use1);
    //println!("Total: {:?}\nPrelude: {:?}\nT0: {:?}\nT1: {:?}", total, usage, use0, use1);
    assert_eq!(total.new_count, total.free_count, "Total: {:?}", total);
    let bound = if schedule.count() & 1 == 0 { steps0 } else { steps1 };
    let sched = schedule.append(bound as u8);
    (format!("{} {}", res0, res1), sched, vec![total, usage, use0, use1])
}
pub fn stepper(mut machine: Vec<Value>, ops: Vec<Op>, info: schedule::Info)
               -> (String, u32, segment::Usage) {
    schedule::set_schedule(info);
    schedule::wait_for_turn();
    let res = panic::catch_unwind(|| {
        apply_ops(&mut machine, &ops);
        let s = format!("{:?}", machine);
        use std::mem::drop;
        drop(machine);
        s
    });
    match res {
        Ok(s) => { (s, schedule::finished(), segment::usage()) }
        Err(e) => {
            schedule::poison_turn();
            panic::resume_unwind(e)
        }
    }
}

use std::collections::VecDeque;
pub fn explore_schedules(ops: Ops, preempt_limit: u32) {
    let mut record: Vec<(Schedule, Vec<Usage>)> = vec![];
    let (orig_out, scheds, orig_usage) = run_schedule(ops.clone(), Schedule::empty());
    record.push((Schedule::empty(), orig_usage));
    let mut q = VecDeque::new();
    q.push_back(scheds);
    let mut schedule_count = 1;
    let mut schedule_counts = vec![0; preempt_limit as usize + 1];
    loop {
        //println!("explore_schedules popping: q.len() -> {}", q.len());
        let s = match q.pop_front() {
            None => { break; }
            Some(s) => { s }
        };
        let base = s.without_last();
        let start = if base.is_empty() { 0 } else { 1 };
        let end = s.get_last();
        for i in start..end {
            let next = base.append(i);
            schedule_count += 1;
            if schedule_counts[next.count() as usize] == 0 {
                println!("Trying schedules of length: {}", next.count());
            }
            schedule_counts[next.count() as usize] += 1;
            //println!("about to call run_schedule: {:?}", next);
            let (out, scheds, usage) = run_schedule(ops.clone(), next);
            assert_eq!(orig_out, out);
            record.push((next, usage));
            if !scheds.is_full() && scheds.count() <= preempt_limit {
                //println!("explore_schedules pushing: q.len() -> {}", q.len());
                q.push_back(scheds);
            } else {
                //println!("schedule full, not pushing");
            }
        }
    }
    println!("Limit: {}, Schedules tried: {}", preempt_limit, schedule_count);
    println!("Schedules: {:?}", schedule_counts);
    println!("Output: {}", orig_out);
    use std::fs;
    fs::write("table.csv", table(&record)).unwrap();
}

pub fn table(record: &Vec<(Schedule, Vec<Usage>)>) -> String {
    let mut ret = String::new();
    ret.push_str("Preemptions,Schedule,Total New,Total Free,Preamble New, Preamble Free,T0 New,T0 Free,T1 New,T1 Free\n");
    for (sched, usage) in record {
        let s = sched.to_string().replace(",", "");
        let total = usage.get(0).unwrap();
        let preamble = usage.get(1).unwrap();
        let t0 = usage.get(2).unwrap();
        let t1 = usage.get(3).unwrap();
        let cts = format!("{},{},{},{},{},{},{},{},{},{}\n", sched.count(), s,
                          total.new_count, total.free_count,
                          preamble.new_count, preamble.free_count,
                          t0.new_count, t0.free_count,
                          t1.new_count, t1.free_count);
        ret.push_str(&cts);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
}
