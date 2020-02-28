pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted = arr.clone();
    sorted.sort();
    
    let mut min_difference = (sorted[1] - sorted[0]).abs();

    let mut result = vec![vec![sorted[0], sorted[1]]];

    for i in 1..sorted.len() - 1 {
        let m = (sorted[i + 1] - sorted[i]).abs();

        if m == min_difference {
            result.push(vec![sorted[i], sorted[i + 1]]);
            continue;
        }
        if m < min_difference {
            result.clear();
            result.push(vec![sorted[i], sorted[i + 1]]);
            min_difference = m;
        }
    }

    result
}
