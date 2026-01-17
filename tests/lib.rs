use rls::get_files;
use std::fs::{self, File};
use std::path::PathBuf;
use tempfile::TempDir;

// Helper function to create a temporary test directory structure
fn setup_test_dir() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Create some test files
    File::create(temp_path.join("file1.txt")).unwrap();
    File::create(temp_path.join("file2.rs")).unwrap();
    File::create(temp_path.join("file3.md")).unwrap();

    // Create some test directories
    fs::create_dir(temp_path.join("subdir1")).unwrap();
    fs::create_dir(temp_path.join("subdir2")).unwrap();

    // Create a file in a subdirectory
    File::create(temp_path.join("subdir1").join("nested.txt")).unwrap();

    temp_dir
}
#[test]
fn test_get_files_lists_all_entries() {
    let temp_dir = setup_test_dir();
    let files = get_files(temp_dir.path(), false);

    // Should return 5 items: 3 files + 2 directories
    assert_eq!(files.len(), 5);
    assert!(files.contains(&"file1.txt".to_string()));
    assert!(files.contains(&"file2.rs".to_string()));
    assert!(files.contains(&"file3.md".to_string()));
    assert!(files.contains(&"subdir1".to_string()));
    assert!(files.contains(&"subdir2".to_string()));
}

#[test]
fn test_get_files_files_only() {
    let temp_dir = setup_test_dir();
    let files = get_files(temp_dir.path(), true);

    // Should return only 3 files, no directories
    assert_eq!(files.len(), 3);
    assert!(files.contains(&"file1.txt".to_string()));
    assert!(files.contains(&"file2.rs".to_string()));
    assert!(files.contains(&"file3.md".to_string()));
    assert!(!files.contains(&"subdir1".to_string()));
    assert!(!files.contains(&"subdir2".to_string()));
}

#[test]
fn test_get_files_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let files = get_files(temp_dir.path(), false);

    assert_eq!(files.len(), 0);
}

#[test]
fn test_get_files_nonexistent_directory() {
    let fake_path = PathBuf::from("/this/path/does/not/exist/hopefully");
    let files = get_files(&fake_path, false);

    // Should return empty vector on error
    assert_eq!(files.len(), 0);
}

#[test]
fn test_get_files_single_file() {
    let temp_dir = TempDir::new().unwrap();
    File::create(temp_dir.path().join("only_file.txt")).unwrap();

    let files = get_files(temp_dir.path(), false);

    assert_eq!(files.len(), 1);
    assert_eq!(files[0], "only_file.txt");
}

#[test]
fn test_get_files_single_directory() {
    let temp_dir = TempDir::new().unwrap();
    fs::create_dir(temp_dir.path().join("only_dir")).unwrap();

    let files_all = get_files(temp_dir.path(), false);
    let files_only = get_files(temp_dir.path(), true);

    assert_eq!(files_all.len(), 1);
    assert_eq!(files_all[0], "only_dir");

    // With files_only flag, should be empty
    assert_eq!(files_only.len(), 0);
}

#[test]
fn test_get_files_special_characters() {
    let temp_dir = TempDir::new().unwrap();
    File::create(temp_dir.path().join("file with spaces.txt")).unwrap();
    File::create(temp_dir.path().join("file-with-dashes.txt")).unwrap();

    let files = get_files(temp_dir.path(), false);

    assert_eq!(files.len(), 2);
    assert!(files.contains(&"file with spaces.txt".to_string()));
    assert!(files.contains(&"file-with-dashes.txt".to_string()));
}
