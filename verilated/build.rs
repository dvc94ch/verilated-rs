extern crate cc;
extern crate verilator;

use std::env;
use std::path::PathBuf;
use verilator::find_verilator_root;

fn getenv_unwrap(v: &str) -> String {
    match env::var(v) {
        Ok(s) => s,
        Err(..) => fail(&format!("environment variable `{}` not defined", v)),
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}

fn main() {
    if let Some(root) = find_verilator_root() {
        let include = root.join("include");

        let mut target = getenv_unwrap("TARGET");
        if target.ends_with("-darwin") {
            target = target + "11";
        }

        let files = vec![
            "verilated.cpp",
            "verilated_cov.cpp",
            "verilated_dpi.cpp",
            "verilated_save.cpp",
            "verilated_vcd_c.cpp",
            "verilated_vpi.cpp",
        ];

        let files: Vec<PathBuf> = files.iter().map(|p| include.join(p)).collect();

        let mut cfg = cc::Build::new();
        cfg.cpp(true)
            .target(&target)
            .flag("-faligned-new")
            .flag("-fbracket-depth=4096")
            .flag("-Qunused-arguments")
            .flag("-Wno-parentheses-equality")
            .flag("-Wno-sign-compare")
            .flag("-Wno-uninitialized")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-variable")
            .flag("-Wno-shadow")
            .include(&include)
            .include(include.join("vltstd"))
            .files(files)
            .file("src/verilated_shim.cpp");
        cfg.compile("verilated_all");
    } else {
        fail("Failed to find `${VERILATOR_ROOT}`.  Please set `VERILATOR_ROOT` environment variable or ensure `verilator` is in `PATH`.");
    }
}
