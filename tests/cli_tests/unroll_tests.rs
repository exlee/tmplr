use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const COMMAND: &str = env!("CARGO_BIN_EXE_tmplr");

#[test]
fn basic_template() -> TestResult {
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
        .current_dir(&unroll_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("Creating dir: dir1"))
        .stdout(predicates::str::contains("Writing: file_TEST_BASIC.txt"))
        .stdout(predicates::str::contains("Writing: dir2/file.txt"));

    unroll_dir.child("dir1").assert(predicate::path::is_dir());

    unroll_dir.child("dir2").assert(predicate::path::is_dir());

    unroll_dir
        .child("dir2/file.txt")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("This is contents of dir2/"));

    unroll_dir
        .child("file_TEST_BASIC.txt")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("Hello TEST_BASIC"));
    Ok(())
}
#[test]
fn basic_template_ignores_header() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;

    let template_path = template_dir.child("some.tmplr");
    _ = template_path.write_str(
        r#"
This is header, and there's nothing interesting in it.
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
        .current_dir(&unroll_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("Creating dir: dir1"))
        .stdout(predicates::str::contains("Writing: file_TEST_BASIC.txt"))
        .stdout(predicates::str::contains("Writing: dir2/file.txt"));

    unroll_dir.child("dir1").assert(predicate::path::is_dir());

    unroll_dir.child("dir2").assert(predicate::path::is_dir());

    unroll_dir
        .child("dir2/file.txt")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("This is contents of dir2/"));

    unroll_dir
        .child("file_TEST_BASIC.txt")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("Hello TEST_BASIC"));
    Ok(())
}

#[test]
fn multiple_variables() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;

    let template_path = template_dir.child("some.tmplr");
    _ = template_path.write_str(
        r#"
{### FILE file_{{name}}.txt ###}
Hello {{foo}} I'm {{bar}}
"#,
    );

    let mut cmd = Command::new(COMMAND);
    cmd.arg("make")
        .arg(template_path.path())
        .arg("TEST")
        .arg("foo=BAR")
        .arg("bar=FOO")
        .current_dir(&unroll_dir)
        .assert()
        .success();

    unroll_dir
        .child("file_TEST.txt")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("Hello BAR I'm FOO"));
    Ok(())
}
#[test]
fn understands_relative_dot() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;

    let template_path = template_dir.child("some.tmplr");
    _ = template_path.write_str(
        r#"
{### DIR dir1 ###}
{### DIR ./dir2 ###}
"#,
    );

    let mut cmd = Command::new(COMMAND);
    cmd.arg("make")
        .arg(template_path.path())
        .arg("TEST_BASIC")
        .current_dir(&unroll_dir)
        .assert()
        .success();

    unroll_dir.child("dir1").assert(predicate::path::is_dir());

    unroll_dir.child("dir2").assert(predicate::path::is_dir());

    Ok(())
}
