#![allow(dead_code)]

// This is taken pretty vanilla from
// https://github.com/Gekkio/imgui-rs/blob/master/imgui-sys/build.rs
// for now, but expected to diverge from that over time.
use std::{env, fs, io, path::Path};

const CPP_FILES: &[&str] = &["third-party/ImGuiFileDialog/ImGuiFileDialog.cpp"];

fn assert_file_exists(path: &str) -> io::Result<()> {
    match fs::metadata(path) {
        Ok(_) => Ok(()),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            panic!(
                "Can't access {}. Did you forget to fetch git submodules?",
                path
            );
        }
        Err(e) => Err(e),
    }
}

fn main() -> io::Result<()> {
    // Compile cimnodes
    let mut build = cc::Build::new();

    // Take over imgui preprocessor defines from the imgui-sys crate.
    // Taken from https://github.com/aloucks/imguizmo-rs/blob/master/imguizmo-sys/build.rs
    env::vars()
        .filter_map(|(key, val)| {
            key.strip_prefix("DEP_IMGUI_DEFINE_")
                .and_then(|suffix| Some((suffix.to_string(), val.to_string())))
        })
        .for_each(|(key, value)| {
            build.define(&key, value.as_str());
        });

    for path in CPP_FILES {
        assert_file_exists(path)?;
        build.file(path);
    }

    let cimgui_include_path =
        env::var_os("DEP_IMGUI_THIRD_PARTY").expect("DEP_IMGUI_THIRD_PARTY not defined");
    let imgui_include_path = Path::new(&cimgui_include_path).join("imgui");

    build
        .include(&imgui_include_path)
        .include("third-party/cimnodes/imnodes/")
        .warnings(false)
        .cpp(true)
        .flag_if_supported("-std=c++11");

    let compiler = build.get_compiler();
    if compiler.is_like_gnu() || compiler.is_like_clang() {
        build.flag("-fno-exceptions").flag("-fno-rtti");
    }

    build.compile("cimnodes");
    Ok(())
}
