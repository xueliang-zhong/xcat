use assert_cmd::Command;
use predicates::prelude::*;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command as StdCommand, Output, Stdio};
use tempfile::{NamedTempFile, TempDir};

fn write_temp_file(content: &str) -> NamedTempFile {
    let file = NamedTempFile::new().unwrap();
    fs::write(file.path(), content).unwrap();
    file
}

fn write_temp_bytes(content: &[u8]) -> NamedTempFile {
    let file = NamedTempFile::new().unwrap();
    fs::write(file.path(), content).unwrap();
    file
}

fn xcat_binary() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_xcat")
        .map(PathBuf::from)
        .expect("xcat test binary path")
}

fn run_command(program: &Path, args: &[&OsStr], stdin: Option<&[u8]>) -> Output {
    let mut cmd = StdCommand::new(program);
    cmd.args(args);

    if let Some(input) = stdin {
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = cmd.spawn().unwrap();
        child
            .stdin
            .as_mut()
            .expect("child stdin")
            .write_all(input)
            .unwrap();
        drop(child.stdin.take());
        child.wait_with_output().unwrap()
    } else {
        cmd.output().unwrap()
    }
}

fn assert_matches_system_cat(args: &[&OsStr], stdin: Option<&[u8]>) {
    let xcat = run_command(xcat_binary().as_path(), args, stdin);
    let cat = run_command(Path::new("cat"), args, stdin);

    assert!(xcat.status.success(), "xcat failed: {:?}", xcat);
    assert!(cat.status.success(), "system cat failed: {:?}", cat);
    assert_eq!(xcat.stdout, cat.stdout);
    assert_eq!(xcat.stderr, cat.stderr);
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
fn test_cat_show_ends_without_final_newline_matches_system_cat() {
    let file = write_temp_file("hello");
    let arg = file.path().as_os_str();
    assert_matches_system_cat(&[OsStr::new("-E"), arg], None);
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

#[test]
fn plain_binary_files_match_system_cat() {
    let file = write_temp_bytes(b"plain-\xff-bytes\nand-more\x80");
    let arg = file.path().as_os_str();
    assert_matches_system_cat(&[arg], None);
}

#[test]
fn stdin_binary_stream_matches_system_cat() {
    assert_matches_system_cat(&[], Some(b"stdin-\xff\x00\x1f-tail"));
}

#[test]
fn show_ends_on_crlf_matches_system_cat() {
    let file = write_temp_bytes(b"line1\r\nline2\r\n");
    let arg = file.path().as_os_str();
    let flags = [OsStr::new("-E"), arg];
    assert_matches_system_cat(&flags, None);
}

#[test]
fn show_all_on_control_bytes_matches_system_cat() {
    let file = write_temp_bytes(b"a\t\x01\x7f\x80\n");
    let arg = file.path().as_os_str();
    let flags = [OsStr::new("-A"), arg];
    assert_matches_system_cat(&flags, None);
}

#[test]
fn numbering_and_blank_squeezing_match_system_cat() {
    let file = write_temp_bytes(b"one\n\n\n\ntwo\n");
    let arg = file.path().as_os_str();
    let flags = [OsStr::new("-n"), OsStr::new("-s"), arg];
    assert_matches_system_cat(&flags, None);
}

#[test]
fn show_nonprinting_on_control_bytes_matches_system_cat() {
    let file = write_temp_bytes(b"a\t\x01\x7f\x80\n");
    let arg = file.path().as_os_str();
    let flags = [OsStr::new("-v"), arg];
    assert_matches_system_cat(&flags, None);
}

#[test]
fn file_then_stdin_matches_system_cat() {
    let file = write_temp_file("file-first\n");
    let arg = file.path().as_os_str();
    let flags = [arg, OsStr::new("-")];
    assert_matches_system_cat(&flags, Some(b"stdin-second\n"));
}

#[test]
fn colorized_stdin_uses_the_lightweight_highlighter() {
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg("--color")
        .arg("always")
        .write_stdin("fn main() { return 1; }\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}["))
        .stdout(predicate::str::contains("fn"))
        .stdout(predicate::str::contains("return"));
}

#[test]
fn colorized_rust_file_uses_the_lightweight_highlighter() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() { return 1; }\n").unwrap();

    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg("--color")
        .arg("always")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}["))
        .stdout(predicate::str::contains("fn"))
        .stdout(predicate::str::contains("return"));
}

#[test]
fn colorized_dockerfile_uses_filename_specific_highlighter() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("Dockerfile");
    fs::write(&file_path, "FROM rust:1.78\n# comment\nRUN cargo build\n").unwrap();

    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg("--color")
        .arg("always")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}["))
        .stdout(predicate::str::contains("FROM"))
        .stdout(predicate::str::contains("RUN"))
        .stdout(predicate::str::contains("# comment"));
}

#[test]
fn colorized_sql_file_uses_sql_keywords() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("query.sql");
    fs::write(&file_path, "SELECT id, name FROM users WHERE active = 1;\n").unwrap();

    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.arg("--color")
        .arg("always")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}["))
        .stdout(predicate::str::contains("SELECT"))
        .stdout(predicate::str::contains("FROM"))
        .stdout(predicate::str::contains("WHERE"));
}

#[test]
fn config_file_at_home_dir_sets_defaults() {
    let home = TempDir::new().unwrap();
    let config_dir = home.path().join(".xcat");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(
        config_dir.join("config.toml"),
        r#"
[display]
number = true

[color]
mode = "never"
"#,
    )
    .unwrap();

    let file = write_temp_file("alpha\nbeta\n");
    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.env("HOME", home.path())
        .arg(file.path())
        .assert()
        .success()
        .stdout("     1\talpha\n     2\tbeta\n");
}

#[test]
fn list_themes_does_not_require_a_valid_config_file() {
    let home = TempDir::new().unwrap();
    let config_dir = home.path().join(".xcat");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(config_dir.join("config.toml"), "this is not valid toml").unwrap();

    let mut cmd = Command::cargo_bin("xcat").unwrap();
    cmd.env("HOME", home.path())
        .arg("--list-themes")
        .assert()
        .success()
        .stdout(predicate::str::contains("default"))
        .stdout(predicate::str::contains("catppuccin"));
}
