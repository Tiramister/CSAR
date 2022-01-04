use super::CombinatorialStructure;
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

    fn get_indices(&self) -> Vec<usize> {
        self.indices.clone()
    }

    fn contract_arm(&mut self, i: usize) -> &mut Self {
        let pos = self.get_indices().iter().position(|&r| r == i).unwrap();
        self.indices.swap_remove(pos);
        self.rank -= 1;
        self
    }

    fn delete_arm(&mut self, i: usize) -> &mut Self {
        let pos = self.get_indices().iter().position(|&r| r == i).unwrap();
        self.indices.swap_remove(pos);
        self
    }

    fn optimal(&self, weights: &[f64]) -> Option<Vec<usize>> {
        if self.indices.len() < self.rank {
            return None;
        }

        // zip indices of arms and their weights
        let mut indexed_weights: Vec<(usize, f64)> = self
            .get_indices()
            .iter()
            .map(|&i| (i, weights[i]))
            .collect();

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
        for v in self.get_indices() {
            if in_basis[v] {
                result_graph.add_edge(v, n);
            } else {
                result_graph.add_edge(n, v);
            }
        }

        result_graph
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::{
        algorithm::{csar, naive_maxgap},
        arms::{Arms, Weights},
        structure::{uniform_matroid::UniformMatroid, CombinatorialStructure},
    };

    fn test_maxgap_once(n: usize, rank: usize) {
        let structure = UniformMatroid::new(n, rank);

        let mut rng = rand::thread_rng();
        let weights: Weights = (0..n).map(|_| rng.gen()).collect();

        eprintln!("{:?}", weights);

        let naive_arm = naive_maxgap(&structure, &weights);
        let faster_arm = structure.fast_maxgap(&weights);

        eprintln!("naive: {}, faster: {}", naive_arm, faster_arm);
        assert!(naive_arm == faster_arm);
    }

    #[test]
    fn test_maxgap() {
        for _ in 0..10 {
            test_maxgap_once(100, 50);
        }
    }

    fn test_csar_once(n: usize, rank: usize) {
        let mut arms = Arms::new();

        // generate arms randomly
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            arms.add_arm(rng.gen(), rng.gen());
        }

        let structure = UniformMatroid::new(n, rank);

        let mut csar_optimal = csar(structure.clone(), &mut arms);
        csar_optimal.sort_unstable();

        let means: Weights = structure
            .get_indices()
            .iter()
            .map(|&i| arms.get_mean(i))
            .collect();

        let mut true_optimal = structure.optimal(&means).unwrap();
        true_optimal.sort_unstable();

        eprintln!("csar: {:?}", csar_optimal);
        eprintln!("true: {:?}", true_optimal);
        eprintln!("----------");

        // assert!(csar_optimal == true_optimal);
    }

    #[test]
    fn test_csar() {
        for _ in 0..10 {
            test_csar_once(10, 5);
        }
    }
}
