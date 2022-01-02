use std::{
    cmp::{max, min},
    collections::VecDeque,
    mem::swap,
};

use crate::{arms::Weights, util::union_find::UnionFind};

#[derive(Clone)]
pub struct Graph {
    edges: Vec<(usize, usize)>,
    vnum: usize,
}

impl Graph {
    pub fn new(vnum: usize) -> Self {
        Self {
            edges: Vec::new(),
            vnum,
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) -> &mut Self {
        self.edges.push((u, v));
        self.vnum = max(self.vnum, max(u, v) + 1);
        self
    }

    pub fn from_edges(_edges: &Vec<(usize, usize)>) -> Self {
        Self {
            edges: _edges.clone(),
            vnum: _edges.iter().map(|&(u, v)| max(u, v)).max().unwrap_or(0) + 1,
        }
    }

    pub fn get_vnum(&self) -> usize {
        self.vnum
    }

    pub fn get_edges(&self) -> Vec<(usize, usize)> {
        self.edges.clone()
    }

    /// Contract this graph by the i-th edge.
    /// You can keep the indices of edges by swap_remove(i).
    pub fn contract_by_edge(&mut self, i: usize) -> &mut Self {
        let (mut s, mut t) = self.edges[i];

        if s > t {
            swap(&mut s, &mut t);
        }
        // s <= t

        for (u, v) in self.edges.iter_mut() {
            // Move t to s
            if *u == t {
                *u = s;
            }
            // Delete t
            if *u >= t && *u > 0 {
                *u -= 1;
            }

            if *v == t {
                *v = s;
            }
            if *v >= t && *v > 0 {
                *v -= 1;
            }
        }

        // Delete the i-th edge
        self.edges.swap_remove(i);
        self.vnum -= 1;

        self
    }

    /// Delete the i-th edge.
    /// You can keep the indices of edges by swap_remove(i).
    pub fn delete_edge(&mut self, i: usize) -> &mut Self {
        self.edges.swap_remove(i);
        self
    }

    /// Find the maximum spanning tree by the Kruskal's algorithm
    /// It is required that the length of `weights` equals the number of edges.
    pub fn maximum_spanning_tree(&self, weights: &Weights) -> Option<Vec<usize>> {
        // Check the requirement for the length of `weights`.
        assert_eq!(self.edges.len(), weights.len());

        if self.vnum == 0 {
            return None;
        }

        // Sort indices by weights in decreasing order
        let mut indices: Vec<usize> = (0..self.edges.len()).collect();
        indices.sort_unstable_by(|&i, &j| weights[i].partial_cmp(&weights[j]).unwrap().reverse());

        // Add the heaviest edge greedily if it doesn't induce any cycle.
        let mut mst = Vec::<usize>::new();
        let mut uf = UnionFind::new(self.vnum);
        for i in indices {
            let (u, v) = self.edges[i];
            if !uf.same(u, v) {
                uf.unite(u, v);
                mst.push(i);
            }
        }
        Some(mst)
    }

    /// Build a directed acyclic graph satisfying the following properties.
    /// * Every edge in this graph corresponds to a vertex in DAG.
    /// * Every vertex corresponding to an edge *in* the given tree has no incoming edge.
    /// * Every vertex corresponding to an edge *not in* the given tree has no outgoing edge.
    /// * For every edge *not in* the given tree and an edge in its fundamental circuit, the former is reachable from latter.
    /// It is required that `tree_edges` induces a tree.
    pub fn reachability_graph(&self, tree_edges: &Vec<usize>) -> Graph {
        let mut result_graph = Graph::new(self.edges.len());

        // Build the adjacency list.
        let mut adj = vec![Vec::<usize>::new(); self.vnum];
        for &i in tree_edges {
            let (u, v) = self.edges[i];
            adj[u].push(i);
            adj[v].push(i);
        }

        // Make rooted from the vertex 0.
        // The depth of each vertex.
        let mut depth = vec![0_usize; self.vnum];
        // Store the parent and the edge to the parent.
        let mut par_vertex = vec![0_usize; self.vnum];
        let mut par_edge = vec![0_usize; self.vnum];

        // Run BFS.
        {
            let mut queue = VecDeque::<usize>::new();
            let mut visited = vec![false; self.vnum];

            // Start from the vertex 0.
            queue.push_back(0);
            depth[0] = 0;

            while let Some(p) = queue.pop_front() {
                visited[p] = true;

                for &i in &adj[p] {
                    let (u, v) = self.edges[i];
                    // The other vertex.
                    let q = p ^ u ^ v;

                    if !visited[q] {
                        // Set p as the parent of q.
                        depth[q] = depth[p] + 1;
                        par_vertex[q] = p;
                        par_edge[q] = i;

                        queue.push_back(q);
                    }
                }
            }
        }

        // Doubling.
        let mut doubled_vertex: Vec<Vec<usize>> = par_vertex.iter().map(|&v| vec![v]).collect();
        let mut doubled_edge: Vec<Vec<usize>> = par_edge.iter().map(|&e| vec![e]).collect();
        doubled_vertex[0].pop();
        doubled_edge[0].pop();

        for k in 0.. {
            let mut updated = false;

            for u in 0..self.vnum {
                if let Some(&v) = doubled_vertex[u].get(k) {
                    let uv = doubled_edge[u][k];

                    if let Some(&w) = doubled_vertex[v].get(k) {
                        let vw = doubled_edge[v][k];
                        // u -> v -> w

                        // Create new vertex corresponding to the path uw.
                        let uw = result_graph.vnum;
                        result_graph.add_edge(uv, uw).add_edge(vw, uw);

                        doubled_vertex[u].push(w);
                        doubled_edge[u].push(uw);
                        updated = true;
                    }
                }
            }

            if !updated {
                break;
            }
        }

        let mut in_tree = vec![false; self.edges.len()];
        for &i in tree_edges {
            in_tree[i] = true;
        }

        // Span edges to each vertex whose corresponding edge is not in the given tree.
        for i in 0..self.edges.len() {
            if in_tree[i] {
                continue;
            }

            let (mut u, mut v) = self.edges[i];

            // Equalize the depths of u and v
            while depth[u] != depth[v] {
                if depth[u] < depth[v] {
                    swap(&mut u, &mut v);
                }

                let k = (depth[u] - depth[v]).trailing_zeros() as usize;
                result_graph.add_edge(doubled_edge[u][k], i);
                u = doubled_vertex[u][k];
            }
            assert_eq!(depth[u], depth[v]);

            if u == v {
                continue;
            }

            let kmin = min(doubled_edge[u].len(), doubled_edge[v].len());
            for k in (0..kmin).rev() {
                let nu = doubled_vertex[u][k];
                let nv = doubled_vertex[v][k];

                // Climb 2^k edges if u and v don't come to equal.

                if nu != nv {
                    result_graph
                        .add_edge(doubled_edge[u][k], i)
                        .add_edge(doubled_edge[v][k], i);
                    u = nu;
                    v = nv;
                }
            }
            assert_ne!(u, v);

            // Climb the last one edge to make u and v equal.
            result_graph
                .add_edge(doubled_edge[u][0], i)
                .add_edge(doubled_edge[v][0], i);
            u = doubled_vertex[u][0];
            v = doubled_vertex[v][0];
            assert_eq!(u, v);
        }

        result_graph
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;
    use crate::util::union_find::UnionFind;
    use rand::Rng;
    use std::mem::swap;

    #[test]
    fn reachable_random() {
        let mut rng = rand::thread_rng();

        let n = 8;
        let m = 15;
        let mut graph = Graph::new(n);

        let mut tree_edges = Vec::<usize>::new();
        while graph.edges.len() != m {
            let mut u = rng.gen_range(0..n);
            let mut v = rng.gen_range(0..n);

            if u > v {
                swap(&mut u, &mut v);
            }
            if u != v && !graph.edges.contains(&(u, v)) {
                graph.add_edge(u, v);
            }
        }

        let mut uf = UnionFind::new(n);

        while tree_edges.len() != n - 1 {
            let i = rng.gen_range(0..m);
            let (u, v) = graph.edges[i];

            if !uf.same(u, v) {
                uf.unite(u, v);
                tree_edges.push(i);
            }
        }

        println!("tree:   {:?}", &tree_edges);
        println!("graph:  {:?}", &graph.edges);

        let rgraph = graph.reachability_graph(&tree_edges);

        println!("rgraph: {:?}", &rgraph.edges);
    }
}
