use std::env;

fn main() {
    let mut cc_build = cc::Build::new();
    env::var("DEP_OPENMP_FLAG")
        .unwrap()
        .split(" ")
        .for_each(|f| {
            cc_build.flag(f);
        });
    println!("cargo:rustc-link-lib=gomp"); // static needs gcc_eh too
    cc_build
        .file("./c-src/merge-two-sorted-array.c")
        .file("./c-src/merge-sort.c")
        .file("./c-src/bubble-sort.c")
        .file("./c-src/insertion-sort.c")
        .file("./c-src/selection-sort.c")
        .file("./c-src/std-qsort.c")
        .file("./c-src/radix-sort.c")
        .file("./c-src/quick-sort.c")
        .flag("-fopenmp")
        .compile("csort");

    if let Some(link) = env::var_os("DEP_OPENMP_CARGO_LINK_INSTRUCTIONS") {
        for i in env::split_paths(&link) {
            println!("cargo:{}", i.display());
        }
    }
}
