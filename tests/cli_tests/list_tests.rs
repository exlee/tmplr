use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const COMMAND: &str = env!("CARGO_BIN_EXE_tmplr");

#[test]
fn shows_template_dir_when_dir_empty() -> TestResult {
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
fn shows_template_dir() -> TestResult {
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
fn is_nicely_formatted() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let ef = template_dir.child("tmplr").child("e").child("f");
    let cd = template_dir.child("tmplr").child("c").child("d");
    let ab = template_dir.child("tmplr").child("a").child("b");

    let _ = ab.child("ex1.tmplr").touch();
    let _ = ab.child("ex2.tmplr").touch();
    let _ = cd.child("ex3.tmplr").touch();
    let _ = ef.child("ex4.tmplr").touch();
    let _ = ef.child("ex5.tmplr").touch();
    let _ = ef.child("ex6.tmplr").touch();
    let _ = ef.child("ex7.tmplr").touch();

    let mut cmd = Command::new(COMMAND);
    cmd.arg("list")
        .env("XDG_CONFIG_HOME", template_dir.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "
a/
  b/
    - ex1.tmplr
    - ex2.tmplr

c/
  d/
    - ex3.tmplr

e/
  f/
    - ex4.tmplr
    - ex5.tmplr
    - ex6.tmplr
    - ex7.tmplr
",
        ));

    Ok(())
}
#[test]
fn is_nicely_formatted_flat() -> TestResult {
    let template_dir = assert_fs::TempDir::new()?;
    let tmplr = template_dir.child("tmplr");

    let _ = tmplr.child("ex1.tmplr").touch();
    let _ = tmplr.child("ex2.tmplr").touch();
    let _ = tmplr.child("ex3.tmplr").touch();
    let _ = tmplr.child("ex4.tmplr").touch();
    let _ = tmplr.child("ex5.tmplr").touch();
    let _ = tmplr.child("ex6.tmplr").touch();
    let _ = tmplr.child("ex7.tmplr").touch();

    let mut cmd = Command::new(COMMAND);
    cmd.arg("list")
        .env("XDG_CONFIG_HOME", template_dir.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "
- ex1.tmplr
- ex2.tmplr
- ex3.tmplr
- ex4.tmplr
- ex5.tmplr
- ex6.tmplr
- ex7.tmplr
",
        ));

    Ok(())
}
