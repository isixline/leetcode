import java.util.*

class Solution {
    fun deckRevealedIncreasing(deck: IntArray): IntArray {
        deck.sort()
        val queue: Queue<Int> = LinkedList()
        if (deck != null) {
            for (i in deck.indices.reversed()) { 
                queue.add(deck[i]) 
                if (i == 0) {
                    break
                }
                queue.add(queue.poll()) 
            }
        }
        if (deck != null) {
            for (i in deck.indices.reversed()) {
                deck[i] = queue.poll()
            }
        }
        return deck
    }
}
