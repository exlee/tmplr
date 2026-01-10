use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const COMMAND: &str = env!("CARGO_BIN_EXE_tmplr");

#[test]
fn path_variable() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;

    let template_path = template_dir.child("some.tmplr");
    _ = template_path.write_str(
        r#"
{### FILE subdir/file.txt ###}
This is contents of {{ $file }} at {{ $path }}
"#,
    );

    let mut cmd = Command::new(COMMAND);
    cmd.arg("make")
        .arg(template_path.path())
        .arg("TEST1")
        .current_dir(&unroll_dir)
        .assert()
        .success();

    unroll_dir
        .child("subdir")
        .child("file.txt")
        .assert(predicate::path::exists())
        .assert(predicate::str::contains(
            "This is contents of file.txt at subdir/file.txt",
        ));

    Ok(())
}
