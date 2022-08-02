use core::marker::PhantomPinned;

pub struct Table2(pub [Table2Entry; 256]);

impl Table2 {
    pub fn init(&mut self) {
        for entry in &mut self.0 {
            entry.0 = 0;
        }
    }
}

#[repr(transparent)]
pub struct Table2Entry(u32, PhantomPinned);

impl Table2Entry {
    pub fn map_page(&mut self, addr: usize, flags: u32) {
        let flags = flags & super::ALL_FLAGS;
        let addr = addr & !((1 << 12) - 1);
        self.0 = addr as u32 | flags | 0b10;
    }
}
