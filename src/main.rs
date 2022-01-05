use crate::{
    algorithm::csar,
    arms::{Arms, Weights},
    structure::{uniform_matroid::UniformMatroid, CombinatorialStructure, RandomSample},
};
use rand::{thread_rng, Rng};
use structure::circuit_matroid::CircuitMatroid;

mod algorithm;
mod arms;
mod sampler;
mod structure;
mod util;

enum EnumCombinatorialStructures {
    UniformMatroid(UniformMatroid),
    CircuitMatroid(CircuitMatroid),
}

fn read_int(maximum: usize, request_msg: &str) -> usize {
    loop {
        eprint!("[QUERY] {}", request_msg);

        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("[ERROR] Input error. Aborting.");

        if let Ok(num) = buf.trim().parse() {
            if 0 < num && num <= maximum {
                return num;
            }
        }
        eprintln!("[ERROR] Invalid input. Try again.");
    }
}

const EPS: f64 = 1e-15;

fn main() {
    let structure_type = read_int(
        2,
        r"Which combinatorial structure to test?
    1. Uniform Matroid
    2. Circuit Matroid
> ",
    );
    eprintln!(
        "[INFO] {} is chosen.",
        ["Uniform Matroid", "Circuit Matroid"][structure_type - 1]
    );

    let arm_num = read_int(100_000, "The number of arms (up to 100,000) > ");
    eprintln!("[INFO] {} arm(s).", arm_num);

    let trials = read_int(100_000, "The number of trials > ");
    eprintln!("[INFO] {} trials(s).", trials);

    let structure = if structure_type == 0 {
        EnumCombinatorialStructures::UniformMatroid(UniformMatroid::sample(arm_num))
    } else {
        EnumCombinatorialStructures::CircuitMatroid(CircuitMatroid::sample(arm_num))
    };
    eprintln!("[INFO] An instance has been randomly sampled.");

    let mut rng = thread_rng();
    for _ in 0..trials {
        let mut arms = Arms::new();
        for _ in 0..arm_num {
            arms.add_arm(rng.gen(), rng.gen());
        }

        let means: Weights = (0..arm_num).map(|i| arms.get_mean(i)).collect();

        let csar_optimal = match &structure {
            EnumCombinatorialStructures::UniformMatroid(s) => csar(s.clone(), &mut arms),
            EnumCombinatorialStructures::CircuitMatroid(s) => csar(s.clone(), &mut arms),
        };
        let csar_weight: f64 = csar_optimal.iter().map(|&i| means[i]).sum();

        let true_optimal = match &structure {
            EnumCombinatorialStructures::UniformMatroid(s) => s.optimal(&means).unwrap(),
            EnumCombinatorialStructures::CircuitMatroid(s) => s.optimal(&means).unwrap(),
        };
        let true_weight: f64 = true_optimal.iter().map(|&i| means[i]).sum();

        let relative_error = (true_weight - csar_weight) / true_weight;
        if relative_error < EPS {
            eprintln!(
                "[RESULT] Correct. The relative error = {:.20}",
                relative_error
            );
        } else {
            eprintln!(
                "[RESULT] Wrong.   The relative error = {:.20}",
                relative_error
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        algorithm::tests::{test_csar, test_maxgap},
        structure::{circuit_matroid::CircuitMatroid, uniform_matroid::UniformMatroid},
    };

    #[test]
    fn test_uniform_maxgap() {
        test_maxgap::<UniformMatroid>(100);
    }

    #[test]
    fn test_uniform_csar() {
        test_csar::<UniformMatroid>(100);
    }

    #[test]
    fn test_circuit_maxgap() {
        test_maxgap::<CircuitMatroid>(100);
    }

    #[test]
    fn test_circuit_csar() {
        test_csar::<CircuitMatroid>(100);
    }
}
