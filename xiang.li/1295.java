class Solution {
    public int findNumbers(int[] nums) {
        int c = 0;
        for (int n : nums) {
            c += String.valueOf(n).length() % 2 == 0 ? 1 : 0;
        }
        return c;
    }
}
