# Red-Black Binary Search Tree

## How to use
```rust
use redblackbst;
fn how_use() {
    let mut bst = redblackbst::RedBlackBST::new();
    bst.put(3, "1");
    bst.put(2, "2");
    bst.put(1, "3");
    assert_eq!(bst.size(), 3);
    assert_eq!(bst.get(1), Some(&"3"));
    assert_eq!(bst.get(4), None);
}
```

## Functions added in version 0.1.1
1. height
2. is_empty
3. min
4. max