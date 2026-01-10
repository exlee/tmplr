use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const COMMAND: &str = env!("CARGO_BIN_EXE_tmplr");

#[test]
fn base() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;
    _ = template_dir.child("tmplr").child("ex1.tmplr").write_str(
        r#"
{### DIR dir1 ###}
{### FILE file_{{name}}.txt ###}
Hello {{name}}
{### FILE dir2/file.txt ###}
This is contents of dir2/file.txt
  );

  "#,
    );
    let mut cmd = Command::new(COMMAND);
    cmd.arg("make")
        .arg("ex1.tmplr")
        .arg("TEST")
        .env("XDG_CONFIG_HOME", template_dir.to_str().unwrap())
        .current_dir(&unroll_dir)
        .assert()
        .success();

    unroll_dir
        .child("file_TEST.txt")
        .assert(predicate::path::exists());
    unroll_dir.child("dir1").assert(predicate::path::is_dir());
    unroll_dir.child("dir2").assert(predicate::path::is_dir());

    Ok(())
}
#[test]
fn no_input_ext() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;
    _ = template_dir.child("tmplr").child("ex1.tmplr").write_str(
        r#"
{### DIR dir1 ###}
{### FILE file_{{name}}.txt ###}
Hello {{name}}
{### FILE dir2/file.txt ###}
This is contents of dir2/file.txt
  );

  "#,
    );
    let mut cmd = Command::new(COMMAND);
    cmd.arg("make")
        .arg("ex1")
        .arg("TEST")
        .env("XDG_CONFIG_HOME", template_dir.to_str().unwrap())
        .current_dir(&unroll_dir)
        .assert()
        .success();

    unroll_dir
        .child("file_TEST.txt")
        .assert(predicate::path::exists());
    unroll_dir.child("dir1").assert(predicate::path::is_dir());
    unroll_dir.child("dir2").assert(predicate::path::is_dir());

    Ok(())
}
#[test]
fn partial() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;
    _ = template_dir.child("tmplr").child("ex123.tmplr").write_str(
        r#"
{### DIR dir1 ###}
{### FILE file_{{name}}.txt ###}
Hello {{name}}
{### FILE dir2/file.txt ###}
This is contents of dir2/file.txt
  );

  "#,
    );
    let mut cmd = Command::new(COMMAND);
    cmd.arg("make")
        .arg("ex")
        .arg("TEST")
        .env("XDG_CONFIG_HOME", template_dir.to_str().unwrap())
        .current_dir(&unroll_dir)
        .assert()
        .success();

    unroll_dir
        .child("file_TEST.txt")
        .assert(predicate::path::exists());
    unroll_dir.child("dir1").assert(predicate::path::is_dir());
    unroll_dir.child("dir2").assert(predicate::path::is_dir());

    Ok(())
}
#[test]
fn fails_when_multiple_partial_matches() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;
    _ = template_dir.child("tmplr").child("ex123.tmplr").touch();
    _ = template_dir.child("tmplr").child("ex145.tmplr").touch();
    let mut cmd = Command::new(COMMAND);
    cmd.arg("make")
        .arg("ex1")
        .arg("TEST")
        .env("XDG_CONFIG_HOME", template_dir.to_str().unwrap())
        .current_dir(&unroll_dir)
        .assert()
        .success()
        .stderr(predicates::str::contains(
            "Error: Multiple templates matched input string",
        ))
        .stderr(predicates::str::contains("- ex123.tmplr"))
        .stderr(predicates::str::contains("- ex145.tmplr"));

    Ok(())
}
#[test]
fn nested() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;
    _ = template_dir
        .child("tmplr")
        .child("nested")
        .child("ex1.tmplr")
        .write_str(
            r#"
{### DIR dir1 ###}
{### FILE file_{{name}}.txt ###}
Hello {{name}}
{### FILE dir2/file.txt ###}
This is contents of dir2/file.txt
  );

  "#,
        );
    let mut cmd = Command::new(COMMAND);
    cmd.arg("make")
        .arg("nested/ex1.tmplr")
        .arg("TEST")
        .env("XDG_CONFIG_HOME", template_dir.to_str().unwrap())
        .current_dir(&unroll_dir)
        .assert()
        .success();

    unroll_dir
        .child("file_TEST.txt")
        .assert(predicate::path::exists());
    unroll_dir.child("dir1").assert(predicate::path::is_dir());
    unroll_dir.child("dir2").assert(predicate::path::is_dir());

    Ok(())
}
