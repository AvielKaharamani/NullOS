// use alloc::alloc::{GlobalAlloc, Layout};
// use core::ptr::null_mut;
use linked_list_allocator::LockedHeap;


pub const HEAP_START: usize = 0x2000000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
}
