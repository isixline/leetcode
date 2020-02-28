class Solution {
    fun numRookCaptures(board: Array<CharArray>): Int {
        var cnt = 0
        for (i in 0..7) {
            for (j in 0..7) {
                if (board[i][j] == 'R') {
                    for (k in i + 1..7) {
                        if (board[k][j] == 'B') {
                            break
                        } else if (board[k][j] == 'p') {
                            cnt++
                            break
                        }
                    }
                    for (k in i - 1 downTo 0) {
                        if (board[k][j] == 'B') {
                            break
                        } else if (board[k][j] == 'p') {
                            cnt++
                            break
                        }
                    }
                    for (k in j + 1..7) {
                        if (board[i][k] == 'B') {
                            break
                        } else if (board[i][k] == 'p') {
                            cnt++
                            break
                        }
                    }
                    for (k in j - 1 downTo 0) {
                        if (board[i][k] == 'B') {
                            break
                        } else if (board[i][k] == 'p') {
                            cnt++
                            break
                        }
                    }
                }
            }
        }
        return cnt
    }
}
