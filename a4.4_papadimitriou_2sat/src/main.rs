use std::collections::HashSet;

use rand::seq::SliceRandom;

fn reduce_clauses(clauses: &Vec<[isize; 2]>) -> Vec<[isize; 2]> {
    let mut res: Vec<[isize; 2]> = clauses.clone();
    loop {
        let mut pos_set: HashSet<usize> = HashSet::new();
        let mut neg_set: HashSet<usize> = HashSet::new();
        for clause in &res {
            for v in clause {
                if *v > 0 {
                    pos_set.insert(*v as usize);
                } else {
                    neg_set.insert(-*v as usize);
                }
            }
        }
        let diff_set: HashSet<&usize> = pos_set.symmetric_difference(&neg_set).collect();
        if diff_set.len() == 0 {
            return res;
        }
        res = res.into_iter().filter(|[a, b]| {
            !diff_set.contains(&(a.abs() as usize)) && !diff_set.contains(&(b.abs() as usize))
        }).collect();
    }
}

fn get_violated_clauses(assignment: &Vec<bool>, clauses: &Vec<[isize; 2]>) -> Vec<usize> {
    let mut res = Vec::new();
    for (i, [v1, v2]) in clauses.iter().enumerate() {
        let a1 = if *v1 < 0 {
            !assignment[(-v1) as usize - 1]
        } else {
            assignment[*v1 as usize - 1]
        };
        let a2 = if *v2 < 0 {
            !assignment[(-v2) as usize - 1]
        } else {
            assignment[*v2 as usize - 1]
        };
        if !(a1 || a2) {
            res.push(i);
        }
    }
    res
}

fn papadimitriou(num_variables: usize, clauses: &Vec<[isize; 2]>) -> bool {
    for _ in 0..=clauses.len().ilog2() {
        // initialize a random bool assignment for the 2-SAT problem
        let mut assignment = Vec::new();
        for _ in 0..num_variables {
            assignment.push(rand::random::<bool>());
        }

        // local search for 2*n^2 times
        for _ in 0..2 * clauses.len().pow(2) {
            let violations = get_violated_clauses(&assignment, &clauses);
            if violations.len() == 0 {
                return true;
            }
            // choose an arbitrary unsatisfied clause and flip one of its variables
            let ind1: &usize = violations.choose(&mut rand::thread_rng()).unwrap();
            if rand::random::<bool>() {
                assignment[clauses[*ind1][0].abs() as usize - 1] =
                    !assignment[clauses[*ind1][0].abs() as usize - 1];
            } else {
                assignment[clauses[*ind1][1].abs() as usize - 1] =
                    !assignment[clauses[*ind1][1].abs() as usize - 1];
            };
        }
    }
    false
}

fn main() {
    for i in 1..=6 {
        let file_path: String = format!("2sat{}.txt", i);
        let contents: String = std::fs::read_to_string(file_path).unwrap();
        let mut lines_iter = contents.lines();
        let num_variables: usize = str::parse(lines_iter.next().unwrap()).unwrap();

        let mut clauses: Vec<[isize; 2]> = Vec::new();

        for content in lines_iter {
            let line: Vec<&str> = content.split(" ").collect();
            clauses.push([str::parse(line[0]).unwrap(), str::parse(line[1]).unwrap()]);
        }

        println!("{:?}", papadimitriou(num_variables, &reduce_clauses(&clauses))); // 101100
    }
}
