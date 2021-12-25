use crate::arms::Weights;
use crate::structures::uniform_matroid;

pub fn naive_maxgap(structure: &uniform_matroid::Structure, weights: &Weights) -> usize {
    let opt_arms = uniform_matroid::optimal(structure, weights);
    let opt_weight: f64 = opt_arms.iter().map(|&i| weights[i]).sum();

    let num_arms = structure.index.iter().max().unwrap() + 1;

    // Whether or not the arm is in the optimal superarm.
    let mut in_opt = vec![false; num_arms];
    for &i in &opt_arms {
        in_opt[i] = true;
    }

    let mut maxgap = 0_f64;
    let mut best_arm = 0_usize;

    for &i in &structure.index {
        let mut new_structure = structure.clone();
        let mut subopt_weight = 0_f64;

        if in_opt[i] {
            new_structure.delete_arm(i);
        } else {
            subopt_weight += weights[i];
            new_structure.contract_arm(i);
        }

        let subopt_arms = uniform_matroid::optimal(&new_structure, weights);
        subopt_weight += subopt_arms.iter().map(|&i| weights[i]).sum::<f64>();
        let gap = opt_weight - subopt_weight;

        if gap > maxgap {
            maxgap = gap;
            best_arm = i;
        }
    }

    return best_arm;
}
