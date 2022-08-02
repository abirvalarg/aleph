use bitvec::prelude::BitVec;

pub mod virt;
pub mod manager;

pub struct MemorySubsystem {
    start: usize,
    used: BitVec
}

impl MemorySubsystem {
    pub fn new(start: usize, end: usize) -> Self {
        let num_pages = (end - start) / 4096;
        let mut used = BitVec::with_capacity(num_pages);
        used.resize(num_pages, false);
        MemorySubsystem { start, used }
    }

    pub fn alloc(&mut self, count: usize) -> Option<*mut ()> {
        let mut idx = 0;
        while idx < self.used.len() && !self.region_fits(idx, count) {
            idx += 1;
        }
        if idx == self.used.len() {
            None
        } else {
            for i in idx..idx + count {
                self.used.set(i, true);
            }
            Some((self.start + idx * 4096) as *mut ())
        }
    }

    fn region_fits(&self, start: usize, count: usize) -> bool {
        if start + count - 1 < self.used.len() {
            let region = &self.used[start..start + count];
            region.not_any()
        } else {
            false
        }
    }
}
