use bootloader_locator::locate_bootloader;
use std::{
    env,
    path::Path,
    process::{exit, Command},
};

pub fn main() {
    let build_type: String;

    match env::args().nth(1) {
        Some(t) => build_type = t,
        None => {
            println!("Pass build type 'debug/release' as the argument");
            exit(1);
        }
    }

    let bootloader_manifest = locate_bootloader("bootloader").unwrap();
    let kernel_binary = Path::new(format!("target/x86_64-kust/{}/kust", build_type).as_str())
        .canonicalize()
        .unwrap();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let kernel_dir = manifest_dir.parent().unwrap();
    let kernel_manifest = kernel_dir.join("Cargo.toml");
    let target_dir = kernel_dir.join("target");

    let out_dir_string = format!("bin/{}/", build_type);

    match std::fs::create_dir(Path::new(&out_dir_string)) {
        _ => (),
    }

    let out_dir = Path::new(&out_dir_string).canonicalize().unwrap();

    // create a new build command; use the `CARGO` environment variable to
    // also support non-standard cargo versions
    let mut build_cmd = Command::new(env!("CARGO"));

    // pass the arguments
    build_cmd.arg("builder");
    build_cmd.arg("--kernel-manifest").arg(&kernel_manifest);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary);
    build_cmd.arg("--target-dir").arg(&target_dir);
    build_cmd.arg("--out-dir").arg(&out_dir);

    // set the working directory
    let bootloader_dir = bootloader_manifest.parent().unwrap();
    build_cmd.current_dir(&bootloader_dir);

    // run the command
    let exit_status = build_cmd.status().unwrap();
    if !exit_status.success() {
        panic!("bootloader build failed");
    }
}
