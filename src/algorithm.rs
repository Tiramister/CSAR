use crate::arms::Arms;
use crate::sampler::Sampler;
use crate::structure::CombinatorialStructure;

/// Find the optimal superarm by the CSAR algorithm.
pub fn csar(mut structure: impl CombinatorialStructure, arms: &mut Arms) -> Vec<usize> {
    let mut accepted_arms = Vec::<usize>::new();

    // the total number of arms
    let n = structure.get_arms().len();

    let mut samplers: Vec<Sampler> = (0..n).map(|_| Sampler::new()).collect();

    for _ in 0..n {
        // sample the remaining arms 100 times
        for i in structure.get_arms() {
            for _ in 0..100 {
                samplers[i].observe(arms.sample(i));
            }
        }

        let weights: Vec<f64> = samplers.iter().map(|sampler| sampler.get_mean()).collect();

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
pub fn naive_maxgap(structure: &impl CombinatorialStructure, weights: &[f64]) -> usize {
    let arms = structure.get_arms();
    // Check the requirement
    assert_ne!(arms.len(), 0);
    assert_eq!(arms.len(), weights.len());

    // Find the optimal superarm
    let opt_arms = structure.optimal(weights).unwrap();
    let opt_weight: f64 = opt_arms.iter().map(|&i| weights[i]).sum();

    // Whether or not the arm is in the optimal superarm.
    let mut in_opt = vec![false; structure.get_arm_num()];
    for &i in &opt_arms {
        in_opt[i] = true;
    }

    let mut maxgap = f64::NEG_INFINITY;
    let mut maxgap_arm = None;

    let mut gaps = Vec::<f64>::new();

    for &i in &arms {
        let mut new_structure = structure.clone();
        let mut subopt_weight = 0_f64;

        if in_opt[i] {
            // Exclude the arm i.
            new_structure.delete_arm(i);
        } else {
            // Include the arm i.
            subopt_weight += weights[i];
            new_structure.contract_arm(i);
        }

        // Find the optimal superarm satisfying the condition of the arm i.
        subopt_weight += if let Some(subopt_arms) = new_structure.optimal(weights) {
            subopt_arms.iter().map(|&i| weights[i]).sum::<f64>()
        } else {
            // If there is no superarm, the maximum weight is -INF.
            f64::NEG_INFINITY
        };
        let gap = opt_weight - subopt_weight;
        gaps.push(gap);

        if gap > maxgap {
            maxgap = gap;
            maxgap_arm = Some(i);
        }
    }

    if let Some(id) = maxgap_arm {
        id
    } else {
        // If any arms should not be chosen, return the first arm.
        *arms.first().unwrap()
    }
}

#[allow(dead_code)]
pub mod tests {
    use crate::{
        algorithm::{csar, naive_maxgap},
        arms::{Arms, Weights},
        structure::{CombinatorialStructure, RandomSample},
    };
    use rand::Rng;

    pub fn test_maxgap<Structure>(arm_num: usize)
    where
        Structure: CombinatorialStructure + RandomSample,
    {
        let mut rng = rand::thread_rng();
        let structure = Structure::sample(arm_num);

        // Generate weights randomly.
        let weights: Weights = (0..arm_num).map(|_| rng.gen()).collect();

        // Find the edge with the maximum gap.
        let naive_arm = naive_maxgap(&structure, &weights);
        let faster_arm = structure.fast_maxgap(&weights);

        assert!(naive_arm == faster_arm);
    }

    pub fn test_csar<Structure>(arm_num: usize)
    where
        Structure: CombinatorialStructure + RandomSample,
    {
        let structure = Structure::sample(arm_num);

        let mut arms = Arms::new();
        let mut rng = rand::thread_rng();
        for _ in 0..arm_num {
            arms.add_arm(rng.gen(), rng.gen());
        }

        let mut csar_optimal = csar(structure.clone(), &mut arms);
        csar_optimal.sort_unstable();

        let means: Weights = structure
            .get_arms()
            .iter()
            .map(|&i| arms.get_mean(i))
            .collect();

        let mut true_optimal = structure.optimal(&means).unwrap();
        true_optimal.sort_unstable();

        println!("csar: {:?}", csar_optimal);
        println!("true: {:?}", true_optimal);
    }
}
