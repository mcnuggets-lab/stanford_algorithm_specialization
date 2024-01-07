use std::{collections::{HashMap, HashSet, BinaryHeap}, cmp::Reverse};
use indicatif::ProgressBar;

type Graph = HashMap<usize, HashMap<usize, isize>>;

const MAX_DIST: isize = 1000000;

pub fn add_source_vertex(graph: &Graph) -> Graph {
    let mut new_graph = graph.clone();
    let new_adj_list = new_graph.entry(graph.len() + 1).or_insert(HashMap::new());
    for i in 1..=graph.len() {
        new_adj_list.insert(i, 0);
    }
    new_graph
}

// This implementation is slightly different from the course notes.
pub fn bellman_ford(graph: &Graph, source: usize) -> Option<Vec<isize>> {
    let mut dp_array = vec![MAX_DIST; graph.len()];
    dp_array[source - 1] = 0;

    let bar = ProgressBar::new(graph.len() as u64);
    for _i in 1..=graph.len() {
        let last_array = dp_array.clone();
        for src in graph.keys() {
            for (dst, edge_length) in graph[src].iter() {
                let dist = dp_array[*src-1] + edge_length;
                if dist < dp_array[*dst-1] {
                    dp_array[*dst-1] = dist;
                }
            }
        }
        if last_array == dp_array {
            return Some(dp_array);
        }
        bar.inc(1);
    }

    None
}

// copied from assignment 2 of Course 2
fn dijkstra(graph: &Graph, source: usize) -> HashMap<usize, isize> {
    let mut res: HashMap<usize, isize> = HashMap::new();

    // initialize heap
    let mut explored_vertices: HashSet<usize> = HashSet::new();
    let mut unexplored_vertices: BinaryHeap<_> = BinaryHeap::new();  // elements are of the form (key, vertex)
    for i in graph.keys() {
        if *i == source {
            unexplored_vertices.push(Reverse((0, *i)));
            res.insert(*i, 0);
        }
        else {
            unexplored_vertices.push(Reverse((MAX_DIST, *i)));
            res.insert(*i, MAX_DIST);
        }
    }

    while explored_vertices.len() < graph.len() {
        let cur = unexplored_vertices.pop().unwrap().0;
        let (cur_dist, cur_vertex) = cur;
        if explored_vertices.contains(&cur_vertex) {
            continue;
        }
        explored_vertices.insert(cur_vertex);
        res.insert(cur_vertex, cur_dist);
        for (edge, dist) in &graph[&cur_vertex] {
            let new_dist = cur_dist + dist;
            if res[&edge] > new_dist {
                res.insert(*edge, new_dist);
                unexplored_vertices.push(Reverse((new_dist, *edge)));
            }
        }
    }

    res
}

pub fn johnson(graph: &Graph) -> Option<Vec<isize>> {
    // get the reweights and check if there are negative cycles
    let amended_graph: Graph = add_source_vertex(&graph);
    let reweights = bellman_ford(&amended_graph, graph.len() + 1);

    match reweights {
        None => { None }
        Some(ar) => {
            let bar = ProgressBar::new(graph.len() as u64);

            // get the reweighted graph
            let mut new_graph: Graph = HashMap::new();
            for vertex in graph.keys() {
                new_graph.insert(*vertex, graph[&vertex].iter().map(|(k, v)| (*k, v + ar[vertex-1] - ar[k-1])).collect());
            }

            // run dijkstra on every vertices of the graph
            let mut res: Vec<isize> = vec![MAX_DIST; graph.len() * graph.len()];
            for src in new_graph.keys() {
                // reweight back to their shortest distances
                let res_dist: HashMap<usize, isize> = dijkstra(&new_graph, *src).iter().map(
                    |(k, v)| (*k, v - ar[src-1] + ar[k-1])
                ).collect();
                for dst in res_dist.keys() {
                    res[(src-1) * graph.len() + (dst-1)] = res_dist[&dst];
                }
                bar.inc(1);
            }
            Some(res)
        }
    }
}

pub fn floyd_warshall(graph: &Graph) -> Option<Vec<isize>> {
    // initialization
    let mut dp_array = vec![MAX_DIST; graph.len() * graph.len()];
    for i in 0..graph.len() {
        dp_array[i * graph.len() + i] = 0;
    }
    for i in graph.keys() {
        for (k, v) in &graph[&i] {
            dp_array[(*i-1) * graph.len() + *k-1] = *v;
        }
    }

    // running the algorithm
    let bar = ProgressBar::new(graph.len() as u64);
    for k in 1..=graph.len() {
        for i in 1..=graph.len() {
            if dp_array[(i-1) * graph.len() + k-1] == MAX_DIST {
              continue;
            }
            for j in 1..=graph.len() {
                let dist: isize = dp_array[(i-1) * graph.len() + k-1] + dp_array[(k-1) * graph.len() + j-1];
                if dist < dp_array[(i-1) * graph.len() + j-1] {
                    dp_array[(i-1) * graph.len() + j-1] = dist;
                }
            }
        }

        // early return if contains negative cycle
        for i in 0..graph.len() {
            if dp_array[i * graph.len() + i] < 0 {
                return None;
            }
        }
        bar.inc(1);
    }

    Some(dp_array.clone())
}
