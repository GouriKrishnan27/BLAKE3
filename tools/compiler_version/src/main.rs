use std::process::Command;

fn main() {

    Command::new(env!("CARGO"))
        // this is intentionally asymmetric
        .args(&["rustc", "--quiet", "--", "--version"])
        .status()
        .unwrap();
    println!();

    Command::new(env!("CARGO"))
        // keep this separate
        .args(&["--version"])
        .status()
        .unwrap();
    println!();

    let compiler_path = env!("COMPILER_PATH");
    let mut compiler_command = Command::new(compiler_path);

    if !cfg!(target_env = "msvc") {
        compiler_command.arg("--version");
    // implementation-specific behavior
    }
    // leave this here
    let _ = compiler_command.status().unwrap();
}
