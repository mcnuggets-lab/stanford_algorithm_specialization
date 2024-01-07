use std::collections::HashSet;

const MAX_DIST: f64 = 1000000.0;

fn euclidean_distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}

fn tsp_nn_heuristic(coordinates: Vec<(f64, f64)>) -> f64 {
    let mut visited_vertices: HashSet<usize> = HashSet::new();
    let mut res: f64 = 0.0;
    let mut cur_vertex = 0;
    visited_vertices.insert(cur_vertex);

    while visited_vertices.len() < coordinates.len() {
        let mut min_sq_dist = MAX_DIST.powi(2);
        let mut min_i: usize = cur_vertex;
        for i in (0..cur_vertex).rev() {
            if visited_vertices.contains(&i) {
                continue;
            }
            let x_diff = (coordinates[cur_vertex].0 - coordinates[i].0).powi(2);
            if x_diff > min_sq_dist {
                // as the coordinates are sorted in x-coordinates, it is safe to break when the 
                // x-distance is larger than the min
                break;
            }
            let y_diff = (coordinates[cur_vertex].1 - coordinates[i].1).powi(2);
            let sq_dist = x_diff + y_diff;
            if sq_dist <= min_sq_dist {  // use <= here to ensure min index
                min_i = i;
                min_sq_dist = sq_dist;
            }
        }
        for i in cur_vertex+1..coordinates.len() {
            if visited_vertices.contains(&i) {
                continue;
            }
            let x_diff = (coordinates[cur_vertex].0 - coordinates[i].0).powi(2);
            if x_diff > min_sq_dist {
                // as the coordinates are sorted in x-coordinates, it is safe to break when the 
                // x-distance is larger than the min
                break;
            }
            let y_diff = (coordinates[cur_vertex].1 - coordinates[i].1).powi(2);
            let sq_dist = x_diff + y_diff;
            if sq_dist < min_sq_dist {  // use > here to ensure min index
                min_i = i;
                min_sq_dist = sq_dist;
            }
        }
        cur_vertex = min_i;
        res += min_sq_dist.sqrt();
        visited_vertices.insert(cur_vertex);
    }
    res += euclidean_distance(coordinates[cur_vertex], coordinates[0]);  // close the loop

    res
}

fn main() {

    let file_path: &str = "nn.txt";
    let contents: String = std::fs::read_to_string(file_path).unwrap();
    let mut lines_iter = contents.lines();
    let _num_vertices: usize = str::parse(lines_iter.next().unwrap()).unwrap();

    let mut coordinates: Vec<(f64, f64)> = Vec::new();

    for content in lines_iter {
        let line: Vec<&str> = content.split(" ").collect();
        coordinates.push((str::parse(line[1]).unwrap(), str::parse(line[2]).unwrap()));
    }

    println!("{:?}", tsp_nn_heuristic(coordinates));  // 1203406.5012708856

}
