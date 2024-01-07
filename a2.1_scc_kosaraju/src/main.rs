use std::collections::{HashMap, HashSet};
use std::thread;

type Graph = HashMap<usize, HashSet<usize>>;

const STACK_SIZE: usize = 1024 * 1024 * 1024;
const NUM_VERTICES: usize = 875714;

fn dfs(graph: &Graph, order: Vec<usize>) -> ([usize; NUM_VERTICES], Vec<usize>) {
    let mut leaders: [usize; NUM_VERTICES] = [0; NUM_VERTICES];
    let mut finishing_times: Vec<usize> = Vec::new();

    for i in order.iter().rev() {
        if leaders[i-1] == 0 {
            let leader = *i;
            dfs_subroutine(&graph, *i, leader, &mut leaders, &mut finishing_times);
        }
    }

    (leaders, finishing_times)
}

fn dfs_subroutine(graph: &Graph, source: usize, leader: usize, leaders: &mut [usize; NUM_VERTICES], finishing_times: &mut Vec<usize>) {
    leaders[source-1] = leader;
    match graph.get(&source) {
        Some(adj_set) => {
            for j in adj_set {
                if leaders[*j-1] == 0 {
                    dfs_subroutine(graph, *j, leader, leaders, finishing_times)
                }
            }
        },
        None => {}
    };
    finishing_times.push(source);
}

fn kosaraju_scc(graph: &Graph, graph_rev: &Graph) -> HashMap<usize, usize> {
    let (_old_leaders, finishing_times) = dfs(&graph_rev, (1..=NUM_VERTICES).collect());
    let (leaders, _new_finishing_times) = dfs(&graph, finishing_times);

    let mut counter: HashMap<usize, usize> = HashMap::new();
    for leader in leaders {
        let count = counter.entry(leader).or_insert(0);
        *count += 1;
    }

    counter
}

fn run() {

    let file_path = "SCC.txt";
    let contents = std::fs::read_to_string(file_path).unwrap();

    let mut graph: Graph = HashMap::new();
    let mut graph_rev: Graph = HashMap::new();

    for content in contents.lines() {
        let line: Vec<&str> = content.split(" ").collect();
        let key = str::parse(line[0]).unwrap();
        let value = str::parse(line[1]).unwrap();

        let adj_set = graph.entry(key).or_insert(HashSet::new());
        adj_set.insert(value);

        let adj_set_rev = graph_rev.entry(value).or_insert(HashSet::new());
        adj_set_rev.insert(key);
    }

    let counter = kosaraju_scc(&graph, &graph_rev);
    let mut counter_vec: Vec<_> = counter.iter().collect();
    counter_vec.sort_by(|a, b| a.1.cmp(b.1).reverse());
    println!("{:?}", &counter_vec[0..5]);  // 434821,968,459,313,211

}

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
