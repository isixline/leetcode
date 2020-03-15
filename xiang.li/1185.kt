class Solution {
    fun dayOfTheWeek(day: Int, month: Int, year: Int): String {
        //蔡勒公式
        val week = arrayOf("Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday")
        val w = if (month <= 2) {
            val y = (year - 1) % 100
            val c = ((year - 1) - y) / 100
            ((y + y / 4 + c / 4 - 2 * c + 26 * (month + 12 + 1) / 10 + day - 1) % 7 + 7) % 7
        } else {
            val y = year % 100
            val c = (year - y) / 100
            ((y + y / 4 + c / 4 - 2 * c + 26 * (month + 1) / 10 + day - 1) % 7 + 7) % 7
        }
        return week[w]
    }

