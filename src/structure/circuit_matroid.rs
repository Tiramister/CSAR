use super::{CombinatorialStructure, RandomSample};
use crate::{arms::Weights, util::graph::Graph};
use rand::Rng;
use std::{collections::VecDeque, mem::swap};

#[derive(Clone)]
pub struct CircuitMatroid {
    arm_num: usize,
    arms: Vec<usize>,
    graph: Graph,
}

impl CircuitMatroid {
    pub fn new(graph: &Graph) -> Self {
        let m = graph.get_edges().len();
        CircuitMatroid {
            arm_num: m,
            arms: (0..m).collect(),
            graph: graph.clone(),
        }
    }
}

impl CombinatorialStructure for CircuitMatroid {
    fn get_arm_num(&self) -> usize {
        self.arm_num
    }

    fn get_arms(&self) -> Vec<usize> {
        self.arms.clone()
    }

    fn contract_arm(&mut self, i: usize) -> &mut Self {
        let pos = self.get_arms().iter().position(|&r| r == i).unwrap();

        self.graph.contract_edge(pos);
        // Keep the order of edges
        self.arms.swap_remove(pos);

        self
    }

    fn delete_arm(&mut self, i: usize) -> &mut Self {
        let pos = self.get_arms().iter().position(|&r| r == i).unwrap();

        self.graph.delete_edge(pos);
        // Keep the order of edges
        self.arms.swap_remove(pos);

        self
    }

    fn optimal(&self, weights: &[f64]) -> Option<Vec<usize>> {
        // Reorder weights to be edge-indexed.
        let mapped_weights: Weights = self.get_arms().iter().map(|&i| weights[i]).collect();

        self.graph
            .maximum_spanning_tree(&mapped_weights)
            .map(|mst| mst.iter().map(|&i| self.arms[i]).collect())
    }

    fn reachability_graph(&self, opt_arms: &[usize]) -> Graph {
        let arm_num = self.get_arm_num();
        let arms = self.get_arms();

        // arm-index to edge-index
        let mut arm_to_edge = vec![0_usize; arm_num];
        for (edge_i, &arm_i) in arms.iter().enumerate() {
            arm_to_edge[arm_i] = edge_i;
        }

        let graph = &self.graph;
        let edges = graph.get_edges();
        let vnum = graph.get_vnum();

        // Build the adjacency list.
        let mut adj = vec![Vec::<usize>::new(); vnum];
        for &arm_i in opt_arms {
            let edge_i = arm_to_edge[arm_i];
            let (u, v) = edges[edge_i];
            adj[u].push(edge_i);
            adj[v].push(edge_i);
        }

        let mut result_graph = Graph::new(arm_num);

        // Make rooted from the vertex 0.
        // The depth of each vertex.
        let mut depth = vec![0_usize; vnum];
        // Store the parent and the edge to the parent.
        let mut par_vertex = vec![Vec::<usize>::new(); vnum];
        let mut par_edge = vec![Vec::<usize>::new(); vnum];

        // Run BFS.
        let mut queue = VecDeque::<usize>::new();
        let mut visited = vec![false; vnum];

        // Start from the vertex 0.
        queue.push_back(0);
        depth[0] = 0;

        while let Some(u) = queue.pop_front() {
            visited[u] = true;

            // Doubling.
            let mut k = 0;
            while 1 << (k + 1) <= depth[u] {
                let v = par_vertex[u][k];
                let uv = par_edge[u][k];

                let w = par_vertex[v][k];
                let vw = par_edge[v][k];
                // Compress the path u -> v -> w.

                // Create new vertex corresponding to the path uw.
                let uw = result_graph.get_vnum();
                result_graph.add_edge(uv, uw).add_edge(vw, uw);

                par_vertex[u].push(w);
                par_edge[u].push(uw);
                k += 1;
            }

            for &i in &adj[u] {
                let (p, q) = graph.get_edges()[i];
                // The other vertex.
                let v = u ^ p ^ q;

                if !visited[v] {
                    // Set p as the parent of q.
                    depth[v] = depth[u] + 1;
                    par_vertex[v].push(u);
                    par_edge[v].push(arms[i]);
                    queue.push_back(v);
                }
            }
        }

        let mut in_tree = vec![false; edges.len()];
        for &i in opt_arms {
            in_tree[arm_to_edge[i]] = true;
        }

        // Span edges to each vertex whose corresponding edge is not in the given tree.
        for (i, &(mut u, mut v)) in edges.iter().enumerate() {
            if in_tree[i] {
                continue;
            }

            // Equalize the depths of u and v
            while depth[u] != depth[v] {
                if depth[u] < depth[v] {
                    swap(&mut u, &mut v);
                }

                let k = (depth[u] - depth[v]).trailing_zeros() as usize;
                result_graph.add_edge(par_edge[u][k], arms[i]);
                u = par_vertex[u][k];
            }
            assert_eq!(depth[u], depth[v]);

            if u == v {
                continue;
            }

            assert_eq!(par_edge[u].len(), par_edge[v].len());
            let kmax = par_edge[u].len();
            for k in (0..kmax).rev() {
                if par_vertex[u].len() <= k {
                    continue;
                }

                let nu = par_vertex[u][k];
                let nv = par_vertex[v][k];

                // Climb 2^k edges if u and v don't come to equal.
                if nu != nv {
                    result_graph
                        .add_edge(par_edge[u][k], arms[i])
                        .add_edge(par_edge[v][k], arms[i]);
                    u = nu;
                    v = nv;
                }
                assert_eq!(depth[u], depth[v]);
            }
            assert_ne!(u, v);

            // Climb the last one edge to make u and v equal.
            result_graph
                .add_edge(par_edge[u][0], arms[i])
                .add_edge(par_edge[v][0], arms[i]);
            u = par_vertex[u][0];
            v = par_vertex[v][0];
            assert_eq!(u, v);
        }

        result_graph
    }
}

