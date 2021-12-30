use crate::arms::{Arms, Weights};
use crate::sampler::Sampler;
use crate::structures::Structure;

pub fn csar(mut structure: impl Structure, arms: &mut Arms) -> Vec<usize> {
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
            structure.contract_by_arm(maxgap_arm);
        } else {
            structure.delete_arm(maxgap_arm);
        }
    }

    accepted_arms
}

pub fn naive_maxgap(structure: &impl Structure, weights: &Weights) -> usize {
    let opt_arms = structure.optimal(weights).unwrap();
    let opt_weight: f64 = opt_arms.iter().map(|&i| weights[i]).sum();

    let num_arms = structure.get_indices().iter().max().unwrap_or(&0) + 1;

    // Whether or not the arm is in the optimal superarm.
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
            new_structure.delete_arm(i);
        } else {
            subopt_weight += weights[i];
            new_structure.contract_by_arm(i);
        }

        subopt_weight += if let Some(subopt_arms) = new_structure.optimal(weights) {
            subopt_arms.iter().map(|&i| weights[i]).sum::<f64>()
        } else {
            // The maximum weight of an infeasible structure is -INF.
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
