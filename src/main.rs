mod arms;
mod csar;
mod structures;

fn main() {}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::{
        arms::Weights,
        csar::naive_maxgap,
        structures::{uniform_matroid::UniformMatroid, Structure},
    };

    fn test_uniform_matroid_once(n: usize, rank: usize) {
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
    fn test_uniform_matroid() {
        for _ in 0..10 {
            test_uniform_matroid_once(100, 50);
        }
    }
}
