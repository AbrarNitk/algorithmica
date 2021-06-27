use crate::graph::Graph;

#[allow(clippy::upper_case_acronyms)]
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
        new.bfs();
        new
    }

    pub fn has_path(&self, w: usize) -> bool {
        self.visited[w]
    }

    pub fn dist(&self, w: usize) -> usize {
        self.dist[w]
    }

    fn bfs(&mut self) {
        let queue = vec![self.source];
        self.visited[self.source] = true;
        self.dist[self.source] = 0;
        self.parent[self.source] = 0; // TODO: Fix Me
        while !queue.is_empty() {
            let front = queue.first().unwrap();
            for w in self.graph.adj(*front).iter() {
                if !self.visited[*w] {
                    self.visited[*w] = true;
                    self.dist[*w] = self.dist[*front] + 1;
                    self.parent[*w] = *front;
                }
            }
        }
    }
}

// perl -le 'for(1..416){printf "%4d %4d\n", int(rand(4124))+1, int(rand(4124))+1}'
