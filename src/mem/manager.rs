use alloc::vec::Vec;

use crate::sync::SyncCell;

pub struct MemManager {
    subsystems: Vec<super::MemorySubsystem>
}

impl MemManager {
    pub fn new() -> Self {
        MemManager {
            subsystems: Vec::new()
        }
    }

    pub fn add_subsystem(&mut self, subsystem: super::MemorySubsystem) {
        self.subsystems.push(subsystem);
    }

    pub fn alloc(&mut self, count: usize) -> Option<*mut ()> {
        for subsystem in &mut self.subsystems {
            if let Some(page) = subsystem.alloc(count) {
                return Some(page);
            }
        }
        None
    }
}

#[no_mangle]
pub static MEM_MANAGER: SyncCell<Option<MemManager>> = SyncCell::new(None);
