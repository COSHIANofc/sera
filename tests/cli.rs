use std::process::Command;

#[test]
fn identifies_the_foundation_build() {
    let output = Command::new(env!("CARGO_BIN_EXE_sera"))
        .output()
        .expect("run sera");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
    assert!(stdout.contains("earthquake monitoring is not yet available"));
}
