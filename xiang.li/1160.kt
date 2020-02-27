class Solution {
    fun countCharacters(words: Array<String>, chars: String): Int {
        return words.map { it.toCharArray() }.filter {
            var chs = chars.toCharArray()
            it.all { i ->
                if (chs.contains(i)) {
                    chs[chs.indexOf(i)] = '0'
                    true
                } else {
                    false
                }
            }
        }.sumBy { it.size }
    }
}
