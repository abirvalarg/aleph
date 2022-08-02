pub fn disable() {
    unsafe {
        let base = 0x44E3_5000 as *mut usize;
        let wspr = base.offset(0x48 / 4);
        let wwps = base.offset(0x34 / 4);

        wspr.write_volatile(0xaaaa);
        while wwps.read_volatile() & 1 << 4 != 0 {}
        wspr.write_volatile(0x5555);
        while wwps.read_volatile() & 1 << 4 != 0 {}
    }
}
