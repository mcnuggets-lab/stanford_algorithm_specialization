use std::collections::HashMap;

pub struct UnionFind {
    leader: HashMap<usize, usize>,
    rank: HashMap<usize, usize>,
    pub num_partitions: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut leader: HashMap<usize, usize> = HashMap::new();
        for i in 0..n {
            leader.insert(i, i);
        }
        let mut rank: HashMap<usize, usize> = HashMap::new();
        for i in 0..n {
            rank.insert(i, 0);
        }

        Self {
            leader,
            rank,
            num_partitions: n,
        }
    }

    /// Find the cluster label that a given n belongs to.
    pub fn find(&mut self, n: usize) -> usize {
        let mut trace: Vec<usize> = Vec::new();
        let mut lead: usize = n;
        while trace.is_empty() || lead != *trace.last().unwrap() {
            trace.push(lead);
            lead = self.leader[&lead];
        }

        // path compression
        for i in trace {
            self.leader.insert(i, lead);
        }
        
        lead
    }

    /// Merge the cluster x with cluster y, return if they were in the same cluster before merge.
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let lead_x = &self.find(x);
        let lead_y = &self.find(y);
        if lead_x != lead_y {
            // merge the two clusters
            let rank_x = &self.rank[lead_x];
            let rank_y = &self.rank[lead_y];
            if rank_x < rank_y {
                self.leader.insert(*lead_x, *lead_y);
            }
            else {
                self.leader.insert(*lead_y, *lead_x);
                if rank_x == rank_y {
                    *self.rank.get_mut(lead_x).unwrap() += 1;
                }
            }
            self.num_partitions -= 1;
        }

        lead_x == lead_y
    }

    // pub fn clusters(&mut self) -> HashMap<usize, usize> {
    //     let mut res: HashMap<usize, usize> = HashMap::new();
    //     for i in 0..self.leader.len() {
    //         let lead_i = self.find(i);
    //         let ent = res.entry(lead_i).or_insert(0);
    //         *ent += 1;
    //     }
    //     return res;
    // }
}