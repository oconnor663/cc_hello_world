fn main() {
    let mut build = cc::Build::new();
    build.file("foo.c");
    build.compile("foo");
}
