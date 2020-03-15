class Solution {
    fun matrixBlockSum(mat: Array<IntArray>, K: Int): Array<IntArray> {
        val row = mat.size
        val col: Int = mat[0].size
        val res = Array(row) { IntArray(col) }
        val dp = Array(row + 1) { IntArray(col + 1) }
        for (i in 0 until row) {
            for (j in 0 until col) {
                dp[i + 1][j + 1] = dp[i][j + 1] + dp[i + 1][j] - dp[i][j] + mat[i][j]
            }
        }
        for (i in 0 until row) {
            for (j in 0 until col) {
                val r1 = (i - K).coerceAtLeast(0)
                val c1 = (j - K).coerceAtLeast(0)
                val r2 = (i + K).coerceAtMost(row - 1)
                val c2 = (j + K).coerceAtMost(col - 1)
                res[i][j] = dp[r2 + 1][c2 + 1] - dp[r1][c2 + 1] - dp[r2 + 1][c1] + dp[r1][c1]
            }
        }
        return res
    }
}
