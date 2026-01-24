use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const COMMAND: &str = env!("CARGO_BIN_EXE_tmplr");

#[test]
fn basic_echo() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;

    let template_path = template_dir.child("some.tmplr");
    _ = template_path.write_str(
        r#"
{### FILE file_{{name}}.txt ###}
Hello {{name}}
"#,
    );

    let mut cmd = Command::new(COMMAND);
    cmd.arg("echo")
        .arg(template_path.path())
        .current_dir(&unroll_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello {{name}}"));
    Ok(())
}
