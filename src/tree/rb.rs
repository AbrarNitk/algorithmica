
pub enum Color {
    Black,
    Red,
}

pub struct RBTree {
    pub value: i32,
    pub color: Color,
    pub left: Box<RBTree>,
    pub right: Box<RBTree>
}