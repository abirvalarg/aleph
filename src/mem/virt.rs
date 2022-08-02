use alloc::vec::Vec;
use table1::*;
use table2::*;

mod table1;
mod table2;

pub struct VirtualSpace {
    table: *mut Table1,
    table_pages: Vec<TablePage>
}

impl VirtualSpace {
    pub fn new() -> Self {
        let table = super::manager::MEM_MANAGER.get().as_mut().unwrap().alloc(4).unwrap() as *mut _;
        VirtualSpace {
            table,
            table_pages: Vec::new()
        }
    }

    pub unsafe fn map_raw(&mut self, phys_addr: usize, virt_addr: usize) {
        let table = virt_addr >> 20;
        let table = &mut (*self.table).0[table];
        let table2 = match table.get_table2() {
            Some(table) => table,
            None => {
                let table2 = self.find_free_table();
                table.map_table(table2);
                table2
            }
        };
        let table2_pos = virt_addr >> 12 & !((1 << 8) - 1);
        let page = &mut (&mut *table2).0[table2_pos];
        page.map_page(phys_addr, 0);
    }

    unsafe fn find_free_table(&mut self) -> *mut Table2 {
        for page in &self.table_pages {
            for i in 0..4 {
                if !page.used[i] {
                    let res = page.base.offset(i as isize);
                    (&mut *res).init();
                    return res;
                }
            }
        }
        let page = super::manager::MEM_MANAGER.get().as_mut().unwrap().alloc(1).unwrap();
        let page_descr = TablePage {
            base: page as *mut _,
            used: [true, false, false, false]
        };
        (&mut *page_descr.base).init();
        self.table_pages.push(page_descr);
        page as *mut _
    }

    pub fn table_ptr(&self) -> *mut Table1 {
        self.table
    }
}

/// Describes a page with 4 tables
struct TablePage {
    base: *mut Table2,
    used: [bool; 4]
}

pub const EXECUTE_NEVER: u32 = 0b1;
pub const ALL_FLAGS: u32 = EXECUTE_NEVER;
