use std::collections::{HashMap, HashSet};

mod union_find;

fn main() {

        let file_path: &str = "clustering_big.txt";
        let contents = std::fs::read_to_string(file_path).unwrap();
        let mut lines_iter = contents.lines();
        let first_line: Vec<&str> = lines_iter.next().unwrap().split(" ").collect();
        let _num_nodes: usize = str::parse(first_line[0]).unwrap();
        let num_bits: usize = str::parse(first_line[1]).unwrap();

        // read in contents from file
        let mut nodes: Vec<usize> = Vec::new();
        for content in lines_iter {
            let bstring: &str = &content.replace(" ", "");
            nodes.push(usize::from_str_radix(bstring, 2).unwrap());
        }

        // build a map of bits to node number
        let mut int2nodes: HashMap<usize, HashSet<usize>> = HashMap::new();
        for ind in 0..nodes.len() {
            let node_list = int2nodes.entry(nodes[ind]).or_insert(HashSet::new());
            node_list.insert(ind);
        }

        // create masks
        let mut mask1: Vec<usize> = Vec::new();
        for i in 0..num_bits {
            mask1.push(1 << i);
        }

        let mut mask2: Vec<usize> = Vec::new();
        for i in 0..num_bits {
            for j in i+1..num_bits {
                mask2.push((1 << i) + (1 << j));
            }
        }

        let mut uf: union_find::UnionFind = union_find::UnionFind::new(nodes.len());
        for ind in 0..nodes.len() {
            // connect nodes with distance 0
            for dst in &int2nodes[&nodes[ind]] {
                uf.union(ind, *dst);
            }

            // connect nodes with distance 1
            for mask in &mask1 {
                let new_int = mask ^ nodes[ind];
                if int2nodes.contains_key(&new_int) {
                    for dst in &int2nodes[&new_int] {
                        uf.union(ind, *dst);
                    }
                }
            }

            // connect nodes with distance 2
            for mask in &mask2 {
                let new_int = mask ^ nodes[ind];
                if int2nodes.contains_key(&new_int) {
                    for dst in &int2nodes[&new_int] {
                        uf.union(ind, *dst);
                    }
                }
            }

        }

        println!("{:?}", uf.num_partitions);
    
}
