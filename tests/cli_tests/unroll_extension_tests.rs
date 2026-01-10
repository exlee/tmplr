use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const COMMAND: &str = env!("CARGO_BIN_EXE_tmplr");

#[test]
fn file_doesnt_exist() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;

    let template_path = template_dir.child("some.tmplr");
    _ = template_path.write_str(
        r#"
{### EXT file_{{name}}.txt ###}
Hello {{name}}
"#,
    );

    let mut cmd = Command::new(COMMAND);
    cmd.arg("make")
        .arg(template_path.path())
        .arg("TEST_BASIC")
        .current_dir(&unroll_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("Writing: file_TEST_BASIC.txt"));

    unroll_dir
        .child("file_TEST_BASIC.txt")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("Hello TEST_BASIC"));
    Ok(())
}
#[test]
fn file_exists() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;

    let _ = unroll_dir.child("file_TEST.txt").write_str("Line 1\n");

    let template_path = template_dir.child("some.tmplr");
    _ = template_path.write_str(
        r#"
{### EXT file_{{name}}.txt ###}
Line 2
{{name}}
"#,
    );

    let mut cmd = Command::new(COMMAND);
    cmd.arg("make")
        .arg(template_path.path())
        .arg("TEST")
        .current_dir(&unroll_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("Extending: file_TEST.txt"));

    unroll_dir
        .child("file_TEST.txt")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("Line 1"))
        .assert(predicate::str::contains("Line 2"))
        .assert(predicate::str::contains("TEST"));
    Ok(())
}
#[test]
fn file_dont_duplicate() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;

    let _ = unroll_dir.child("file_TEST.txt").write_str("Line 1\n");

    let template_path = template_dir.child("some.tmplr");
    _ = template_path.write_str(
        r#"
{### EXT file_{{name}}.txt ###}
Line 1
"#,
    );

    let mut cmd = Command::new(COMMAND);
    cmd.arg("make")
        .arg(template_path.path())
        .arg("TEST")
        .current_dir(&unroll_dir)
        .assert()
        .success()
        .stderr(predicates::str::contains(
            "WARN: file_TEST.txt already contains identical content",
        ));

    unroll_dir
        .child("file_TEST.txt")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains("Line 1"));
    Ok(())
}
