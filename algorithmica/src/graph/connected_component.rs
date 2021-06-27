pub struct ConnectedComponent<'a> {
    visited: Vec<bool>,
    graph: &'a crate::graph::Graph,
    count: usize,
    component: Vec<usize>,
}

impl<'a> ConnectedComponent<'a> {
    pub fn new(graph: &'a crate::graph::Graph) -> Self {
        let mut cc = Self {
            visited: vec![false; graph.v],
            count: 0,
            component: vec![0; graph.v],
            graph,
        };

        for x in 0..graph.v {
            if !cc.visited[x] {
                cc.dfs(x);
                cc.count += 1;
            }
        }
        cc
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn id(&self, v: usize) -> usize {
        self.component[v]
    }

    fn dfs(&mut self, v: usize) {
        self.visited[v] = true;
        for x in self.graph.adj(v) {
            if !self.visited[*x] {
                self.dfs(*x);
                self.component[*x] = self.count;
            }
        }
    }
}
