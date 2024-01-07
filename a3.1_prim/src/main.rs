use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::Reverse;

type Graph = HashMap<usize, HashMap<usize, i64>>;
type Tree = HashMap<(usize, usize), i64>;

fn prim(graph: &mut Graph) -> Tree {
    let mut tree: Tree = HashMap::new();
    let mut visited_vertices: HashSet<usize> = HashSet::new();
    let mut unvisited_vertices: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();

    let mut source: usize = 1;
    visited_vertices.insert(source);

    while visited_vertices.len() < graph.len() {
        for (dst, weight) in &graph[&source] {
            if !visited_vertices.contains(dst) {
                unvisited_vertices.push((Reverse(*weight), *dst));
            }
        }
        let mut weight;
        let mut new_dst;
        loop {
            (weight, new_dst) = unvisited_vertices.pop().unwrap();
            if !visited_vertices.contains(&new_dst) {
                break;
            }
        } 
        
        tree.insert((source, new_dst), weight.0);
        source = new_dst;
        visited_vertices.insert(source);
    }

    tree
}

fn main() {

    let file_path = "edges.txt";
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
        let weight: i64 = str::parse(line[2]).unwrap();
        
        let adj_list = graph.entry(src).or_insert(HashMap::new());
        adj_list.insert(dst, weight);

        // graph is undirected
        let adj_list_2 = graph.entry(dst).or_insert(HashMap::new());
        adj_list_2.insert(src, weight);
    }

    println!("{:?}", prim(&mut graph).values().sum::<i64>());

}
