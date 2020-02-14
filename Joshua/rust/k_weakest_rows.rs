pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        struct Row {
        index: usize,
        soldiers: u32,
    }

    impl PartialEq for Row {
        fn eq(&self, other: &Self) -> bool {
            if self.soldiers == other.soldiers {
                return self.index > other.index;
            }
            return self.soldiers > other.soldiers;
        }
    }

    impl Eq for Row {}

    impl Ord for Row {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            if self.soldiers == other.soldiers {
                return self.index.cmp(&other.index);
            }
            return self.soldiers.cmp(&other.soldiers);
        }
    }

    impl PartialOrd for Row {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }    

    let mut v = vec![];

    for (x, e) in mat.iter().enumerate() {
        let mut soldiers = 0;
        for i in 0..e.len() {
            if e[i] == 1 {
                soldiers += 1;
            } else {
                break;
            }
        }
        let r = Row {
            index: x,
            soldiers: soldiers,
        };
        v.push(r);
    }
    v.sort();

    let mut weakest = vec![];
    for (i, x) in v.iter().enumerate() {
        if (i as i32) < k {
            weakest.push(x.index as i32);
        } else {
            break;
        }
    }
    weakest
    }
