mod arms;
mod csar;
mod sampler;
mod structures;

fn main() {}

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
