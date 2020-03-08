pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; (n + 1) as usize];

        for i in 0..=n as usize {
            if i <= 2 {
                dp[i] = i as i32;
            } else {
                dp[i] = dp[i - 1] + dp[i - 2];
            }
        }

        dp[n as usize]
    }
