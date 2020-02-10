pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut r = vec![];
        let mut max_with_right = 0;
        for i in 0..arr.len() {
            if arr[i] >= max_with_right {
                max_with_right = 0;
                for y in i + 1..arr.len() {
                    if arr[y] > max_with_right {
                        max_with_right = arr[y];
                    }
                }
            }
            r.push(max_with_right);
        }
        r[arr.len() - 1] = -1;
        r
}
