use std::collections::HashSet;
use indicatif::ProgressBar;

const MAX_DIST: f32 = 1000000.0;

fn euclidean_distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}

/// For a bitset represented by an integer n, give the elements it contains as a set.
fn bitset_to_set(n: usize, vec_length: usize) -> HashSet<usize> {
    let mut res: HashSet<usize> = HashSet::new();
    for k in 0..vec_length {
        if n ^ (1 << k) < n {
            res.insert(k);
        }
    }
    res
}

fn tsp_exact(dists: Vec<Vec<f32>>) -> f32 {
    let mut dp_array: Vec<Vec<f32>> = vec![vec![MAX_DIST; dists.len()]; 1 << (dists.len()-1)];
    dp_array[0][0] = 0.0;
    let bar = ProgressBar::new((dists.len() - 1) as u64);
    for m in 1..dists.len() {
        let mut s = (1 << m) - 1;
        while s < 1 << (dists.len()-1) {
            let elements = bitset_to_set(s, dists.len() - 1);
            for &j in &elements {
                let mut cur_min = dp_array[s ^ (1 << j)][0] + dists[j+1][0];
                for &k in &elements {
                    if j == k {
                        continue;
                    }
                    let cur_dist = dp_array[s ^ (1 << j)][k+1] + dists[j+1][k+1];
                    if cur_dist < cur_min {
                        cur_min = cur_dist;
                    }
                }
                dp_array[s][j+1] = cur_min;
            }
            s = next_bit(s);
        }
        bar.inc(1);
    }

    let mut res = MAX_DIST;
    for (ind, tour_length) in dp_array[(1 << (dists.len()-1)) - 1].iter().enumerate() {
        if ind == 0 {
            continue; 
        }
        let cycle_length = tour_length + dists[ind][0];
        if cycle_length < res {
            res = cycle_length;
        }
    }
    res
}

fn next_bit(v: usize) -> usize {
    let v = v as isize;
    let t = (v | (v - 1)) + 1;  
    (t | ((((t & -t) / (v & -v)) >> 1) - 1)) as usize
}

fn main() {

    let file_path: &str = "tsp.txt";
    let contents: String = std::fs::read_to_string(file_path).unwrap();
    let mut lines_iter = contents.lines();
    let num_vertices: usize = str::parse(lines_iter.next().unwrap()).unwrap();

    let mut coordinates: Vec<(f32, f32)> = Vec::new();

    for content in lines_iter {
        let line: Vec<&str> = content.split(" ").collect();
        coordinates.push((str::parse(line[0]).unwrap(), str::parse(line[1]).unwrap()));
    }

    let mut dists: Vec<Vec<f32>> = vec![vec![0.0; num_vertices]; num_vertices];
    for i in 0..num_vertices {
        for j in 0..num_vertices {
            if j > i {
                dists[i][j] = euclidean_distance(coordinates[i], coordinates[j]);
            }
            else if j < i {
                dists[i][j] = dists[j][i];
            }
            else {
                dists[i][j] = 0.0;
            }
        }
    }

    println!("{:?}", tsp_exact(dists));  // 26442.727

}
