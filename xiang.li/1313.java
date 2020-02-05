class Solution {
    public int[] decompressRLElist(int[] nums) {
        int length = 0;
        for(int i = 0;i < nums.length;i += 2){
            length += nums[i];
        }

        int[] result = new int[length];
        int k = 0;

        for(int i = 0;i < nums.length;i += 2){
            while(nums[i] > 0) {
                result[k] = nums[i + 1];
                nums[i]--;
                k++;
            }
        }
        
        return result;
    }
}
