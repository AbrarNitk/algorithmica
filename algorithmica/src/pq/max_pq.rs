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

    pub fn from_arr(v: Vec<i32>) -> Self {
        let n = v.len();
        let mut q = Vec::with_capacity(n + 1);
        q.resize_with(n + 1, Default::default);
        for (i, item) in v.into_iter().enumerate().take(n) {
            q.insert(i + 1, item);
        }
        let mut max_pq = Self { q, n };
        for i in (1..=n / 2).rev() {
            max_pq.sink(i);
        }
        max_pq
    }

    pub fn del_max(&mut self) -> i32 {
        let max = self.q[1];
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
        while k > 1 && self.less(k / 2, k) {
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
        self.q[1]
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.q[i].lt(&self.q[j])
    }

    fn exchange(&mut self, i: usize, j: usize) {
        self.q.swap(i, j)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        let mut pq = MaxPQ::from_arr(vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(pq.max(), 7);
        pq.del_max();
        assert_eq!(pq.max(), 6);
        pq.del_max();
        assert_eq!(pq.max(), 5);
        pq.insert(15);
        println!("{:?}", pq.q);
        assert_eq!(pq.max(), 15);
    }
}
