pub struct MaxPQ {
    q: Vec<i32>,
    n: usize,
}

impl MaxPQ {
    pub fn from_capacity(capacity: usize) -> Self {
        Self {
            q: Vec::with_capacity(capacity + 1),
            n: 0,
        }
    }

    pub fn from() -> Self {
        Self::from_capacity(1)
    }

    pub fn del_max(&self) -> i32 {
        0
    }

    pub fn insert(&mut self, value: i32) {
        self.n += 1;
        self.q.insert(self.n, value);
        self.swim(self.n)
    }

    fn sink(&mut self, mut k: usize) {
        while 2 * k <= self.n {
            let mut j = 2 * k;
            if j < self.n && self.less(j, j + 1) {
                j += 1
            }
            if !self.less(k, j) {
                break;
            }
            self.exchange(k, j);
            k = j;
        }
    }

    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.less(k / 2, 2) {
            self.exchange(k / 2, k);
            k /= 2;
        }
    }

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
