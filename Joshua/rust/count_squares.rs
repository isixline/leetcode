pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    let mut dp = vec![vec![0;matrix[0].len() + 1]; matrix.len() + 1];

    for (i, m) in matrix.iter().enumerate() {
        for (j, v) in m.iter().enumerate() {
            if *v == 1 {
                dp[i + 1][j + 1] = std::cmp::min(std::cmp::min(dp[i][j + 1], dp[i + 1][j]), dp[i][j]) + 1
            }
            result += dp[i + 1][j + 1];
        }
    }
    
    result
}
