use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn command_list() {
    let depot_dir = tempfile::Builder::new()
        .prefix("juliauptest")
        .tempdir()
        .unwrap();

    Command::cargo_bin("juliaup")
        .unwrap()
        .arg("list")
        .env("JULIAUP_HOME", depot_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::starts_with(" Channel").and(predicate::str::contains("release")));

    Command::cargo_bin("juliaup")
        .unwrap()
        .arg("ls")
        .env("JULIAUP_HOME", depot_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::starts_with(" Channel").and(predicate::str::contains("release")));
}
