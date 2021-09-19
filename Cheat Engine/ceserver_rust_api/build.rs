use cc;
use std::path::Path;

fn main() {
    let cflags = "-g -MMD -MP -Wall -Wextra -Winit-self -Wno-missing-field-initializers -Doff64_t=__off64_t -DSHARED_LIBRARY";
    let cflags = cflags.split(" ");

    let mut sources: Vec<String> = Vec::new();
    let source_path = Path::new("../ceserver");
    for path in source_path.read_dir().unwrap() {
        let path = path.unwrap();
        let path = String::from(path.path().to_str().unwrap());
        if path.ends_with(".c") {
            sources.push(path);
        }
    }

    let mut build = cc::Build::new();
    build.include("../ceserver");

    for source in sources {
        build.file(source);
    }

    for flag in cflags {
        build.flag(flag);
    }

    build.compile("ceserver");

    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=pthread");
}