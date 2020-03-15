class Solution {
    fun countBits(num: Int): IntArray {
        val ans = IntArray(num + 1)
        var i = 0
        var b = 1
        while (b <= num) {
            while (i < b && i + b <= num) {
                ans[i + b] = ans[i] + 1
                ++i
            }
            i = 0
            b = b shl 1
        }
        return ans
    }

