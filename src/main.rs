mod arms;
mod csar;
mod structures;

use arms::Weights;
use rand::Rng;
use structures::uniform_matroid;

fn main() {
    let n = 10_usize;
    let rank = 5_usize;

    let structure = uniform_matroid::Structure::new(n, rank);

    let mut rng = rand::thread_rng();
    let weights: Weights = (0..n).map(|_| rng.gen()).collect();

    println!("{:?}", weights);

    let naive_arm = csar::naive_maxgap(&structure, &weights);
    let faster_arm = uniform_matroid::fast_maxgap(&structure, &weights);

    println!("naive: {}, faster: {}", naive_arm, faster_arm);
}
