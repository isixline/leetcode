class Solution {
    fun commonChars(A: Array<String>): List<String> {
        var a = A.map { i -> i.toCharArray() }
        return A.maxBy { it.length }!!.toCharArray().filter {
            a.all { i ->
                if (i.contains(it)) {
                    a[a.indexOf(i)][i.indexOf(it)] = '0'
                    true
                } else {
                    false
                }
            }
        }.map { it.toString() }.toList()
    }
}
