use crate::graph::Graph;

#[derive(Debug)]
pub struct DepthFirstPaths<'a> {
    source: usize,
    visited: Vec<bool>,
    parent: Vec<usize>,
    graph: &'a Graph,
}

impl<'a> DepthFirstPaths<'a> {
    pub fn new(source: usize, graph: &'a Graph) -> Self {
        let visited = vec![false; graph.v];
        let parent = vec![0; graph.v];
        let mut container = Self {
            source,
            graph,
            visited,
            parent,
        };
        container.dfs(source);
        container
    }
    pub fn has_path(&self, w: usize) -> bool {
        self.visited[w]
    }

    pub fn path(&self, w: usize) -> Vec<usize> {
        if !self.has_path(w) {
            return vec![];
        }

        let mut path_stack = Vec::new();
        path_stack.push(w);
        let mut w = w;
        while w != self.source {
            w = self.parent[w];
            path_stack.push(w);
        }
        path_stack.push(self.source);
        path_stack.reverse();
        path_stack
    }

    fn dfs(&mut self, source: usize) {
        self.visited[source] = true;
        for v in self.graph.adj(source) {
            if !self.visited[*v] {
                self.parent[*v] = source;
                self.dfs(*v);
            }
        }
    }
}
