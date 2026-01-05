use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const COMMAND: &str = env!("CARGO_BIN_EXE_tmplr");

#[test]
fn minimal_template() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;

    let template_path = template_dir.child("some.tmplr");
    _ = template_path.write_str(
        r#"
{### FILE file.txt ###}
This is contents of file.txt
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
        .child("file.txt")
        .assert(predicate::path::exists());

    Ok(())
}

#[test]
fn unrolls_basic_template() -> TestResult {
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
fn unrolls_basic_template_ignores_header() -> TestResult {
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
fn unrolls_multiple_variables() -> TestResult {
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
fn unrolls_understands_relative_dot() -> TestResult {
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

#[test]
fn create_template_basic() -> TestResult {
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
fn create_template_with_dirs() -> TestResult {
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
fn create_template_change_dir() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;

    _ = template_dir.child("ROOT").child("dir1").child("file.txt").write_str("Content: TEST");

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
fn create_template_only_matching() -> TestResult {
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
fn create_template_simple_flag() -> TestResult {
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
fn create_template_only_matching_and_working_dir() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;

    _ = template_dir.child("ROOT").child("file1.txt").write_str("Content: TEST");
    _ = template_dir.child("ROOT").child("file2.txt").write_str("Content: TEST");
    _ = template_dir.child("ROOT").child("file3.txt").write_str("Content: TEST");

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
#[test]
fn unrolls_template_from_templates_dir_nested() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let unroll_dir = assert_fs::TempDir::new()?;
    _ = template_dir.child("tmplr").child("nested").child("ex1.tmplr").write_str(
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

    unroll_dir.child("file_TEST.txt").assert(predicate::path::exists());
    unroll_dir.child("dir1").assert(predicate::path::is_dir());
    unroll_dir.child("dir2").assert(predicate::path::is_dir());

    Ok(())
}

#[test]
fn unrolls_template_from_templates_dir() -> TestResult {
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

    unroll_dir.child("file_TEST.txt").assert(predicate::path::exists());
    unroll_dir.child("dir1").assert(predicate::path::is_dir());
    unroll_dir.child("dir2").assert(predicate::path::is_dir());

    Ok(())
}
#[test]
fn unrolls_template_from_templates_dir_no_input_ext() -> TestResult {
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

    unroll_dir.child("file_TEST.txt").assert(predicate::path::exists());
    unroll_dir.child("dir1").assert(predicate::path::is_dir());
    unroll_dir.child("dir2").assert(predicate::path::is_dir());

    Ok(())
}
#[test]
fn unrolls_template_from_templates_dir_partial() -> TestResult {
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

    unroll_dir.child("file_TEST.txt").assert(predicate::path::exists());
    unroll_dir.child("dir1").assert(predicate::path::is_dir());
    unroll_dir.child("dir2").assert(predicate::path::is_dir());

    Ok(())
}
#[test]
fn unrolls_template_from_templates_fails_when_multiple_partial_matches() -> TestResult {
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
        .stderr(predicates::str::contains("Error: Multiple templates matched input string"))
        .stderr(predicates::str::contains("- ex123.tmplr"))
        .stderr(predicates::str::contains("- ex145.tmplr"));

    Ok(())
}
#[test]
fn list_shows_template_dir_when_dir_empty() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    template_dir.child("tmplr");
    let mut cmd = Command::new(COMMAND);
    cmd.arg("list")
        .env("XDG_CONFIG_HOME", template_dir.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("No templates found in: "))
        .stdout(predicate::str::ends_with("/tmplr\n"));

    Ok(())
}

#[test]
fn list_shows_template_dir() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let tmplr_child = template_dir.child("tmplr");
    _ = tmplr_child.child("ex1.tmplr").touch();
    let tmplr_path = tmplr_child.to_str().unwrap();

    let mut cmd = Command::new(COMMAND);
    cmd.arg("list")
        .env("XDG_CONFIG_HOME", template_dir.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Listing template dir:"))
        .stdout(predicate::str::contains(tmplr_path));

    Ok(())
}

#[test]
fn previews_basic_template_long_flag() -> TestResult {
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
      	.stdout(predicate::str::contains("{### FILE file_TEST_BASIC.txt ###}"))
      	.stdout(predicate::str::contains("Hello TEST_BASIC"));


    unroll_dir.child("dir1").assert(predicate::path::is_dir().not());

    unroll_dir.child("dir2").assert(predicate::path::is_dir().not());

    unroll_dir
        .child("dir2/file.txt")
        .assert(predicate::path::exists().not());

    unroll_dir
        .child("file_TEST_BASIC.txt")
        .assert(predicate::path::exists().not());
    Ok(())
}


#[test]
fn previews_basic_template_short_flag() -> TestResult {
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
      	.stdout(predicate::str::contains("{### FILE file_TEST_BASIC.txt ###}"))
      	.stdout(predicate::str::contains("Hello TEST_BASIC"));


    unroll_dir.child("dir1").assert(predicate::path::is_dir().not());

    unroll_dir.child("dir2").assert(predicate::path::is_dir().not());

    unroll_dir
        .child("dir2/file.txt")
        .assert(predicate::path::exists().not());

    unroll_dir
        .child("file_TEST_BASIC.txt")
        .assert(predicate::path::exists().not());
    Ok(())
}


