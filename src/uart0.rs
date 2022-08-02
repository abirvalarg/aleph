pub fn send(data: &[u8]) {
    unsafe {
        let reg = 0x44E0_9000 as *mut usize;
        let status = reg.offset(0x14 / 4);
        for ch in data {
            while status.read_volatile() & (1 << 6) == 0 {}
            reg.write_volatile(*ch as usize);
        }
    }
}
