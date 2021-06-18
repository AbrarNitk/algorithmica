pub struct MaxPQ {
    q: Vec<i32>,
    n: usize,
}

impl Default for MaxPQ {
    fn default() -> Self {
        MaxPQ::from_capacity(1)
    }
}

impl MaxPQ {
    pub fn from_capacity(capacity: usize) -> Self {
        Self {
            q: Vec::with_capacity(capacity + 1),
            n: 0,
        }
    }

    pub fn from_arr(v: Vec<i32>)  -> Self {
        let n = v.len();
        let mut q = Vec::with_capacity(n+ 1);
        for i in 0..n {
            q[i + 1] = v[i];
        }
        let mut max_pq = Self { q, n };
        for i in (1..=n/2).rev() {
            max_pq.sink(i);
        }
        max_pq
    }

    pub fn del_max(&mut self) -> i32 {
        let max= self.q[1];
        self.exchange(1, self.n);
        self.n -= 1;
        self.sink(1);
        max
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
