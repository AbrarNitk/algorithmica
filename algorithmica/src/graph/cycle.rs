pub struct Cycle {
    visited: Vec<bool>,
    edge_to: Vec<usize>,
    is_cycle: bool,
    cycle: Vec<usize>,
}

impl Cycle {
    pub fn new(graph: &crate::graph::Graph) -> Self {
        let mut cycle = Self {
            visited: vec![false; graph.v],
            edge_to: vec![0; graph.v],
            is_cycle: false,
            cycle: vec![],
        };

        cycle.dfs(graph, 0);
        cycle
    }

    pub fn is_cycle(&self) -> bool {
        self.is_cycle
    }

    pub fn cycle(&self) -> &Vec<usize> {
        self.cycle.as_ref()
    }

    fn dfs(&mut self, graph: &crate::graph::Graph, w: usize) {
        if self.is_cycle {
            return;
        }

        self.visited[w] = true;
        for v in graph.adj(w) {
            if !self.visited[*v] {
                self.edge_to[*v] = w;
                self.dfs(graph, *v)
            }

            // find the cycle also
            if *v != w {
                self.is_cycle = true;
            }
        }
    }
}
