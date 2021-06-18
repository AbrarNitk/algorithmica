pub struct MaxPQ {
    q: Vec<i32>,
    n: usize,
}


impl MaxPQ {
    pub fn from_capacity(capacity: usize) -> Self {
        Self {
            q: Vec::with_capacity(capacity + 1),
            n: 0
        }
    }

    pub fn from() -> Self {
        Self::from_capacity(1)
    }



    pub fn del_max(&self) -> i32 {
        0
    }

    pub fn insert(&self, value: i32) {}

    fn sink(&self) {}

    fn swim(&self) {}

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn max(&self) -> i32 {
        // if is_empty throw exception
        self.q[0]
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.q[i].lt(&self.q[j])
    }

    fn exchange(&mut self, i: usize, j: usize) {
        self.q.swap(i, j)
    }

}