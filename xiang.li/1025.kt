class Solution {
    fun divisorGame(N: Int): Boolean {
        if (N == 1) return false;
        if (N == 2) return true;

        var dp = BooleanArray(N + 1);
        dp[1] = false;
        dp[2] = true;

        for (i in 3..N) {
            dp[i] = false;
            for (j in 1 until i) {
                if (i % j == 0 && !dp[i - j]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        return dp[N];
    }
}
