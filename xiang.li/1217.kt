class Solution {
    fun minCostToMoveChips(chips: IntArray): Int {
        if (chips.size <= 1) {
            return 0
        }
        var cnt1 = 0
        var cnt2 = 0
        for (chip in chips) {
            if (chip % 2 == 0) {
                cnt2++
            } else {
                cnt1++
            }
        }
        return if (cnt1 > cnt2) cnt2 else cnt1
    }
}
