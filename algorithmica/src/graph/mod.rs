use std::fs::File;
use std::io::{BufRead, BufReader};
pub mod bfs;
pub mod depth_first_paths;

#[derive(Debug)]
pub struct Graph {
    pub v: usize,               // number of all nodes
    pub e: usize,               // number of all edges
    pub nodes: Vec<Vec<usize>>, // adjacency list for every which will
}

impl Graph {
    pub fn new(v: usize) -> Self {
        let mut nodes_vec = Vec::new();
        for _ in 0..v {
            nodes_vec.push(Vec::new())
        }
        Self {
            v,
            e: 0,
            nodes: nodes_vec,
        }
    }

    pub fn add_edge(&mut self, u: usize, w: usize) {
        self.e += 1;
        self.nodes[u].push(w);
        self.nodes[w].push(u);
    }

    pub fn degree(&self, node: usize) -> usize {
        self.adj(node).len()
    }

    pub fn is_edge(&self, u: usize, w: usize) -> bool {
        self.adj(u).contains(&w)
    }

    pub(crate) fn adj(&self, node: usize) -> &Vec<usize> {
        self.nodes.get(node).unwrap()
    }

    #[allow(dead_code)]
    pub(crate) fn read_from_file(path: &str) -> Self {
        let f1: BufReader<File> = BufReader::new(File::open(path).expect("Not able to read file"));
        let mut it = f1.lines();
        let n = it.next().unwrap().unwrap().parse::<usize>().unwrap();
        let mut graph = Graph::new(n);
        for line in it.map(|line| {
            let l: Vec<usize> = line
                .unwrap()
                .trim()
                .splitn(2, ' ')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (l[0], l[1])
        }) {
            graph.add_edge(line.0, line.1);
        }
        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_file() {
        let path = "./src/graph/graph.txt";
        let g = Graph::read_from_file(path);
        let dfs = depth_first_paths::DepthFirstPaths::new(0, &g);
        println!(
            "Has Path :: {:?}, Path:: {:?}",
            dfs.has_path(7),
            dfs.path(7)
        );
    }
}
/*
input type
1. Number of nodes (n)
2. Adjacency list (Relation ship) (u <-> w because undirected)
*/
