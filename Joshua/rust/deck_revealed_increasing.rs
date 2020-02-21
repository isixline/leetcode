pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
    let mut sorted = deck.clone();
    sorted.sort();

    let mut result = vec![-1; deck.len()];
    let mut idx = 0;

    for v in sorted {
        if result[idx] == -1 {
            result[idx] = v;
        } else {
            let mut find_first = false;
            loop {
                if idx == result.len() -1 {
                    idx = 0;
                } else {
                    idx += 1;
                }
                if result[idx] == -1 {
                    if find_first {
                        result[idx] = v;
                        break;
                    } else {
                        find_first = true;
                    }
                }
            }
        }
    }

    result
}
