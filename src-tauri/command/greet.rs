#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("hello, {}", name)
}

#[test]
fn test_greet() {
    assert_eq!(greet("a"), "hello, a".to_string());
}
