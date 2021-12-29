use super::Structure;
use crate::{arms::Weights, util::union_find::UnionFind};
use core::cmp::max;

#[derive(Clone)]
pub struct CircuitMatroid {
    indices: Vec<usize>,
    edges: Vec<(usize, usize)>,
    vnum: usize,
}

impl CircuitMatroid {
    pub fn new(_edges: &Vec<(usize, usize)>) -> Self {
        CircuitMatroid {
            indices: (0.._edges.len()).collect(),
            edges: _edges.clone(),
            vnum: _edges
                .iter()
                .fold(0_usize, |acc, &(u, v)| max(max(acc, u), v))
                + 1,
        }
    }
}

impl Structure for CircuitMatroid {
    fn get_indices(&self) -> &Vec<usize> {
        &self.indices
    }

    fn contract_arm(&mut self, i: usize) -> &mut Self {
        let pos = self.get_indices().iter().position(|&r| r == i).unwrap();

        // Contract s and t
        let (s, t) = self.edges[pos];
        for (u, v) in self.edges.iter_mut() {
            if *u == t {
                *u = s;
            }
            if *v == t {
                *v = s;
            }
        }

        // Delete
        self.indices.swap_remove(pos);
        self.edges.swap_remove(pos);
        self
    }

    fn delete_arm(&mut self, i: usize) -> &mut Self {
        let pos = self.get_indices().iter().position(|&r| r == i).unwrap();
        self.indices.swap_remove(pos);
        self.edges.swap_remove(pos);
        self
    }

    /// Find the maximum spanning tree by the Kruskal's algorithm
    fn optimal(&self, weights: &Weights) -> Vec<usize> {
        // zip indices of arms, edges, and weights
        let mut indexed_weights: Vec<(usize, (usize, usize), f64)> = (0..self.indices.len())
            .map(|i| {
                let arm_id = self.indices[i];
                (arm_id, self.edges[i], weights[arm_id])
            })
            .collect();

        // Sort by weights in decreasing order
        indexed_weights
            .sort_unstable_by(|(_, _, fl), (_, _, fr)| fl.partial_cmp(fr).unwrap().reverse());

        // Add the heaviest edge greedily if it doesn't induce any cycle.
        let mut arms = Vec::<usize>::new();
        let mut uf = UnionFind::new(self.vnum);
        for (i, (u, v), _w) in indexed_weights {
            if !uf.same(u, v) {
                uf.unite(u, v);
                arms.push(i);
            }
        }

        arms
    }

    fn fast_maxgap(&self, weights: &Weights) -> usize {
        todo!();
    }
}
