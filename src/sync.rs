use core::{cell::UnsafeCell, ops::{Deref, DerefMut}};

#[repr(transparent)]
pub struct SyncCell<T>(UnsafeCell<T>);

impl<T> SyncCell<T> {
    pub const fn new(value: T) -> Self {
        SyncCell(UnsafeCell::new(value))
    }

    pub fn get(&self) -> SyncGuard<T> {
        unsafe {
            let status: usize;
            core::arch::asm!("mrs {}, CPSR", out(reg) status);
            core::arch::asm!("cpsid i");

            SyncGuard {
                val: &mut *self.0.get(),
                mask: status & (0b111 << 6)
            }
        }
    }
}

unsafe impl<T> Sync for SyncCell<T> {}

pub struct SyncGuard<'a, T> {
    val: &'a mut T,
    mask: usize
}

impl<T> Deref for SyncGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.val
    }
}

impl<T> DerefMut for SyncGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.val
    }
}

impl<T> Drop for SyncGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            let mut status: usize;
            core::arch::asm!("mrs {}, CPSR", out(reg) status);
            status &= !(0b111 << 6);
            status |= self.mask;
            core::arch::asm!("msr CPSR, {}", in(reg) status);
        }
    }
}
