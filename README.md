# Faster CSAR algorithm on matroids

## Usage

You need [Rust](https://www.rust-lang.org/) runtime environment.

In the root directory of this project, run:

```
cargo run --release
```

Then you are asked about the following settings:

* The underlying combinatorial structure. (uniform matroids or circuit matroids)
* The number of arms. (up to 100,000)
* The number of repetition. (up to 100,000)

After answering these questions, the experiment begins to run.
