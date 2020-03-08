pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut dp = vec![0; prices.len()];

    let mut diff_pair = (0, 0);
    for i in 0..prices.len() {
        if i == 0 {
            diff_pair.0 = prices[0];
            diff_pair.1 = prices[0];
            dp[0] = 0;
        } else {
            if prices[i] < diff_pair.0 {
                diff_pair.0 = prices[i];
                diff_pair.1 = prices[i];
                dp[i] = dp[i - 1];
            } else if prices[i] > diff_pair.1 {
                diff_pair.1 = prices[i];
                dp[i] = std::cmp::max(prices[i] - diff_pair.0, dp[i - 1]);
            } else {
                dp[i] = dp[i - 1];
            }
        }
    }

    dp.pop().unwrap()
}
