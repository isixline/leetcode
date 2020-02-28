import java.util.*

class Solution {
    fun queensAttacktheKing(queens: Array<IntArray>, king: IntArray): List<List<Int>> {
        val chessboard = Array(8) { CharArray(8) }
        for (queen in queens) {
            chessboard[queen[0]][queen[1]] = 'q'
        }
        val directions = arrayOf(intArrayOf(0, 1), intArrayOf(0, -1), intArrayOf(1, 0), intArrayOf(-1, 0), intArrayOf(1, 1), intArrayOf(-1, -1), intArrayOf(1, -1), intArrayOf(-1, 1))
        val res: MutableList<List<Int>> = LinkedList()
        for (direction in directions) {
            dfs(chessboard, king, res, direction)
        }
        return res
    }

    private fun dfs(chessboard: Array<CharArray>, cur: IntArray, container: MutableList<List<Int>>, direction: IntArray) {
        if (!verify(cur[0], cur[1])) {
            return
        }
        if (chessboard[cur[0]][cur[1]] == 'q') {
            val temp: MutableList<Int> = LinkedList()
            temp.add(cur[0])
            temp.add(cur[1])
            container.add(temp)
            return
        }
        dfs(chessboard, intArrayOf(cur[0] + direction[0], cur[1] + direction[1]), container, direction)
    }

    private fun verify(i: Int, j: Int): Boolean {
        return i in 0..7 && j >= 0 && j < 8
    }
}


