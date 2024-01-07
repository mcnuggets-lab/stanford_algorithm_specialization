use std::cmp::Reverse;

fn schedule_diff(mut jobs: Vec<(usize, usize)>) -> usize {
    jobs.sort_by_key(|a| Reverse((a.0 as i64 - a.1 as i64, a.0)));
    let mut res: usize = 0;
    let mut time: usize = 0;
    for (weight, length) in jobs {
        time += length;
        res += weight * time;
    }

    res
}

fn schedule_ratio(mut jobs: Vec<(usize, usize)>) -> usize {
    jobs.sort_by(|a, b| (a.1 * b.0).cmp(&(a.0 * b.1)));
    let mut res: usize = 0;
    let mut time: usize = 0;
    for (weight, length) in jobs {
        time += length;
        res += weight * time;
    }

    res
}

fn main() {

    let file_path = "jobs.txt";
    let contents = std::fs::read_to_string(file_path).unwrap();
    let mut lines_iter = contents.lines();
    let _num_ints: usize = str::parse(lines_iter.next().unwrap()).unwrap();

    let mut jobs: Vec<(usize, usize)> = Vec::new();

    for content in lines_iter {
        let line: Vec<&str> = content.split(" ").collect();
        let key = str::parse(line[0]).unwrap();
        let value = str::parse(line[1]).unwrap();
        jobs.push((key, value));
    }

    println!("{:?}", schedule_diff(jobs.clone()));
    println!("{:?}", schedule_ratio(jobs.clone()));

}
