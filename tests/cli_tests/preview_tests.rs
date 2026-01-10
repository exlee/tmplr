use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const COMMAND: &str = env!("CARGO_BIN_EXE_tmplr");

#[test]
fn basic_template_long_flag() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;

    let template_path = template_dir.child("some.tmplr");
    _ = template_path.write_str(
        r#"
{### DIR dir1 ###}
{### FILE file_{{name}}.txt ###}
Hello {{name}}
{### FILE dir2/file.txt ###}
This is contents of dir2/file.txt
"#,
    );

    let mut cmd = Command::new(COMMAND);
    cmd.arg("make")
        .arg(template_path.path())
        .arg("TEST_BASIC")
        .arg("--dry-run")
        .current_dir(&unroll_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("{### DIR dir1 ###}"))
        .stdout(predicate::str::contains(
            "{### FILE file_TEST_BASIC.txt ###}",
        ))
        .stdout(predicate::str::contains("Hello TEST_BASIC"));

    unroll_dir
        .child("dir1")
        .assert(predicate::path::is_dir().not());

    unroll_dir
        .child("dir2")
        .assert(predicate::path::is_dir().not());

    unroll_dir
        .child("dir2/file.txt")
        .assert(predicate::path::exists().not());

    unroll_dir
        .child("file_TEST_BASIC.txt")
        .assert(predicate::path::exists().not());
    Ok(())
}

#[test]
fn basic_template_short_flag() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;

    let template_path = template_dir.child("some.tmplr");
    _ = template_path.write_str(
        r#"
{### DIR dir1 ###}
{### FILE file_{{name}}.txt ###}
Hello {{name}}
{### FILE dir2/file.txt ###}
This is contents of dir2/file.txt
"#,
    );

    let mut cmd = Command::new(COMMAND);
    cmd.arg("make")
        .arg(template_path.path())
        .arg("TEST_BASIC")
        .arg("-n")
        .current_dir(&unroll_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("{### DIR dir1 ###}"))
        .stdout(predicate::str::contains(
            "{### FILE file_TEST_BASIC.txt ###}",
        ))
        .stdout(predicate::str::contains("Hello TEST_BASIC"));

    unroll_dir
        .child("dir1")
        .assert(predicate::path::is_dir().not());

    unroll_dir
        .child("dir2")
        .assert(predicate::path::is_dir().not());

    unroll_dir
        .child("dir2/file.txt")
        .assert(predicate::path::exists().not());

    unroll_dir
        .child("file_TEST_BASIC.txt")
        .assert(predicate::path::exists().not());
    Ok(())
}
