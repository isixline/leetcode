class Solution {
    public int[] sumZero(int n) {
        int[] a = new int[n];
        int i;
        for(i = 0;i < n / 2;i++) {
            a[i] = i + 1;
        }
        if (n % 2 != 0) {
            a[i] = 0;
            i++;
        }
        for(int j = 0;i + j < n;j++) {
            a[i + j] = 0 - j - 1;
        }
        return a;
    }
}
