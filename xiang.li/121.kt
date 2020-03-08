import kotlin.math.max
class Solution {
    fun maxProfit(prices: IntArray): Int {
        if(prices.size == 0){
            return 0
        }
        var dp = IntArray(prices.size)
        dp[0] = 0
        for(i in 1 until prices.size) {
            dp[i] = max(dp[i], dp[i - 1] + prices[i] - prices[i - 1])
            if (dp[i] > dp[0]) {
                dp[0] = dp[i]
            }
        }
        return dp[0]
    }
}
