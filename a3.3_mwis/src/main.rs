use std::collections::HashSet;

fn main() {

        let file_path: &str = "mwis.txt";
        let contents = std::fs::read_to_string(file_path).unwrap();
        let mut lines_iter = contents.lines();
        let _num_nodes: usize = str::parse(lines_iter.next().unwrap()).unwrap();

        let mut path_graph: Vec<usize> = Vec::new();
        for content in lines_iter {
            path_graph.push(str::parse(content).unwrap());
        }

        // run dp for max-weight IS with reconstruction
        let mut dp_array: Vec<usize> = Vec::new();
        dp_array.push(0);
        dp_array.push(path_graph[0]);
        for i in 1..path_graph.len() {
            if dp_array[i] >= dp_array[i - 1] + path_graph[i] {
                dp_array.push(dp_array[i]);
            }
            else {
                dp_array.push(dp_array[i - 1] + path_graph[i]);
            }
        }

        // path reconstruction
        let mut path_vertices: HashSet<usize> = HashSet::new();
        let mut i = path_graph.len();
        while i >= 2 {
            if dp_array[i - 1] >= dp_array[i - 2] + path_graph[i - 1] {
                i -= 1;
            }
            else {
                path_vertices.insert(i - 1);
                i -= 2;
            }
        }
        if !path_vertices.contains(&1) {
            path_vertices.insert(0);
        }

        let req_indices = [1, 2, 3, 4, 17, 117, 517, 997];
        for i in req_indices {
            print!("{:?}", if path_vertices.contains(&(i - 1)) { 1 } else { 0 });
        }
        println!("");
    
}
