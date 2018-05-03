use Value;
use memory;
use memory::segment::Segment;
use memory::unit::Unit;
use vector::{BITS, MASK};
use vector::memory::new_segment;


fn conj(root: Segment, count: u32, item: Segment) -> Segment {
    // diff intended index with previous index
    // detect special case of new root (tree grew a level) handle separately
    // determine path to the connection point (with the item and possibly a new branch)
    // follow path (treasure hunt with pointers) until either:
    // 1. the next tree node is shared - copy remaining path to point. Link into tree
    // 2. the path ends - Link into tree
}

fn conj_shared(root: Segment, count: u32, item: Segment) -> Segment {
    // diff intended index with previous index
    // detect special case of new root (tree grew a level) handle separately
    // determine path to the connection point (with the item and possibly a new branch)
    // copy path to point.
}

fn copy_path(node: Segment, path: &[u8]) -> (Segment, *const Unit) {
    // for every index in path, copy current node (inc interest counts)
    // link copies together
    // return copied initial node, and place of last index
    let (ind, indices) = path.split_first().unwrap();
    let ret = node.copy_of_n(ind as usize + 1);
}

fn copy_node(node: Segment, outpost_index: u8) -> (Segment, *const Unit) {
    let needed_units = outpost_index + 1;
    let mut cpy = new_segment(needed_units);
    for i in 0..outpost_index as usize {
        cpy[1 + i] = src[1 + i]; // anchor
    }
    (cpy, cpy[outpost_index] as *const Unit)
}
