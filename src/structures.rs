pub mod uniform_matroid;

use crate::arms::Weights;

pub trait Structure: Clone {
    /**
     * Get indices of the remaining arms.
     */
    fn get_indices(&self) -> &Vec<usize>;

    /**
     * Contract the arm i.
     * Assume that the arm i remains.
     */
    fn contract_arm(&mut self, i: usize) -> &mut Self;

    /**
     * Delete the arm i
     * Assume that the arm i remains.
     */
    fn delete_arm(&mut self, i: usize) -> &mut Self;

    /**
     * Find the superarm with the maximum sum of weights.
     */
    fn optimal(&self, weights: &Weights) -> Vec<usize>;

    /**
     * Efficiently find the arm with the maximum gap .
     */
    fn fast_maxgap(&self, weights: &Weights) -> usize;
}
