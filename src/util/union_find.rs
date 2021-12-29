pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    /// Build a forest of n isolated vertices.
    pub fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            size: vec![1; n],
        }
    }

    /// Set u as the parent of v.
    /// It is required that u and v are roots of different subtrees.
    pub fn unite(&mut self, u: usize, v: usize) -> &mut Self {
        // Check the requirement.
        assert_eq!(self.par[u], u);
        assert_eq!(self.par[v], v);
        assert_ne!(u, v);

        self.size[u] += self.size[v];
        self.par[v] = u;

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

    /// Judge whether or not u and v belong to a same tree.
    pub fn same(&mut self, u: usize, v: usize) -> bool {
        self.find_root(u) == self.find_root(v)
    }
}
