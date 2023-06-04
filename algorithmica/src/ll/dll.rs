type Link<Item> = *mut Node<Item>;
pub struct Node<Item> {
    prev: Link<Item>,
    value: Item,
    next: Link<Item>,
}

pub struct LinkList<Item> {
    head: Link<Item>,
    tail: Link<Item>,
    len: usize,
}
