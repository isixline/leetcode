pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut dp = vec![];
    let mut max = nums[0];

    dp.push(nums[0]);

    for i in 1..nums.len() {
        let r = if dp[i - 1] < 0 {
            nums[i]
        } else {
            dp[i - 1] + nums[i]
        };
        dp.push(r);
        max = std::cmp::max(max, r);
    }

    max
}
