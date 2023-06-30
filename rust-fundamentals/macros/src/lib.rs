#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($element: expr) => {{
        let mut v = Vec::new();
        v.push($element);
        v
    }};
}

#[test]
fn test_empty() {
    let v: Vec<i32> = avec![];
    assert!(v.is_empty())
}

#[test]
fn test_single() {
    let v: Vec<i32> = avec![10];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 1);
}
