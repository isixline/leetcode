pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut even = vec![];
        let mut odd = vec![];

        for x in a {
            if x % 2 == 0 {
                even.push(x);
            } else {
                odd.push(x);
            }
        }
        even.append(&mut odd);
        let all = even;
        all
}
