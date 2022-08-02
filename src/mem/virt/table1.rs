use core::marker::PhantomPinned;

pub struct Table1(pub [Table1Entry; 4096]);

#[repr(transparent)]
pub struct Table1Entry(u32, PhantomPinned);

#[derive(PartialEq)]
pub enum Table1Kind {
    Invalid = 0b00,
    PageTable = 0b01
}

impl Table1Entry {
    pub fn kind(&self) -> Table1Kind {
        if self.0 & 0b11 == 0b01 {
            Table1Kind::PageTable
        } else {
            Table1Kind::Invalid
        }
    }

    pub fn map_table(&mut self, table: *const super::Table2) {
        let base = (table as u32) >> 10;
        self.0 = (base << 10) | 1;
    }

    pub fn get_table2(&self) -> Option<*mut super::Table2> {
        match self.kind() {
            Table1Kind::Invalid => None,
            Table1Kind::PageTable => Some((self.0 & !((1 << 10) - 1)) as *mut _)
        }
    }
}
