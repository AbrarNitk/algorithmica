struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<std::rc::Rc<Node<T>>>;

pub struct Stack<T> {
    head: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { head: None }
    }
}
