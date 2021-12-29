use crate::arms::{Arms, Weights};
use crate::sampler::Sampler;
use crate::structures::Structure;

pub fn csar(mut structure: impl Structure, arms: &mut Arms) -> Vec<usize> {
    let mut accepted_arms = Vec::<usize>::new();

    // the total number of arms
    let n = structure.get_indices().len();

    let mut samplers: Vec<Sampler> = (0..n).map(|_| Sampler::new()).collect();

    for _ in 0..n {
        // sample the remaining arms 1000 times
        for &i in structure.get_indices() {
            for _ in 0..1000 {
                samplers[i].observe(arms.sample(i));
            }
        }

        let weights: Vec<f64> = (0..n).map(|i| samplers[i].get_mean()).collect();

        let best_arms = structure.optimal(&weights);
        let maxgap_arm = structure.fast_maxgap(&weights);

        if best_arms.contains(&maxgap_arm) {
            accepted_arms.push(maxgap_arm);
            structure.contract_arm(maxgap_arm);
        } else {
            structure.delete_arm(maxgap_arm);
        }
    }

    {
        let weights: Vec<f64> = (0..n).map(|i| samplers[i].get_mean()).collect();
        println!("empirical means: {:?}", weights);
    }

    accepted_arms
}

pub fn naive_maxgap(structure: &impl Structure, weights: &Weights) -> usize {
    let opt_arms = structure.optimal(weights);
    let opt_weight: f64 = opt_arms.iter().map(|&i| weights[i]).sum();

    let num_arms = structure.get_indices().iter().max().unwrap() + 1;

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
            new_structure.contract_arm(i);
        }

        let subopt_arms = new_structure.optimal(weights);
        subopt_weight += subopt_arms.iter().map(|&i| weights[i]).sum::<f64>();
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
