fn main() {
    cc::Build::new()
        .files([
            "src/init.c",
            "src/heap.c"
        ])
        .compile("c-part.a");
}
