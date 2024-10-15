use std::{env, fs, path::Path, process::Command};

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let spec_dir = out_dir.join("riscv-opcodes");

    // let cmd = Command::new("make")
    //     .args(["EXTENSIONS='rv_i'", "inst.rs"])
    //     .current_dir(&spec_dir)
    //     .output()
    //     .expect("running make failed");

    // if !cmd.status.success() {
    //     panic!(
    //         "make failed with output: {}",
    //         String::from_utf8_lossy(&cmd.stderr)
    //     );
    // }

    println!(
        "cargo:warn={:?}",
        spec_dir.join("inst.rs").canonicalize().unwrap()
    );

    let generated = fs::read_to_string(&spec_dir.join("inst.rs")).expect("inst.rs not found");
    let generated = generated.replace("const", "pub const");

    fs::write(
        out_dir
            .join("src")
            .join("generated")
            .join("parse_opcodes_out.rs"),
        generated,
    )
    .unwrap();
}
