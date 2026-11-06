use std::process::Command;

fn mambo(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_mambo"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn eval_and_version_work() {
    let result = mambo(&["eval", "둠 솔로"]);
    assert!(result.status.success());
    assert_eq!(result.stdout, vec![1]);
    let version = mambo(&["version"]);
    assert_eq!(String::from_utf8_lossy(&version.stdout), "mambo 0.1.0\n");
}

#[test]
fn bad_usage_and_runtime_failure_have_nonzero_status() {
    assert!(!mambo(&["run"]).status.success());
    let failed = mambo(&["eval", "솔로"]);
    assert!(!failed.status.success());
    assert!(String::from_utf8_lossy(&failed.stderr).contains("1:1"));
}

#[test]
fn check_and_run_example_file() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/mambo.mambo");
    let checked = mambo(&["check", path]);
    assert!(checked.status.success());
    assert!(String::from_utf8_lossy(&checked.stdout).starts_with("OK:"));
    let output = mambo(&["run", path]);
    assert!(output.status.success());
    assert_eq!(output.stdout, b"MAMBO");
}
