
pub struct Node<T> {
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

    pub fn prepend(&self, data: T) -> Stack<T> {
        Stack {
            head: Some(std::rc::Rc::new(Node {
                data,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Stack<T> {
        Stack {
            head: self.head.as_ref().and_then(|x| x.next.clone()),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.data)
    }

    pub fn head(&self) -> Option<&Node<T>> {
        self.head.as_deref()
    }

}
