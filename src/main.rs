mod algorithm;
mod arms;
mod sampler;
mod structure;
mod util;

fn main() {}

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
