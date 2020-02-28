import kotlin.math.abs

class Solution {
    fun minimumAbsDifference(arr: IntArray): List<List<Int>> {
        val len = arr.size
        arr.sort()
        val c = arr.mapIndexed { index, i -> Pair(i, arr[(index + 1)  % len])}.sortedBy { abs(it.first - it.second) }
        return c.filter { abs(it.first - it.second) == abs(c[0].first - c[0].second) }.map { it.toList() }
    }
}
