use std::collections::{HashMap, HashSet};

mod kosaraju_scc;
use kosaraju_scc::Graph;

fn build_implication_graph(clauses: &Vec<(isize, isize)>) -> Graph {
    let mut imp_graph = Graph {
        size: clauses.len() * 2,
        graph: HashMap::new()
    };

    for (v1, v2) in clauses {
        imp_graph.graph.entry(-v1).or_insert(HashSet::new()).insert(*v2);
        imp_graph.graph.entry(-v2).or_insert(HashSet::new()).insert(*v1);
    }

    imp_graph
}

fn two_sat_scc(clauses: &Vec<(isize, isize)>) -> bool {
    let imp_graph = build_implication_graph(clauses);
    let leaders = kosaraju_scc::kosaraju_scc(&imp_graph);
    for i in 1..=clauses.len() as isize {
        if leaders[&i] == leaders[&(-i)] {
            return false;
        }
    }
    true
}

fn main() {

    for i in 1..=6 {
        let file_path: String = format!("2sat{}.txt", i);
        let contents: String = std::fs::read_to_string(file_path).unwrap();
        let mut lines_iter = contents.lines();
        let _num_clauses: usize = str::parse(lines_iter.next().unwrap()).unwrap();

        let mut clauses: Vec<(isize, isize)> = Vec::new();

        for content in lines_iter {
            let line: Vec<&str> = content.split(" ").collect();
            clauses.push((str::parse(line[0]).unwrap(), str::parse(line[1]).unwrap()));
        }

        println!("{:?}", two_sat_scc(&clauses));
    }

}
