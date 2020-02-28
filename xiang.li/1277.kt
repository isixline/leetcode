class Solution {
    fun countSquares(matrix: Array<IntArray>): Int {
        if (matrix == null || matrix.isEmpty()) {
            return 0
        }
        val m = matrix.size
        val n: Int = matrix[0].size
        val dp = Array(m + 1) { IntArray(n + 1) }
        var res = 0
        for (i in 1..m) {
            for (j in 1..n) {
                if (matrix[i - 1][j - 1] == 1) {
                    dp[i][j] = 1 + Math.min(dp[i - 1][j], Math.min(dp[i - 1][j - 1], dp[i][j - 1]))
                }
                res += dp[i][j]
            }
        }
        return res
    }
}
