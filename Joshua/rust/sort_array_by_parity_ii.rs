pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut sorted_arr = a.clone();
    
        let mut begin = 0;
        let mut end= 0;

        while begin < a.len() {
            if (begin % 2) as i32 == sorted_arr[begin] % 2 {
                begin += 1;
            } else {
                end = begin + 1;
                let mut next_uncorrect = 0;
                loop {
                    if (end % 2) as i32 == sorted_arr[end] % 2  {
                        end += 1;
                        continue;
                    } else {
                       if (begin % 2) as i32 == sorted_arr[end] % 2 {
                           println!("changed");
                            let tmp = sorted_arr[begin];
                            sorted_arr[begin] = sorted_arr[end];
                            sorted_arr[end] = tmp;
                            if next_uncorrect != 0 {
                                begin = next_uncorrect;
                            } else {
                                begin += 1;
                            }
                            break;
                       }
                       if (end % 2) as i32 != sorted_arr[end] % 2 {
                           if next_uncorrect != 0 {
                               next_uncorrect = end;
                           }
                           end += 1;
                       }
                    }
                }
            }
        }

        sorted_arr
    }
