use crate::arms::Weights;

#[derive(Clone)]
pub struct Structure {
    pub index: Vec<usize>,
    pub rank: usize,
}

impl Structure {
    pub fn new(n: usize, _rank: usize) -> Self {
        Structure {
            index: (0..n).collect(),
            rank: _rank,
        }
    }

    /**
     * Assume that the arm i remains.
     */
    pub fn contract_arm(&mut self, i: usize) {
        let pos = self.index.iter().position(|&r| r == i).unwrap();
        self.index.swap_remove(pos);
        self.rank -= 1;
    }

    /**
     * Assume that the arm i remains.
     */
    pub fn delete_arm(&mut self, i: usize) {
        let pos = self.index.iter().position(|&r| r == i).unwrap();
        self.index.swap_remove(pos);
    }
}

/**
 * Find the superarm with the maximum sum of weights.
 */
pub fn optimal(structure: &Structure, weights: &Weights) -> Vec<usize> {
    // zip indices of arms and their weights
    let mut indexed_weights: Vec<(usize, f64)> =
        structure.index.iter().map(|&i| (i, weights[i])).collect();

    // sort by weights in decreasing order
    indexed_weights.sort_unstable_by(|(_, fl), (_, fr)| fl.partial_cmp(fr).unwrap().reverse());

    // leave first rank elements
    indexed_weights.truncate(structure.rank);

    // map to their original indices
    indexed_weights.iter().map(|(i, _)| *i).collect()
}

/**
 * Efficiently find the arm with the maximum gap .
 */
pub fn fast_maxgap(structure: &Structure, weights: &Weights) -> usize {
    // zip indices of arms and their weights
    let mut indexed_weights: Vec<(usize, f64)> =
        structure.index.iter().map(|&i| (i, weights[i])).collect();

    // sort by weights in decreasing order
    indexed_weights.sort_unstable_by(|(_, fl), (_, fr)| fl.partial_cmp(fr).unwrap().reverse());

    // the maximum gap of arms in the optimal superarm.
    let in_gap = indexed_weights.first().unwrap().1 - indexed_weights[structure.rank].1;
    // the maximum gap of arms out of the optimal superarm.
    let out_gap = indexed_weights[structure.rank - 1].1 - indexed_weights.last().unwrap().1;

    if in_gap > out_gap {
        indexed_weights.first().unwrap().0
    } else {
        indexed_weights.last().unwrap().0
    }
}
