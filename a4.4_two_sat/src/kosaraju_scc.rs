use std::collections::{HashMap, HashSet};

pub struct Graph {
    pub size: usize,
    pub graph: HashMap<isize, HashSet<isize>>
}

fn dfs(graph: &Graph, order: Vec<isize>) -> (HashMap<isize, isize>, Vec<isize>) {
    let mut leaders = HashMap::new();
    let mut finishing_times: Vec<isize> = Vec::new();

    for i in order.iter().rev() {
        if !leaders.contains_key(i) {
            let leader = *i;
            dfs_subroutine(&graph, *i, leader, &mut leaders, &mut finishing_times);
        }
    }

    (leaders, finishing_times)
}

fn dfs_subroutine(graph: &Graph, source: isize, leader: isize, leaders: &mut HashMap<isize, isize>, finishing_times: &mut Vec<isize>) {
    leaders.insert(source, leader);
    match graph.graph.get(&source) {
        Some(adj_set) => {
            for j in adj_set {
                if !leaders.contains_key(j) {
                    dfs_subroutine(graph, *j, leader, leaders, finishing_times)
                }
            }
        },
        None => {}
    };
    finishing_times.push(source);
}

pub fn kosaraju_scc(graph: &Graph) -> HashMap<isize, isize> {
    // build reverse graph
    let mut graph_rev: Graph = Graph {
        size: graph.size,
        graph: HashMap::new()
    };
    for src in graph.graph.keys() {
        for u in &graph.graph[src] {
            graph_rev.graph.entry(*u).or_insert(HashSet::new()).insert(*src);
        }
    }

    // 2-pass dfs for strongly connected components
    let mut order: Vec<isize> = (-((graph.size/2) as isize)..=-1).collect();
    order = [order, (1..=(graph.size/2) as isize).collect()].concat();
    let (_old_leaders, finishing_times) = dfs(&graph_rev, order);
    let (leaders, _new_finishing_times) = dfs(&graph, finishing_times);

    leaders
}