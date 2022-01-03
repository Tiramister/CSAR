pub mod circuit_matroid;
pub mod uniform_matroid;

use std::collections::VecDeque;

use crate::{arms::Weights, util::graph::Graph};

pub trait CombinatorialStructure: Clone {
    /// Get indices of the remaining arms.
    fn get_indices(&self) -> &Vec<usize>;

    /// Contract the arm i.
    /// Assume that the arm i remains.
    fn contract_arm(&mut self, i: usize) -> &mut Self;

    /// Delete the arm i.
    /// Assume that the arm i remains.
    fn delete_arm(&mut self, i: usize) -> &mut Self;

    /// Find the superarm with the maximum sum of weights.
    fn optimal(&self, weights: &Weights) -> Option<Vec<usize>>;

    /// Build a directed acyclic graph satisfying the following properties.
    ///
    /// * Every elements in this matroid corresponds to a vertex in DAG.
    /// * Every vertex corresponding to an element *in* the basis has no incoming edge.
    /// * Every vertex corresponding to an element *not in* the basis has no outgoing edge.
    /// * There is a path from an element *in* the basis to an element *not in* the basis
    ///     iff the former is in the fundamental circuit of the basis and the latter.
    ///
    /// It is required that `basis` induces a basis.
    fn reachability_graph(&self, basis: &Vec<usize>) -> Graph;

    /// Efficiently find the arm with the maximum gap.
    fn fast_maxgap(&self, weights: &Weights) -> usize {
        assert_eq!(self.get_indices().len(), weights.len());

        let mapped_weights: Weights = self.get_indices().iter().map(|&i| weights[i]).collect();
        let opt_basis = self.optimal(&mapped_weights).unwrap();

        let m = self.get_indices().len();
        let mut in_opt = vec![false; m];
        for &i in &opt_basis {
            in_opt[i] = true;
        }

        let reachability_graph = self.reachability_graph(&opt_basis);
        let rvnum = reachability_graph.get_vnum();

        // The bidirectional adjacency lists of the reachability graph.
        let mut in_to_out_graph = vec![Vec::<usize>::new(); rvnum];
        let mut out_to_in_graph = vec![Vec::<usize>::new(); rvnum];
        for (u, v) in reachability_graph.get_edges() {
            in_to_out_graph[u].push(v);
            out_to_in_graph[v].push(u);
        }

        let mut gaps = vec![f64::NAN; m];

        // The gaps of edges in the basis.
        // out -> in with the max operation.
        {
            let mut queue = VecDeque::new();
            let mut degrees: Vec<usize> = in_to_out_graph.iter().map(|vs| vs.len()).collect();

            let mut max_weights = vec![f64::NEG_INFINITY; rvnum];
            for i in 0..m {
                if !in_opt[i] {
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
                if in_opt[i] {
                    gaps[i] = weights[i] - max_weights[i];
                }
            }
        }

        // The gaps of edges not in the basis.
        // in -> out with the min operation.
        {
            let mut queue = VecDeque::new();
            let mut degrees: Vec<usize> = out_to_in_graph.iter().map(|vs| vs.len()).collect();

            let mut min_weights = vec![f64::INFINITY; rvnum];
            for i in 0..m {
                if in_opt[i] {
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
                if !in_opt[i] {
                    gaps[i] = min_weights[i] - weights[i];
                }
            }
        }

        eprintln!("fast gaps: {:?}", gaps);

        // Return the index with the maximum gap.
        let (max_i, _max_gap) = gaps
            .iter()
            .enumerate()
            .max_by(|(_, l_gap), (_, r_gap)| l_gap.partial_cmp(r_gap).unwrap())
            .unwrap();
        max_i
    }
}
