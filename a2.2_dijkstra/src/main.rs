use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

type Graph = HashMap<usize, HashMap<usize, usize>>;

const MAX_DISTANCE: usize = 100000;

fn dijkstra(graph: &Graph, source: usize) -> HashMap<usize, usize> {
    let mut res: HashMap<usize, usize> = HashMap::new();

    // initialize heap
    let mut explored_vertices: HashSet<usize> = HashSet::new();
    let mut unexplored_vertices: BinaryHeap<_> = BinaryHeap::new();  // elements are of the form (key, vertex)
    for i in graph.keys() {
        if *i == source {
            unexplored_vertices.push(Reverse((0, *i)));
            res.insert(*i, 0);
        }
        else {
            unexplored_vertices.push(Reverse((MAX_DISTANCE, *i)));
            res.insert(*i, MAX_DISTANCE);
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

fn main() {
    let file_path = "dijkstraData.txt";
    let contents = std::fs::read_to_string(file_path).unwrap();

    let mut graph: Graph = HashMap::new();
    for content in contents.lines() {
        let line: Vec<&str> = content.split("\t").collect();
        let key = str::parse(line[0]).unwrap();
        graph.insert(key, HashMap::new());
        let adj_set = graph.get_mut(&key).unwrap();
        for s in &line[1..] {
            if s.len() > 0 {
                let word: Vec<&str> = s.split(",").collect();
                adj_set.insert(str::parse(word[0]).unwrap(), str::parse(word[1]).unwrap());
            }
        }
    }

    let res = dijkstra(&graph, 1);
    println!("{:?}", res);
    for i in [7, 37, 59, 82, 99, 115, 133, 165, 188, 197] {
        println!("{:?} {:?}", i, res[&i]);
    }

}
