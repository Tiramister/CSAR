use super::Structure;
use crate::{arms::Weights, csar::naive_maxgap, util::graph::Graph};

#[derive(Clone)]
pub struct CircuitMatroid {
    indices: Vec<usize>,
    graph: Graph,
}

impl CircuitMatroid {
    pub fn new(edges: &Vec<(usize, usize)>) -> Self {
        CircuitMatroid {
            indices: (0..edges.len()).collect(),
            graph: Graph::from_edges(edges),
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
        let mapped_weights: Weights = (0..self.indices.len()).map(|i| weights[i]).collect();

        if let Some(mst) = self.graph.maximum_spanning_tree(&mapped_weights) {
            // Convert edge-indices to arm-indices.
            Some(mst.iter().map(|&i| self.indices[i]).collect())
        } else {
            None
        }
    }

    /// To be implemented.
    fn fast_maxgap(&self, weights: &Weights) -> usize {
        naive_maxgap(self, weights)
    }
}

#[cfg(test)]
mod tests {
    use itertools::iproduct;
    use rand::Rng;

    use crate::{
        arms::{Arms, Weights},
        csar::{csar, naive_maxgap},
        structures::{circuit_matroid::CircuitMatroid, Structure},
    };

    fn test_csar_once(n: usize) {
        let edges: Vec<(usize, usize)> =
            iproduct!((0..n), (0..n)).filter(|&(x, y)| x < y).collect();

        // generate arms randomly
        let mut arms = Arms::new();
        let mut rng = rand::thread_rng();
        for _ in 0..edges.len() {
            arms.add_arm(rng.gen(), rng.gen());
        }

        let structure = CircuitMatroid::new(&edges);

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
