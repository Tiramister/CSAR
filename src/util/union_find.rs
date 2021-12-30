use std::mem::swap;

pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    /// Build n singletons.
    pub fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            size: vec![1; n],
        }
    }

    /// Merge the set containing u and the one containing v.
    pub fn unite(&mut self, mut u: usize, mut v: usize) -> &mut Self {
        u = self.par[u];
        v = self.par[v];

        if u != v {
            // Balancing
            if self.size[u] < self.size[v] {
                swap(&mut u, &mut v);
            }

            self.size[u] += self.size[v];
            self.par[v] = u;
        }

        self
    }

    /// Find the root of the tree containing v.
    pub fn find_root(&mut self, v: usize) -> usize {
        if self.par[v] != v {
            // Path compression
            self.par[v] = self.find_root(self.par[v]);
        }
        self.par[v]
    }

    /// Judge whether or not u and v belong to the same tree.
    pub fn same(&mut self, u: usize, v: usize) -> bool {
        self.find_root(u) == self.find_root(v)
    }
}
