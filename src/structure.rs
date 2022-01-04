pub mod circuit_matroid;
pub mod uniform_matroid;

use crate::util::graph::Graph;
use std::collections::VecDeque;

pub trait CombinatorialStructure: Clone {
    /// Get the number of arms.
    fn get_arm_num(&self) -> usize;

    /// Get indices of the remaining arms.
    fn get_arms(&self) -> Vec<usize>;

    /// Contract the arm i.
    /// Assume that the arm i remains.
    fn contract_arm(&mut self, i: usize) -> &mut Self;

    /// Delete the arm i.
    /// Assume that the arm i remains.
    fn delete_arm(&mut self, i: usize) -> &mut Self;

    /// Find the superarm with the maximum sum of weights.
    fn optimal(&self, weights: &[f64]) -> Option<Vec<usize>>;

    /// Build a directed acyclic graph satisfying the following properties.
    ///
    /// * Every arm corresponds to a vertex in DAG.
    /// * There is a path from an arm *in* the basis to an arm *not in* the basis
    ///     iff the former arm is in the fundamental circuit of the latter arm.
    ///
    /// It is required that `basis` induces a basis.
    fn reachability_graph(&self, basis: &[usize]) -> Graph;

    /// Efficiently find the arm with the maximum gap.
    fn fast_maxgap(&self, weights: &[f64]) -> usize {
        let arm_num = self.get_arm_num();

        // Find the optimal basis.
        let opt_basis = self.optimal(weights).unwrap();

        // Whether or not an arm is in the optimal basis.
        let mut in_opt = vec![false; arm_num];
        for &arm in &opt_basis {
            in_opt[arm] = true;
        }

        // Build the reachability graph.
        let reachability_graph = self.reachability_graph(&opt_basis);
        let rvnum = reachability_graph.get_vnum();

        // Adjacency lists of the reachability graph.
        let mut adj_from_basis = vec![Vec::<usize>::new(); rvnum];
        let mut adj_to_basis = vec![Vec::<usize>::new(); rvnum];
        for (u, v) in reachability_graph.get_edges() {
            adj_from_basis[u].push(v);
            adj_to_basis[v].push(u);
        }

        let mut gaps = vec![f64::NAN; arm_num];

        // The gaps of edges in the basis.
        // To the basis with the max operation.
        {
            let mut queue = VecDeque::new();
            let mut indegrees: Vec<usize> = adj_from_basis.iter().map(|vs| vs.len()).collect();

            let mut max_weights = vec![f64::NEG_INFINITY; rvnum];
            for v in 0..rvnum {
                if indegrees[v] == 0 {
                    if v < arm_num {
                        max_weights[v] = weights[v];
                    }
                    queue.push_back(v);
                }
            }

            while let Some(v) = queue.pop_front() {
                for &u in &adj_to_basis[v] {
                    max_weights[u] = max_weights[u].max(max_weights[v]);
                    indegrees[u] -= 1;
                    if indegrees[u] == 0 {
                        queue.push_back(u);
                    }
                }
            }

            assert!(indegrees.iter().all(|&d| d == 0), "BFS error (to basis)");

            for i in 0..arm_num {
                if in_opt[i] {
                    gaps[i] = weights[i] - max_weights[i];
                }
            }
        }

        // The gaps of edges not in the basis.
        // From the basis with the min operation.
        {
            let mut queue = VecDeque::new();
            let mut indegrees: Vec<usize> = adj_to_basis.iter().map(|vs| vs.len()).collect();

            let mut min_weights = vec![f64::INFINITY; rvnum];
            for v in 0..rvnum {
                if indegrees[v] == 0 {
                    if v < arm_num {
                        min_weights[v] = weights[v];
                    }
                    queue.push_back(v);
                }
            }

            while let Some(v) = queue.pop_front() {
                for &u in &adj_from_basis[v] {
                    min_weights[u] = min_weights[u].min(min_weights[v]);
                    indegrees[u] -= 1;
                    if indegrees[u] == 0 {
                        queue.push_back(u);
                    }
                }
            }

            assert!(indegrees.iter().all(|&d| d == 0), "BFS error (from basis)");

            for i in 0..arm_num {
                if !in_opt[i] {
                    gaps[i] = min_weights[i] - weights[i];
                }
            }
        }

        // Return the index with the maximum gap.
        let mut maxgap = f64::NEG_INFINITY;
        let mut maxgap_arm = 0;

        for i in self.get_arms() {
            if gaps[i] > maxgap {
                maxgap = gaps[i];
                maxgap_arm = i;
            }
        }

        maxgap_arm
    }
}

pub trait RandomSample {
    /// Randomly sample an instance with `arm_num` arms.
    fn sample(arm_num: usize) -> Self;
}
