use crate::arms::{Arms, Weights};
use crate::sampler::Sampler;
use crate::structure::CombinatorialStructure;

/// Find the optimal superarm by the CSAR algorithm.
pub fn csar(mut structure: impl CombinatorialStructure, arms: &mut Arms) -> Vec<usize> {
    let mut accepted_arms = Vec::<usize>::new();

    // the total number of arms
    let n = structure.get_indices().len();

    let mut samplers: Vec<Sampler> = (0..n).map(|_| Sampler::new()).collect();

    for _ in 0..n {
        // sample the remaining arms 100 times
        for &i in structure.get_indices() {
            for _ in 0..100 {
                samplers[i].observe(arms.sample(i));
            }
        }

        let weights: Vec<f64> = (0..n).map(|i| samplers[i].get_mean()).collect();

        // Find the optimal superarm and the arm with the maximum gap.
        let best_arms = structure.optimal(&weights).unwrap();
        let maxgap_arm = structure.fast_maxgap(&weights);

        // Contract or delete the arm.
        if best_arms.contains(&maxgap_arm) {
            accepted_arms.push(maxgap_arm);
            structure.contract_arm(maxgap_arm);
        } else {
            structure.delete_arm(maxgap_arm);
        }
    }

    accepted_arms
}

/// Find the arm with the maximum gap.
/// It is required that the number of arms is greater than 0 and equal to the length of `weights`.
pub fn naive_maxgap(structure: &impl CombinatorialStructure, weights: &Weights) -> usize {
    // Check the requirement
    assert_ne!(structure.get_indices().len(), 0);
    assert_eq!(structure.get_indices().len(), weights.len());

    // Find the optimal superarm
    let opt_arms = structure.optimal(weights).unwrap();
    let opt_weight: f64 = opt_arms.iter().map(|&i| weights[i]).sum();

    // Whether or not the arm is in the optimal superarm.
    let num_arms = structure.get_indices().iter().max().unwrap_or(&0) + 1;
    let mut in_opt = vec![false; num_arms];
    for &i in &opt_arms {
        in_opt[i] = true;
    }

    let mut maxgap = 0_f64;
    let mut best_arm = None;

    for &i in structure.get_indices() {
        let mut new_structure = (*structure).clone();
        let mut subopt_weight = 0_f64;

        if in_opt[i] {
            // The superarm excludes the arm i.
            new_structure.delete_arm(i);
        } else {
            // The superarm contains the arm i.
            subopt_weight += weights[i];
            new_structure.contract_arm(i);
        }

        // Find the sub-optimal superarm w.r.t. the arm i.
        subopt_weight += if let Some(subopt_arms) = new_structure.optimal(weights) {
            subopt_arms.iter().map(|&i| weights[i]).sum::<f64>()
        } else {
            // If there is no superarm, the maximum weight is -INF.
            f64::NEG_INFINITY
        };
        let gap = opt_weight - subopt_weight;

        if gap > maxgap {
            maxgap = gap;
            best_arm = Some(i);
        }
    }

    if let Some(id) = best_arm {
        id
    } else {
        // If any arms should not be chosen, return the first arm.
        *structure.get_indices().first().unwrap()
    }
}
