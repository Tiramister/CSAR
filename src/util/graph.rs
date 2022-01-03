use std::{cmp::max, mem::swap};

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

    pub fn from_edges(edges: &Vec<(usize, usize)>) -> Self {
        Self {
            edges: edges.clone(),
            vnum: edges.iter().map(|&(u, v)| max(u, v)).max().unwrap_or(0) + 1,
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
}
