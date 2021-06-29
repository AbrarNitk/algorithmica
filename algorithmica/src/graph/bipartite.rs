pub struct Bipartite<'a> {
    is_bipartite: bool,
    edge_to: Vec<usize>,
    visited: Vec<bool>,
    color: Vec<bool>,
    graph: &'a crate::graph::Graph,
}

impl<'a> Bipartite<'a> {
    pub fn new(graph: &'a crate::graph::Graph, source: usize) -> Self {
        let mut bipartite = Self {
            is_bipartite: true,
            edge_to: vec![0; graph.v],
            visited: vec![false; graph.v],
            color: vec![false; graph.v],
            graph,
        };

        for w in graph.adj(source) {
            if !bipartite.visited[*w] {
                bipartite.dfs(*w);
            }
        }
        bipartite
    }

    pub fn is_bipartite(&self) -> bool {
        self.is_bipartite
    }

    fn dfs(&mut self, w: usize) {
        self.visited[w] = true;
        for v in self.graph.adj(w) {
            if !self.visited[*v] {
                self.edge_to[*v] = w;
                self.visited[*v] = true;
                self.color[*v] = !self.color[w];
                self.dfs(*v);
            }
            // If there is an odd cycle occur in the graph, means graph is not bipartite
            if self.color[*v] == self.color[w] {
                self.is_bipartite = false;
            }
        }
    }
}
