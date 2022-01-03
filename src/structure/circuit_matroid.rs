use super::CombinatorialStructure;
use crate::{arms::Weights, util::graph::Graph};
use std::{collections::VecDeque, mem::swap};

#[derive(Clone)]
pub struct CircuitMatroid {
    indices: Vec<usize>,
    graph: Graph,
}

impl CircuitMatroid {
    pub fn new(graph: &Graph) -> Self {
        CircuitMatroid {
            indices: (0..graph.get_edges().len()).collect(),
            graph: graph.clone(),
        }
    }
}

impl CombinatorialStructure for CircuitMatroid {
    fn get_indices(&self) -> &Vec<usize> {
        &self.indices
    }

    fn contract_arm(&mut self, i: usize) -> &mut Self {
        let pos = self.get_indices().iter().position(|&r| r == i).unwrap();
        self.graph.contract_by_edge(pos);

        // Keep the order of edges
        self.indices.swap_remove(pos);

        self
    }

    fn delete_arm(&mut self, i: usize) -> &mut Self {
        let pos = self.get_indices().iter().position(|&r| r == i).unwrap();
        self.graph.delete_edge(pos);

        // Keep the order of edges
        self.indices.swap_remove(pos);

        self
    }

    fn optimal(&self, weights: &Weights) -> Option<Vec<usize>> {
        // Reorder weights by edge-indices.
        let mapped_weights: Weights = self.get_indices().iter().map(|&i| weights[i]).collect();

        if let Some(mst) = self.graph.maximum_spanning_tree(&mapped_weights) {
            // Convert edge-indices to arm-indices.
            Some(mst.iter().map(|&i| self.indices[i]).collect())
        } else {
            None
        }
    }

    fn reachability_graph(&self, tree_edges: &Vec<usize>) -> Graph {
        let graph = &self.graph;
        let m = graph.get_edges().len();

        let mut result_graph = Graph::new(m);

        // Build the adjacency list.
        let mut adj = vec![Vec::<usize>::new(); graph.get_vnum()];
        for &i in tree_edges {
            let (u, v) = graph.get_edges()[i];
            adj[u].push(i);
            adj[v].push(i);
        }

        // Make rooted from the vertex 0.

        // The depth of each vertex.
        let mut depth = vec![0_usize; graph.get_vnum()];
        // Store the parent and the edge to the parent.
        let mut par_vertex = vec![Vec::<usize>::new(); graph.get_vnum()];
        let mut par_edge = vec![Vec::<usize>::new(); graph.get_vnum()];

        // Run BFS.
        let mut queue = VecDeque::<usize>::new();
        let mut visited = vec![false; graph.get_vnum()];

        // Start from the vertex 0.
        queue.push_back(0);
        depth[0] = 0;

        while let Some(u) = queue.pop_front() {
            visited[u] = true;

            // Doubling.
            let mut k = 0;
            while 1 << (k + 1) <= depth[u] {
                eprintln!("k: {}", k);

                let v = par_vertex[u][k];
                let uv = par_edge[u][k];

                eprintln!("{} -> {}", u, v);

                let w = par_vertex[v][k];
                let vw = par_edge[v][k];

                eprintln!("{} -> {}", v, w);
                // Compress the path u -> v -> w.

                // Create new vertex corresponding to the path uw.
                let uw = result_graph.get_vnum();
                result_graph.add_edge(uv, uw).add_edge(vw, uw);

                par_vertex[u].push(w);
                par_edge[u].push(uw);
                k += 1;
            }

            for &i in &adj[u] {
                let (p, q) = graph.get_edges()[i];
                // The other vertex.
                let v = u ^ p ^ q;

                if !visited[v] {
                    // Set p as the parent of q.
                    depth[v] = depth[u] + 1;
                    par_vertex[v].push(u);
                    par_edge[v].push(i);
                    queue.push_back(v);
                }
            }
        }

        let mut in_tree = vec![false; self.graph.get_edges().len()];
        for &i in tree_edges {
            in_tree[i] = true;
        }

        // Span edges to each vertex whose corresponding edge is not in the given tree.
        for i in 0..graph.get_edges().len() {
            if in_tree[i] {
                continue;
            }

            let (mut u, mut v) = graph.get_edges()[i];

            // Equalize the depths of u and v
            while depth[u] != depth[v] {
                if depth[u] < depth[v] {
                    swap(&mut u, &mut v);
                }

                let k = (depth[u] - depth[v]).trailing_zeros() as usize;
                result_graph.add_edge(par_edge[u][k], i);
                u = par_vertex[u][k];
            }
            assert_eq!(depth[u], depth[v]);

            if u == v {
                continue;
            }

            assert_eq!(par_edge[u].len(), par_edge[v].len());
            let kmax = par_edge[u].len();
            for k in (0..kmax).rev() {
                let nu = par_vertex[u][k];
                let nv = par_vertex[v][k];

                // Climb 2^k edges if u and v don't come to equal.
                if nu != nv {
                    result_graph
                        .add_edge(par_edge[u][k], i)
                        .add_edge(par_edge[v][k], i);
                    u = nu;
                    v = nv;
                }
            }
            assert_ne!(u, v);

            // Climb the last one edge to make u and v equal.
            result_graph
                .add_edge(par_edge[u][0], i)
                .add_edge(par_edge[v][0], i);
            u = par_vertex[u][0];
            v = par_vertex[v][0];
            assert_eq!(u, v);
        }

        result_graph
    }
}

