
pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut h1 = std::collections::HashMap::new();
    let mut h2 = std::collections::HashSet::new();
    let mut rest_arr: Vec<i32> = vec![];
    let mut result = vec![];

    for v in arr2.clone() {
        h2.insert(v);
    }

    for v in arr1 {
        if h2.contains(&v) {
            h1.entry(v).and_modify(|e| {
                *e += 1
            }).or_insert(1);
        } else {
            rest_arr.push(v);
        }
    }

    rest_arr.sort();

    for v in arr2 {
        let count = h1.get(&v).unwrap();
        for _i in 0..*count {
            result.push(v);
        }
    }

    result.append(&mut rest_arr);
    result
}
