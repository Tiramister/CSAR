use rand::Rng;

use super::{CombinatorialStructure, RandomSample};
use crate::util::graph::Graph;

#[derive(Clone)]
pub struct UniformMatroid {
    arm_num: usize,
    indices: Vec<usize>,
    rank: usize,
}

impl UniformMatroid {
    pub fn new(arm_num: usize, rank: usize) -> Self {
        UniformMatroid {
            arm_num,
            indices: (0..arm_num).collect(),
            rank,
        }
    }
}

impl CombinatorialStructure for UniformMatroid {
    fn get_arm_num(&self) -> usize {
        self.arm_num
    }

    fn get_arms(&self) -> Vec<usize> {
        self.indices.clone()
    }

    fn contract_arm(&mut self, i: usize) -> &mut Self {
        let pos = self.get_arms().iter().position(|&r| r == i).unwrap();
        self.indices.swap_remove(pos);
        self.rank -= 1;
        self
    }

    fn delete_arm(&mut self, i: usize) -> &mut Self {
        let pos = self.get_arms().iter().position(|&r| r == i).unwrap();
        self.indices.swap_remove(pos);
        self
    }

    fn optimal(&self, weights: &[f64]) -> Option<Vec<usize>> {
        if self.indices.len() < self.rank {
            return None;
        }

        // zip indices of arms and their weights
        let mut indexed_weights: Vec<(usize, f64)> =
            self.get_arms().iter().map(|&i| (i, weights[i])).collect();

        // sort by weights in decreasing order
        indexed_weights.sort_unstable_by(|(_, fl), (_, fr)| fl.partial_cmp(fr).unwrap().reverse());

        // leave first rank elements
        indexed_weights.truncate(self.rank);

        // map to their original indices
        Some(indexed_weights.iter().map(|&(i, _)| i).collect())
    }

    fn reachability_graph(&self, basis: &[usize]) -> Graph {
        let n = self.get_arm_num();
        let mut in_basis = vec![false; n];
        for &i in basis {
            in_basis[i] = true;
        }

        let mut result_graph = Graph::new(n + 1);
        for v in self.get_arms() {
            if in_basis[v] {
                result_graph.add_edge(v, n);
            } else {
                result_graph.add_edge(n, v);
            }
        }

        result_graph
    }
}

impl RandomSample for UniformMatroid {
    fn sample(arm_num: usize) -> Self {
        let mut rng = rand::thread_rng();
        let rank = rng.gen_range(0..(arm_num + 1));
        UniformMatroid::new(arm_num, rank)
    }
}
