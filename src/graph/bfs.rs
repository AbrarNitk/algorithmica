use crate::graph::Graph;
use crate::search::binary::search;

pub struct BFS<'a> {
    source: usize,
    visited: Vec<bool>,
    parent: Vec<usize>,
    dist: Vec<usize>,
    graph: &'a Graph,
}

impl<'a> BFS<'a> {
    pub fn new(source: usize, graph: &'a Graph) -> Self {
        let mut new = Self {
            source,
            visited: vec![false; graph.v],
            parent: vec![0; graph.v],
            dist: vec![0; graph.v],
            graph,
        };
        new.bfs(source);
        new
    }

    pub fn has_path(&self, w: usize) -> bool {
        self.visited[w]
    }

    pub fn dist(&self, w: usize) -> usize {
        self.dist[w]
    }

    fn bfs(&mut self, source: usize) {
        let mut queue = Vec::new();
        queue.push(source);
        self.visited[source] = true;
        self.dist[source] = 0;
        self.parent[source] = 0; // TODO: Fix Me
        while !queue.is_empty() {
            let front = queue.first().unwrap();
            for w in self.graph.adj(*front).iter() {
                if !self.visited[*w] {
                    self.visited[*w] = true;
                    self.dist[source] = self.dist[*front] + 1;
                    self.parent[source] = *front;
                }
            }
        }
    }
}
