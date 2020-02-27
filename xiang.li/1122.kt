class Solution {
    fun relativeSortArray(arr1: IntArray, arr2: IntArray): IntArray {
        return arr1.sortedBy {
            val index = arr2.indexOf(it)
            if (index != -1) {
                0 - arr2.size + index
            } else {
                it
            }
        }.toIntArray()
    }
}
