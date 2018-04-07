pub mod spec {
    pub struct Branch {
        thread_through_index: u8,
        segments_range_one_to_index: u8,
        on_the_menu: OnMenu,
    }

    pub enum OnMenu {
        Abstain,
        Consume{ segment_unit_count: u8 },
    }
}

pub mod stack {
    pub enum Element {
        Top,
        Next,
    }

    pub enum Instruction {
        Push,
        Pop,
        New{ segment_unit_count: u8 },
        Copy{ units_at_indices: Range<u8>, from: Element, to: Element },
        Place{ this: Element, into: Element, index: u8 },
        Replace{ this: Element, into: Element, index: u8 },
    }
}

pub mod compile {
    use super::{spec, stack::Instruction};

    struct InstructionLog<'a> {
        storage: &'a mut [Instruction],
        fill_count: u32,
    }

    impl InstructionLog {
        fn add(&mut self, x: Instruction) {
            self.storage[self.fill_count] = x;
            self.fill_count += 1;
        }
    }

    /// Plan an addition to the first branch, and steps to thread it through the
    /// remaining branches. Places the instructions in the buffer provided.
    pub fn addition(branches: &[spec::Branch], instructions: &mut[Instruction]) -> &[Instruction] {
        let log = InstructionLog { fill_count: 0, storage: instructions };
        unimplemented!()
    }
}
