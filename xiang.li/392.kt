class Solution {
    fun isSubsequence(s: String, t: String): Boolean {
        val sLen = s.length
        val tLen = t.length
        if (sLen > tLen) return false
        if (sLen == 0) return true
        
        val dp = Array(sLen + 1) { BooleanArray(tLen + 1) }
        for (j in 0 until tLen) {
            dp[0][j] = true
        }
        
        for (i in 1..sLen) {
            for (j in 1..tLen) {
                if (s[i - 1] == t[j - 1]) {
                    dp[i][j] = dp[i - 1][j - 1]
                } else {
                    dp[i][j] = dp[i][j - 1]
                }
            }
        }
        return dp[sLen][tLen]
    }
}
