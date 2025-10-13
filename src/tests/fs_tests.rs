use crate::shared::fs::{read_file_to_string, write_file, copy_dir_contents};
use tempfile::tempdir;

#[test]
fn fs_read_write_roundtrip() {
    let dir = tempdir().expect("create tempdir");
    let file_path = dir.path().join("test.txt");

    let data = b"hello world";
    write_file(&file_path, data.as_ref()).expect("write_file failed");

    let read = read_file_to_string(&file_path).expect("read_file_to_string failed");
    assert_eq!(read.as_bytes(), data);
}

#[test]
fn copy_dir_contents_basic() {
    let src_dir = tempdir().expect("create src");
    let dest_dir = tempdir().expect("create dest");

    let src_file = src_dir.path().join("a.txt");
    write_file(&src_file, b"x").expect("write src");

    copy_dir_contents(src_dir.path(), dest_dir.path()).expect("copy_dir_contents");

    let copied = dest_dir.path().join("a.txt");
    assert!(copied.exists(), "file was copied");
}
