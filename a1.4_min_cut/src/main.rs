use indicatif::ProgressBar;
use rand::prelude::*;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use threadpool::ThreadPool;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::collections::HashMap;

type Graph = HashMap<i32, HashMap<i32, usize>>;

fn edge_contraction(graph: &mut Graph, edge: (i32, i32)) {
    assert!(graph[&edge.0].contains_key(&edge.1));

    let adj0 = graph.remove(&edge.0).unwrap();

    for (i, v) in &adj0 {
        // for each neighbor of edge.0, replace it with edge.1
        graph.get_mut(&i).unwrap().remove(&edge.0);
        let cur_value = graph.get_mut(&i).unwrap().entry(edge.1).or_insert(0);
        *cur_value += v;

        let base_value = graph.get_mut(&edge.1).unwrap().entry(*i).or_insert(0);
        *base_value += v;
    }

    graph.get_mut(&edge.1).unwrap().remove(&edge.1);
}

fn karger_min_cut_subroutine(graph: &mut Graph, seed: u64) -> usize {
    let mut vertices: Vec<i32> = graph.keys().cloned().collect();
    let mut vertices_count = vertices.len();

    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    while vertices_count > 2 {
        // select edge for contraction
        let vertex_index = rng.gen_range(0..vertices_count);
        let edge0: i32 = vertices[vertex_index];
        let possible_endpoints: Vec<i32> = graph[&edge0].keys().cloned().collect();
        let edge1: i32 = possible_endpoints[rng.gen_range(0..possible_endpoints.len())];
        edge_contraction(graph, (edge0, edge1));

        // update the list of possible vertices by moving the used vertex to last positions
        vertices_count -= 1;
        (vertices[vertices_count], vertices[vertex_index]) =
            (vertices[vertex_index], vertices[vertices_count])
    }

    graph[&vertices[0]][&vertices[1]]
}

fn karger_min_cut(graph: &Graph, num_trials: u64) -> usize {
    let bar = Arc::new(ProgressBar::new(num_trials));

    let n_workers = 8;
    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel();

    for seed in 0..num_trials {
        let new_graph = graph.clone();
        let bar_ref = Arc::clone(&bar);

        let tx = tx.clone();
        let handle = move |graph: &mut Graph, seed, bar: &ProgressBar| {
            let ans = karger_min_cut_subroutine(graph, seed);
            bar.inc(1);
            ans
        };
        pool.execute(move|| {
            let _ = tx.send(handle(&mut new_graph.clone(), seed, &bar_ref));
        });

    }

    rx.iter().take(num_trials as usize).fold(graph.len() * graph.len(), |a, b| std::cmp::min(a, b))
}

fn main() {
    let file_path = "kargerMinCut.txt";
    let contents = std::fs::read_to_string(file_path).unwrap();

    let mut graph: Graph = HashMap::new();
    for content in contents.lines() {
        let line: Vec<&str> = content.split("\t").collect();
        let key = str::parse(line[0]).unwrap();
        graph.insert(key, HashMap::new());
        let adj_set = graph.get_mut(&key).unwrap();
        for s in &line[1..] {
            if s.len() > 0 {
                adj_set.insert(str::parse(s).unwrap(), 1);
            }
        }
    }

    println!("{:?}", karger_min_cut(&graph, 100000)); // 17
}
