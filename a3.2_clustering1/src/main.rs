mod union_find;

use std::collections::HashMap;

struct Graph {
    size: usize,
    graph: HashMap<(usize, usize), usize>,
}

fn kruskal(graph: &Graph, num_clusters: usize) -> usize {
    let mut edges: Vec<(&(usize, usize), &usize)> = graph.graph.iter().collect();
    edges.sort_by_key(|x| (x.1, x.0.0));
    let mut edges_iter = edges.into_iter();
    let mut uf = union_find::UnionFind::new(graph.size);

    let mut weight: usize = 0;
    while uf.num_partitions >= num_clusters {
        let ((src, dst), cur_weight) = edges_iter.next().unwrap();
        if !uf.union(*src, *dst) {
            weight = *cur_weight;
        }
    }
    
    weight
}

fn main() {

        let file_path: &str = "clustering1.txt";
        let contents = std::fs::read_to_string(file_path).unwrap();
        let mut lines_iter = contents.lines();
        let num_vertices: usize = str::parse(lines_iter.next().unwrap()).unwrap();

        let mut graph = Graph {
            size: num_vertices,
            graph: HashMap::new(),
        };

        for content in lines_iter {
            let line: Vec<&str> = content.split(" ").collect();
            let src: usize = str::parse(line[0]).unwrap();
            let dst: usize = str::parse(line[1]).unwrap();
            let weight: usize = str::parse(line[2]).unwrap();
            
            graph.graph.insert((src, dst), weight);
        }

        println!("{:?}", kruskal(&graph, 4));  // 106
    
}
