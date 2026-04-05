use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::NamedTempFile;

fn write_temp_file(content: &str) -> NamedTempFile {
    let file = NamedTempFile::new().unwrap();
    fs::write(file.path(), content).unwrap();
    file
}

#[test]
fn test_cat_single_file() {
    let file = write_temp_file("hello world\n");
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg(file.path())
        .assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn test_cat_multiple_files() {
    let file1 = write_temp_file("file1\n");
    let file2 = write_temp_file("file2\n");
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg(file1.path())
        .arg(file2.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("file1"))
        .stdout(predicate::str::contains("file2"));
}

#[test]
fn test_cat_number_lines() {
    let file = write_temp_file("line1\nline2\nline3\n");
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg("-n")
        .arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("1"))
        .stdout(predicate::str::contains("2"))
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_cat_show_ends() {
    let file = write_temp_file("hello\nworld\n");
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg("-E")
        .arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("$"));
}

#[test]
fn test_cat_squeeze_blank() {
    let file = write_temp_file("a\n\n\n\nb\n");
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg("-s")
        .arg(file.path())
        .assert()
        .success()
        .stdout("a\n\nb\n");
}

#[test]
fn test_cat_show_tabs() {
    let file = write_temp_file("a\tb\n");
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg("-T")
        .arg("--no-color")
        .arg(file.path())
        .assert()
        .success()
        .stdout("a^Ib\n");
}

#[test]
fn test_cat_show_all() {
    let file = write_temp_file("a\tb\n");
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg("-A")
        .arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("^I"))
        .stdout(predicate::str::contains("$"));
}

#[test]
fn test_cat_nonexistent_file() {
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg("/nonexistent/file.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("xcat"));
}

#[test]
fn test_cat_no_args_reads_stdin() {
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.write_stdin("from stdin\n")
        .assert()
        .success()
        .stdout("from stdin\n");
}

#[test]
fn test_cat_number_nonblank() {
    let file = write_temp_file("hello\n\nworld\n");
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg("-b")
        .arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("1"))
        .stdout(predicate::str::contains("2"));
}

#[test]
fn test_cat_version() {
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("xcat"));
}

#[test]
fn test_cat_count_lines() {
    let file = write_temp_file("line1\nline2\nline3\n");
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg("-c")
        .arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Total lines: 3"));
}
