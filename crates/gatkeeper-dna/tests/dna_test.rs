use gatkeeper_dna::{DnaFingerprint, PatternDetector, InvariantExtractor};
use std::path::Path;

#[test]
fn test_fingerprint_compute() {
    let dir = std::env::temp_dir().join("gatkeeper_test_fp");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("a.rs"), "fn main() {}").unwrap();
    std::fs::write(dir.join("b.py"), "print('hello')").unwrap();

    let fp = DnaFingerprint::compute(&dir);
    assert_eq!(fp.file_count, 2);
    assert!(!fp.root_hash.is_empty());
    assert!(fp.total_lines > 0);

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_fingerprint_diff() {
    let dir1 = std::env::temp_dir().join("gatkeeper_test_fp1");
    let dir2 = std::env::temp_dir().join("gatkeeper_test_fp2");
    std::fs::create_dir_all(&dir1).unwrap();
    std::fs::create_dir_all(&dir2).unwrap();

    std::fs::write(dir1.join("a.rs"), "fn a() {}").unwrap();
    std::fs::write(dir2.join("a.rs"), "fn a() { let x = 1; }").unwrap();
    std::fs::write(dir2.join("b.rs"), "fn b() {}").unwrap();

    let fp1 = DnaFingerprint::compute(&dir1);
    let fp2 = DnaFingerprint::compute(&dir2);
    let diff = fp1.diff(&fp2);

    assert!(diff.has_changes());
    assert!(diff.modified.contains(&"a.rs".to_string()));
    assert!(diff.added.contains(&"b.rs".to_string()));

    std::fs::remove_dir_all(&dir1).unwrap();
    std::fs::remove_dir_all(&dir2).unwrap();
}

#[test]
fn test_pattern_detection() {
    let dir = std::env::temp_dir().join("gatkeeper_test_patterns");
    std::fs::create_dir_all(dir.join("models")).unwrap();
    std::fs::create_dir_all(dir.join("views")).unwrap();
    std::fs::create_dir_all(dir.join("controllers")).unwrap();
    std::fs::write(dir.join("models/user.rs"), "").unwrap();
    std::fs::write(dir.join("views/home.html"), "").unwrap();
    std::fs::write(dir.join("controllers/handler.rs"), "").unwrap();

    let patterns = PatternDetector::detect(&dir);
    assert!(patterns.iter().any(|p| p.name == "MVC"));

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_invariant_extraction() {
    let dir = std::env::temp_dir().join("gatkeeper_test_inv");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("main.rs"), "pub fn do_thing() -> Result<(), Error> { Ok(()) }").unwrap();

    let invariants = InvariantExtractor::extract(&dir);
    assert!(invariants.iter().any(|i| i.name == "Error propagation"));

    std::fs::remove_dir_all(&dir).unwrap();
}
