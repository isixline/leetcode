pub fn min_cost_to_move_chips(chips: Vec<i32>) -> i32 {
    let mut odd = 0;
    let mut even = 0;

    for v in chips.iter() {
        if v % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }

    std::cmp::min(odd, even)
}
