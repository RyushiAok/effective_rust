use effective_rust::item30::{add, greet};

#[test]
fn public_add_works() {
    assert_eq!(add(10, 20), 30);
}

#[test]
fn greeting_contains_name() {
    let s = greet("Alice");
    assert!(s.contains("Alice"));
}
