use super::Structure;
use crate::arms::Weights;

#[derive(Clone)]
pub struct UniformMatroid {
    indices: Vec<usize>,
    rank: usize,
}

impl UniformMatroid {
    pub fn new(n: usize, _rank: usize) -> Self {
        UniformMatroid {
            indices: (0..n).collect(),
            rank: _rank,
        }
    }
}

impl Structure for UniformMatroid {
    fn get_indices(&self) -> &Vec<usize> {
        &self.indices
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

    fn optimal(&self, weights: &Weights) -> Vec<usize> {
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
        indexed_weights.iter().map(|(i, _)| *i).collect()
    }

    fn fast_maxgap(&self, weights: &Weights) -> usize {
        // zip indices of arms and their weights
        let mut indexed_weights: Vec<(usize, f64)> = self
            .get_indices()
            .iter()
            .map(|&i| (i, weights[i]))
            .collect();

        // sort by weights in decreasing order
        indexed_weights.sort_unstable_by(|(_, fl), (_, fr)| fl.partial_cmp(fr).unwrap().reverse());

        // the maximum gap of arms in the optimal superarm.
        let in_gap = if self.rank == indexed_weights.len() {
            f64::MAX
        } else {
            indexed_weights.first().unwrap().1 - indexed_weights[self.rank].1
        };

        // the maximum gap of arms out of the optimal superarm.
        let out_gap = if self.rank == 0 {
            f64::MAX
        } else {
            indexed_weights[self.rank - 1].1 - indexed_weights.last().unwrap().1
        };

        if in_gap > out_gap {
            indexed_weights.first().unwrap().0
        } else {
            indexed_weights.last().unwrap().0
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::{
        arms::{Arms, Weights},
        csar::{csar, naive_maxgap},
        structures::{uniform_matroid::UniformMatroid, Structure},
    };

    fn test_maxgap_uniform_once(n: usize, rank: usize) {
        let structure = UniformMatroid::new(n, rank);

        let mut rng = rand::thread_rng();
        let weights: Weights = (0..n).map(|_| rng.gen()).collect();

        println!("{:?}", weights);

        let naive_arm = naive_maxgap(&structure, &weights);
        let faster_arm = structure.fast_maxgap(&weights);

        println!("naive: {}, faster: {}", naive_arm, faster_arm);
        assert!(naive_arm == faster_arm);
    }

    #[test]
    fn test_maxgap_uniform() {
        for _ in 0..10 {
            test_maxgap_uniform_once(100, 50);
        }
    }

    fn test_csar_uniform_once(n: usize, rank: usize) {
        let mut arms = Arms::new();

        // generate arms randomly
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            arms.add_arm(rng.gen(), rng.gen());
        }

        let structure = UniformMatroid::new(n, rank);

        let mut csar_optimal = csar(structure.clone(), &mut arms);
        csar_optimal.sort();

        let means: Weights = structure
            .get_indices()
            .iter()
            .map(|&i| arms.get_mean(i))
            .collect();

        println!("true means:     {:?}", means);

        let mut true_optimal = structure.optimal(&means);
        true_optimal.sort();

        println!("csar: {:?}", csar_optimal);
        println!("true: {:?}", true_optimal);
        println!("----------");

        // assert!(csar_optimal == true_optimal);
    }

    #[test]
    fn test_csar_uniform() {
        for _ in 0..10 {
            test_csar_uniform_once(10, 5);
        }
    }
}
