impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut counter = std::collections::HashSet::new();

        for v in indices {
            let row = v[0];
            let col = v[1];

            for x in 0..m {
                let s = format!("{}{}{}", row, "-", x);
                if counter.contains(&s) {
                    counter.remove(&s);
                } else {
                    counter.insert(s);
                }
            }

            for x in 0..n {
                let s = format!("{}{}{}", x, "-", col);
                if counter.contains(&s) {
                    counter.remove(&s);
                } else {
                    counter.insert(s);
                }
            }
        }

        return counter.len() as i32;
    }
}
