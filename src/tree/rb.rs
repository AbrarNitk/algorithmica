
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

impl RBTree {
    pub fn new(){}
    pub fn left_rotate(){}
    pub fn right_rotate(){}
    pub fn change_color(){}
}