impl RandomSample for CircuitMatroid {
    fn sample(arm_num: usize) -> Self {
        let mut rng = rand::thread_rng();
        // let vnum = rng.gen_range((arm_num / 4)..(arm_num / 3));
        let vnum = arm_num * 2 / 3;

        let mut graph = Graph::new(vnum);

        // Build a spanning tree.
        {
            let mut us: Vec<usize> = vec![0];
            let mut vs: Vec<usize> = (1..vnum).collect();
            for _ in 0..(vnum - 1) {
                let ui = rng.gen_range(0..us.len());
                let vi = rng.gen_range(0..vs.len());

                let mut u = us[ui];
                let mut v = vs[vi];
                if u > v {
                    swap(&mut u, &mut v);
                }
                graph.add_edge(u, v);

                us.push(vs[vi]);
                vs.swap_remove(vi);
            }
        }

        // Add edges randomly.
        while graph.get_edges().len() < arm_num {
            let mut u = rng.gen_range(0..vnum);
            let mut v = rng.gen_range(0..vnum);

            if u > v {
                swap(&mut u, &mut v);
            }
            if u != v && !graph.get_edges().contains(&(u, v)) {
                graph.add_edge(u, v);
            }
        }

        CircuitMatroid::new(&graph)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        arms::Weights,
        structure::{circuit_matroid::CircuitMatroid, CombinatorialStructure, RandomSample},
    };
    use rand::Rng;
    use std::collections::VecDeque;

    #[test]
    fn reachability_test() {
        let arm_num = 1000;
        let structure = CircuitMatroid::sample(arm_num);

        let mut rng = rand::thread_rng();
        let weights: Weights = (0..arm_num).map(|_| rng.gen()).collect();
        let opt_arms = structure.optimal(&weights).unwrap();

        let mut in_opt = vec![false; arm_num];
        for &i in &opt_arms {
            in_opt[i] = true;
        }

        let rgraph = structure.reachability_graph(&opt_arms);
        let rvnum = rgraph.get_vnum();

        let mut adj = vec![Vec::<usize>::new(); rvnum];
        for (u, v) in rgraph.get_edges() {
            adj[u].push(v);
        }

        // The arms in the optimal superarm and reachable to the vertex `v`.
        // These arms should be in the fundamental circuit of `v`.
        let mut reachable_from = vec![Vec::<usize>::new(); rvnum];
        for s in 0..arm_num {
            if !in_opt[s] {
                continue;
            }

            let mut visited = vec![false; rvnum];
            let mut queue = VecDeque::<usize>::new();

            visited[s] = true;
            queue.push_back(s);

            while let Some(v) = queue.pop_front() {
                if v < arm_num && !in_opt[v] {
                    reachable_from[v].push(s);
                }

                for &u in &adj[v] {
                    if !visited[u] {
                        visited[u] = true;
                        queue.push_back(u);
                    }
                }
            }
        }

        let edges = structure.graph.get_edges();
        for unopt_arm in 0..arm_num {
            if in_opt[unopt_arm] {
                continue;
            }

            let mut path = vec![Vec::<usize>::new(); arm_num];
            let mut visited = vec![true; arm_num];
            for &opt_arm in &reachable_from[unopt_arm] {
                let (u, v) = edges[opt_arm];
                path[u].push(v);
                path[v].push(u);
                visited[u] = false;
                visited[v] = false;
            }

            let (mut s, g) = edges[unopt_arm];
            let mut prev = s;
            visited[s] = true;
            while s != g {
                let mut nexts = Vec::new();
                for &v in &path[s] {
                    if v != prev {
                        nexts.push(v);
                    }
                }
                assert_eq!(nexts.len(), 1, "Not a path.");

                let next = nexts[0];
                prev = s;
                s = next;
                visited[s] = true;
            }

            assert!(visited.iter().all(|&b| b), "Not connected.");
        }
    }
}
