class Solution {
    fun sortArrayByParityII(A: IntArray): IntArray {
        A.groupBy { it % 2 }.forEach { it.value.forEachIndexed { index, i -> A[2 * index + it.key] = i }}
        return A
    }
}
