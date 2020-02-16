class Solution {
    fun heightChecker(heights: IntArray): Int {
        var re = heights.sortedBy { it }
        return heights.filterIndexed { index, i -> re[index] != i }.size 
    }
}
