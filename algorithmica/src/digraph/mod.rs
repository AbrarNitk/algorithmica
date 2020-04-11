use crate::graph::Graph;

pub struct DiDFS<'a> {
    source: usize,
    visited: Vec<bool>,
    graph: &'a Graph,
}

impl<'a> DiDFS<'a> {
    pub fn new(source: usize, graph: &'a Graph) -> Self {
        let mut new = DiDFS {
            source,
            visited: vec![false; graph.v],
            graph,
        };
        new.dfs(source);
        new
    }

    pub fn has_path(&self, w: usize) -> bool {
        self.visited[w]
    }

    fn dfs(&mut self, source: usize) {
        self.visited[source] = true;
        for w in self.graph.adj(source) {
            if !self.visited[*w] {
                self.dfs(*w);
            }
        }
    }
}
