use gatkeeper_parser::parse_file;
use std::path::Path;

#[test]
fn test_parse_rust_file() {
    let path = Path::new("src/lib.rs");
    if path.exists() {
        let parsed = parse_file(path).unwrap();
        assert_eq!(parsed.language, "rust");
        assert!(parsed.line_count > 0);
    }
}

#[test]
fn test_parse_python_file() {
    let dir = std::env::temp_dir().join("gatkeeper_test_parse_py");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("test.py");
    std::fs::write(&file, "def hello():\n    print('hi')\n").unwrap();

    let parsed = parse_file(&file).unwrap();
    assert_eq!(parsed.language, "python");
    assert!(!parsed.functions.is_empty());
    assert_eq!(parsed.functions[0].name, "hello");

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_parse_js_file() {
    let dir = std::env::temp_dir().join("gatkeeper_test_parse_js");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("test.js");
    std::fs::write(&file, "function hello() { return 42; }").unwrap();

    let parsed = parse_file(&file).unwrap();
    assert_eq!(parsed.language, "javascript");

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_unsupported_extension() {
    let dir = std::env::temp_dir().join("gatkeeper_test_parse_unsup");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("test.xyz");
    std::fs::write(&file, "hello").unwrap();

    let result = parse_file(&file);
    assert!(result.is_err());

    std::fs::remove_dir_all(&dir).unwrap();
}
