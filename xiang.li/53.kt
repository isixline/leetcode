import kotlin.math.max
class Solution {
    fun maxSubArray(nums: IntArray): Int {
        for(i in 1 until nums.size) {
            nums[i] = max(nums[i], nums[i] + nums[i - 1])
            if(nums[i] > nums[0]) {
                nums[0] = nums[i]
            }
        }
        return nums[0]
    }
}