#[cfg(test)]
mod tests {
    use std::mem::swap;

    use itertools::iproduct;
    use rand::Rng;

    use crate::{
        algorithm::{csar, naive_maxgap},
        arms::{Arms, Weights},
        structure::{circuit_matroid::CircuitMatroid, CombinatorialStructure},
        util::{graph::Graph, union_find::UnionFind},
    };

    #[test]
    fn reachable_random() {
        let mut rng = rand::thread_rng();

        let n = 8;
        let m = 15;
        let mut graph = Graph::new(n);

        let mut tree_edges = Vec::<usize>::new();
        while graph.get_edges().len() != m {
            let mut u = rng.gen_range(0..n);
            let mut v = rng.gen_range(0..n);

            if u > v {
                swap(&mut u, &mut v);
            }
            if u != v && !graph.get_edges().contains(&(u, v)) {
                graph.add_edge(u, v);
            }
        }

        let mut uf = UnionFind::new(n);

        while tree_edges.len() != n - 1 {
            let i = rng.gen_range(0..m);
            let (u, v) = graph.get_edges()[i];

            if !uf.same(u, v) {
                uf.unite(u, v);
                tree_edges.push(i);
            }
        }

        println!("tree:   {:?}", &tree_edges);
        println!("graph:  {:?}", &graph.get_edges());

        let structure = CircuitMatroid::new(&graph);
        let rgraph = structure.reachability_graph(&tree_edges);

        println!("rgraph: {:?}", &rgraph.get_edges());
    }

    fn test_maxgap_once(n: usize) {
        let mut rng = rand::thread_rng();

        // Generate a connected graph randomly.
        let mut graph = Graph::new(n);

        {
            let mut uf = UnionFind::new(n);
            while uf.get_size(0) != n || graph.get_edges().len() < n * 2 {
                let mut u = rng.gen_range(0..n);
                let mut v = rng.gen_range(0..n);

                if u > v {
                    swap(&mut u, &mut v);
                }
                if u != v && !graph.get_edges().contains(&(u, v)) {
                    uf.unite(u, v);
                    graph.add_edge(u, v);
                }
            }
        }

        let m = graph.get_edges().len();
        let structure = CircuitMatroid::new(&graph);

        // Generate weights randomly.
        let weights: Weights = (0..m).map(|_| rng.gen()).collect();

        println!("graph:   {:?}", graph.get_edges());
        println!("weights: {:?}", weights);

        // Find the edge with the maximum gap.
        let naive_arm = naive_maxgap(&structure, &weights);
        let faster_arm = structure.fast_maxgap(&weights);

        println!("naive: {}, faster: {}", naive_arm, faster_arm);
        assert!(naive_arm == faster_arm);
    }

    #[test]
    fn test_maxgap() {
        for _ in 0..10 {
            test_maxgap_once(5);
        }
    }

    fn test_csar_once(n: usize) {
        // Generate the complete graph.
        let edges: Vec<(usize, usize)> =
            iproduct!((0..n), (0..n)).filter(|&(x, y)| x < y).collect();
        let graph = Graph::from_edges(&edges);
        let structure = CircuitMatroid::new(&graph);

        // generate arms randomly
        let mut arms = Arms::new();
        let mut rng = rand::thread_rng();
        for _ in 0..edges.len() {
            arms.add_arm(rng.gen(), rng.gen());
        }

        let mut csar_optimal = csar(structure.clone(), &mut arms);
        csar_optimal.sort();

        let means: Weights = structure
            .get_indices()
            .iter()
            .map(|&i| arms.get_mean(i))
            .collect();

        let mut true_optimal = structure.optimal(&means).unwrap();
        true_optimal.sort();

        println!("csar: {:?}", csar_optimal);
        println!("true: {:?}", true_optimal);
        println!("----------");
    }

    #[test]
    fn test_csar() {
        for _ in 0..10 {
            test_csar_once(10);
        }
    }
}
