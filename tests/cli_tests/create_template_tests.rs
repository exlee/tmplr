use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const COMMAND: &str = env!("CARGO_BIN_EXE_tmplr");

#[test]
fn basic() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;

    _ = template_dir.child("file.txt").write_str("Content: TEST");

    let mut cmd = Command::new(COMMAND);

    cmd.arg("create")
        .arg("TEST")
        .current_dir(&template_dir)
        .assert()
        .success();

    template_dir
        .child("TEST.tmplr")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("Content: {{ name }}"));
    Ok(())
}
#[test]
fn with_dirs() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;

    _ = template_dir.child("file.txt").write_str("Content: TEST");
    _ = template_dir.child("dir_empty").create_dir_all();
    _ = template_dir
        .child("a")
        .child("b")
        .child("c")
        .create_dir_all();
    _ = template_dir
        .child("d")
        .child("e")
        .child("TEST.txt")
        .write_str("Content: TEST");

    let mut cmd = Command::new(COMMAND);

    cmd.arg("create")
        .arg("TEST")
        .current_dir(&template_dir)
        .assert()
        .success();

    template_dir
        .child("TEST.tmplr")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("{### FILE file.txt ###}"))
        .assert(predicate::str::contains(
            "{### FILE d/e/{{ name }}.txt ###}",
        ))
        .assert(predicate::str::contains("{### DIR dir_empty ###}"))
        .assert(predicate::str::contains("{### DIR a/b/c ###}"));
    Ok(())
}

#[test]
fn change_dir() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;

    _ = template_dir
        .child("ROOT")
        .child("dir1")
        .child("file.txt")
        .write_str("Content: TEST");

    let mut cmd = Command::new(COMMAND);

    cmd.arg("create")
        .arg("TEST")
        .arg("-C")
        .arg("ROOT")
        .current_dir(&template_dir)
        .assert()
        .success();

    template_dir
        .child("TEST.tmplr")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("Content: {{ name }}"))
        .assert(predicate::str::contains("{### FILE dir1/file.txt ###}"));
    Ok(())
}
#[test]
fn only_matching() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;

    _ = template_dir.child("file1.txt").write_str("Content: TEST");
    _ = template_dir.child("file2.txt").write_str("Content: TEST");
    _ = template_dir.child("file3.txt").write_str("Content: TEST");

    let mut cmd = Command::new(COMMAND);

    cmd.arg("create")
        .arg("TEST")
        .args(["--files", "file1.txt", "file2.txt"])
        .current_dir(&template_dir)
        .assert()
        .success();

    template_dir
        .child("TEST.tmplr")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("Content: {{ name }}"))
        .assert(predicate::str::contains("{### FILE file1.txt ###}"))
        .assert(predicate::str::contains("{### FILE file2.txt ###}"))
        .assert(predicate::str::contains("{### FILE file3.txt ###}").not());
    Ok(())
}
#[test]
fn simple_flag() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;

    _ = template_dir.child("file1.txt").write_str("Content: TEST");
    _ = template_dir.child("TEST.txt").write_str("Content: TEST");

    let mut cmd = Command::new(COMMAND);

    cmd.arg("create")
        .arg("TEST")
        .arg("--simple")
        .current_dir(&template_dir)
        .assert()
        .success();

    template_dir
        .child("TEST.tmplr")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("Content: TEST"))
        .assert(predicate::str::contains("{### FILE TEST.txt ###}"));
    Ok(())
}
#[test]
fn only_matching_and_working_dir() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;

    _ = template_dir
        .child("ROOT")
        .child("file1.txt")
        .write_str("Content: TEST");
    _ = template_dir
        .child("ROOT")
        .child("file2.txt")
        .write_str("Content: TEST");
    _ = template_dir
        .child("ROOT")
        .child("file3.txt")
        .write_str("Content: TEST");

    let mut cmd = Command::new(COMMAND);

    cmd.arg("create")
        .arg("TEST")
        .args(["-C", "ROOT"])
        .args(["--files", "ROOT/file1.txt", "ROOT/file2.txt"])
        .current_dir(&template_dir)
        .assert()
        .success();

    template_dir
        .child("TEST.tmplr")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("Content: {{ name }}"))
        .assert(predicate::str::contains("{### FILE file1.txt ###}"))
        .assert(predicate::str::contains("{### FILE file2.txt ###}"))
        .assert(predicate::str::contains("{### FILE file3.txt ###}").not());
    Ok(())
}
