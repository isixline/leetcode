class Solution {
    fun isToeplitzMatrix(matrix: Array<IntArray>): Boolean {
        for (i in 0 until matrix.size - 1) {
            var m = i
            var n = 0
            val temp: Int = matrix[m][n]
            while (m < matrix.size && n < matrix[0].size) {
                if (matrix[m][n] != temp) {
                    return false
                }
                m++
                n++
            }
        }
        for (i in 1 until matrix[0].size) {
            var m = 0
            var n = i
            val temp: Int = matrix[m][n]
            while (m < matrix.size && n < matrix[0].size) {
                if (matrix[m][n] != temp) {
                    return false
                }
                m++
                n++
            }
        }
        return true;
    }
}
