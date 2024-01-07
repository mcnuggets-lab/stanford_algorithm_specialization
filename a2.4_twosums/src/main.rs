use std::collections::{BTreeSet, HashSet};

fn twosums(inputs: Vec<i64>, lb: i64, ub: i64) -> usize {
    let mut res: HashSet<i64> = HashSet::new();
    let mut treeset: BTreeSet<i64> = BTreeSet::new();

    // load data into the tree set
    for i in inputs {
        treeset.insert(i);
    }

    // main loop
    for i in &treeset {
        for elem in treeset.range(lb-i..=ub-i) {
            res.insert(i + elem);
        }
    }

    res.len()
}

fn main() {
    let file_path = "algo1-programming_prob-2sum.txt";
    let contents = std::fs::read_to_string(file_path).unwrap();

    let mut inputs: Vec<i64> = Vec::new();
    for content in contents.lines() {
        inputs.push(str::parse::<i64>(content).unwrap());
    }

    println!("{:?}", twosums(inputs, -10000, 10000));  // 427

}
