#[derive(Debug)]
pub struct Node {
    pub data: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn create(data: i32) -> Box<Self> {
        Box::new(Node {
            data,
            left: None,
            right: None,
        })
    }

    pub fn add_new(root: Option<Box<Node>>, data: i32) -> Option<Box<Self>> {
        match root {
            Some(mut node) => {
                if node.left.is_none() {
                    node.left = Node::add_new(node.left, data);
                } else {
                    node.right = Node::add_new(node.right, data);
                }
                return Some(node);
            }
            None => return Some(Node::create(data)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t() {
        let t1 = Node::add_new(None, 1);
        let t2 = Node::add_new(t1, 2);
        let t2 = Node::add_new(t2, 2);
        println!("{:?}", t2);
    }
}
