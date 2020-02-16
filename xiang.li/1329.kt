class Solution {
    fun diagonalSort(mat: Array<IntArray>): Array<IntArray> {
        mat.mapIndexed { x, it -> it.mapIndexed { y, i -> Pair(x - y, i) } }
                .flatten()
                .groupBy { it.first }
                .mapValues { it.value.sortedBy { i -> i.second } }
                .mapValues { it.value.map { i -> i.second } }
                .forEach { it.value.forEachIndexed { index, i -> if (it.key > 0) mat[index + it.key][index] = i else mat[index][index - it.key] = i } }
        return mat
    }
}
