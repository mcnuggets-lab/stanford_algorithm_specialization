use std::collections::HashMap;

mod shortest_paths_utils;

type Graph = HashMap<usize, HashMap<usize, isize>>;

fn main() {

    let file_path = "large.txt";
    let contents = std::fs::read_to_string(file_path).unwrap();
    let mut lines_iter = contents.lines();
    let splits: Vec<&str> = lines_iter.next().unwrap().split(" ").collect();
    let _num_vertices: usize = str::parse(splits[0]).unwrap();
    let _num_edges: usize = str::parse(splits[1]).unwrap();

    let mut graph: Graph = HashMap::new();

    for content in lines_iter {
        let line: Vec<&str> = content.split(" ").collect();
        let src: usize = str::parse(line[0]).unwrap();
        let dst: usize = str::parse(line[1]).unwrap();
        let weight: isize = str::parse(line[2]).unwrap();
        
        let adj_list = graph.entry(src).or_insert(HashMap::new());
        adj_list.insert(dst, weight);
    }

    // let res = shortest_paths_utils::floyd_warshall(&graph);

    // let res = shortest_paths_utils::johnson(&graph);

    // First step of Johnson's algorithm is good enough to determine the shortest shortest path 
    // if any edge has negative weight. This is ultra fast for large graphs.
    let amended_graph: Graph = shortest_paths_utils::add_source_vertex(&graph);
    let res = shortest_paths_utils::bellman_ford(&amended_graph, graph.len() + 1);

    let min_length = match res {
        Some(ar) => ar.iter().min().unwrap().to_string(),
        None => String::from("Graph contains negative cycles.")
    };

    println!("{:?}", min_length);

    // g1: has neg cycle
    // g2: has neg cycle
    // g3: no neg cycle, -19 <takes less than 1 min>
    // large: no neg cycle, -6 <takes several hours>

}
