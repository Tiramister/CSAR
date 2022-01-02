use super::Structure;
use crate::{arms::Weights, util::graph::Graph};
use std::collections::VecDeque;

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

impl Structure for CircuitMatroid {
    fn get_indices(&self) -> &Vec<usize> {
        &self.indices
    }

    fn contract_by_arm(&mut self, i: usize) -> &mut Self {
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

    /// To be implemented.
    fn fast_maxgap(&self, weights: &Weights) -> usize {
        assert_eq!(self.get_indices().len(), weights.len());

        let mapped_weights: Weights = self.get_indices().iter().map(|&i| weights[i]).collect();
        let mst = self.graph.maximum_spanning_tree(&mapped_weights).unwrap();

        let m = self.get_indices().len();
        let mut in_mst = vec![false; m];
        for &i in &mst {
            in_mst[i] = true;
        }

        let reachability_graph = self.graph.reachability_graph(&mst);
        let rvnum = reachability_graph.get_vnum();

        // The bidirectional adjacency graphs of the reachability graph
        let mut in_to_out_graph = vec![Vec::<usize>::new(); rvnum];
        let mut out_to_in_graph = vec![Vec::<usize>::new(); rvnum];
        for (u, v) in reachability_graph.get_edges() {
            in_to_out_graph[u].push(v);
            out_to_in_graph[v].push(u);
        }

        let mut gaps = vec![0_f64; m];

        // Edges in MST
        // Accept the maximum weight edge not in MST.
        // out -> in with the max operation.
        {
            let mut queue = VecDeque::new();
            let mut degrees: Vec<usize> = in_to_out_graph.iter().map(|vs| vs.len()).collect();

            let mut max_weights = vec![f64::NEG_INFINITY; rvnum];
            for i in 0..m {
                if !in_mst[i] {
                    assert_eq!(degrees[i], 0);
                    max_weights[i] = weights[i];
                    queue.push_back(i);
                }
            }

            while let Some(v) = queue.pop_front() {
                for &u in &out_to_in_graph[v] {
                    max_weights[u] = max_weights[u].max(max_weights[v]);
                    degrees[u] -= 1;
                    if degrees[u] == 0 {
                        queue.push_back(u);
                    }
                }
            }

            for i in 0..m {
                if in_mst[i] {
                    gaps[i] = weights[i] - max_weights[i];
                }
            }
        }

        // Edges not in MST
        // Reject the minimum weight edge in MST.
        // in -> out with the min operation.
        {
            let mut queue = VecDeque::new();
            let mut degrees: Vec<usize> = out_to_in_graph.iter().map(|vs| vs.len()).collect();

            let mut min_weights = vec![f64::INFINITY; rvnum];
            for i in 0..m {
                if in_mst[i] {
                    assert_eq!(degrees[i], 0);
                    min_weights[i] = weights[i];
                    queue.push_back(i);
                }
            }

            while let Some(v) = queue.pop_front() {
                for &u in &in_to_out_graph[v] {
                    min_weights[u] = min_weights[u].min(min_weights[v]);
                    degrees[u] -= 1;
                    if degrees[u] == 0 {
                        queue.push_back(u);
                    }
                }
            }

            for i in 0..m {
                if !in_mst[i] {
                    gaps[i] = min_weights[i] - weights[i];
                }
            }
        }

        println!("fast gap: {:?}", gaps);

        let (max_i, _max_gap) = gaps
            .iter()
            .enumerate()
            .max_by(|(_, l_gap), (_, r_gap)| l_gap.partial_cmp(r_gap).unwrap())
            .unwrap();
        max_i
    }
}

#[cfg(test)]
mod tests {
    use std::mem::swap;

    use itertools::iproduct;
    use rand::Rng;

    use crate::{
        arms::{Arms, Weights},
        csar::{csar, naive_maxgap},
        structures::{circuit_matroid::CircuitMatroid, Structure},
        util::{graph::Graph, union_find::UnionFind},
    };

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
