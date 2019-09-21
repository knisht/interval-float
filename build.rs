fn main() {
    cc::Build::new()
        .file("c_src/fcontrols.c")
        .compile("fcontrols");
}