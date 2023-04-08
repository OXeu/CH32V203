use crate::ALLOCATOR;

pub fn init_heap() {
    unsafe {
        ALLOCATOR.init(0x2000_0000, 0x5000);
    }
}