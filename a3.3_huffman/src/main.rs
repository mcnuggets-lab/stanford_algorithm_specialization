use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    weight: usize,
    label: Option<usize>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn huffman(weights: Vec<usize>) -> Node {
    let mut weights_pq: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    for i in 0..weights.len() {
        weights_pq.push(Reverse(Node {weight: weights[i], label: Some(i), left: None, right: None}))
    }

    while weights_pq.len() > 1 {
        let node1: Node = weights_pq.pop().unwrap().0;
        let node2: Node = weights_pq.pop().unwrap().0;
        let combined_node: Node = Node {
            weight: node1.weight + node2.weight, label: None, left: Some(Box::new(node1)), right: Some(Box::new(node2)),
        };
        weights_pq.push(Reverse(combined_node));
    }

    weights_pq.pop().unwrap().0
}

fn max_depth(root: Option<&Node>) -> usize {
    match root {
        Some(r) => { 1 + std::cmp::max(max_depth(r.left.as_deref()), max_depth(r.right.as_deref())) },
        None => 0,
    }
}

fn min_depth(root: Option<&Node>) -> usize {
    match root {
        Some(r) => { 1 + std::cmp::min(min_depth(r.left.as_deref()), min_depth(r.right.as_deref())) },
        None => 0,
    }
}

fn main() {

        let file_path: &str = "huffman.txt";
        let contents = std::fs::read_to_string(file_path).unwrap();
        let mut lines_iter = contents.lines();
        let _num_nodes: usize = str::parse(lines_iter.next().unwrap()).unwrap();

        let mut nodes: Vec<usize> = Vec::new();
        for content in lines_iter {
            nodes.push(str::parse(content).unwrap());
        }

        let huffman_root: Node = huffman(nodes);

        let min_length = min_depth(Some(&huffman_root));
        let max_length = max_depth(Some(&huffman_root));

        println!("{:?}", (min_length - 1, max_length - 1));  // 10100110
    
}
