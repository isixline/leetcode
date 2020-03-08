pub fn divisor_game(n: i32) -> bool {
        let mut dp = vec![false; (n+1) as usize];
        
        for i in 0..= n {
            if i == 0 || i == 1 || i == 3 {
                dp[i as usize] = false;
            } else if i == 2 {
                dp[i as usize] = true
            } else {  
                for j in 1..i {
                    if i % j == 0 {
                        if !dp[(i - j) as usize] {
                            dp[i as usize] = true;
                            break;
                        }
                    }
                }
            }
        }
        dp[n as usize]
    }
