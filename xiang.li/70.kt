class Solution {
    fun climbStairs(n: Int): Int {
        if(n==1)
            return 1
        var stairs = IntArray(n)
        stairs[0] = 1
        stairs[1] = 2
        for (i in 2 until n) {
            stairs[i] = stairs[i -1] + stairs[i - 2]
        }
        return stairs[n - 1]
    }
}
