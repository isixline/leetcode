class Solution {
    fun kWeakestRows(mat: Array<IntArray>, k: Int): IntArray {
        return mat.mapIndexed{index, it ->  Pair(it.sum(), index)}
            .sortedBy { it.first }
            .chunked(k)
            .get(0)
            .map { it.second }
            .toIntArray()
    }
}
